<script module>
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import {page, modes} from './page.js'
    import {accounts} from './accounts.js'
    import {config, formatAmount, formatDate, dateFormat} from './config.js'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import {DateInput} from 'date-picker-svelte'

    const ACCOUNT_TYPES = [{value:"Asset", name:"Asset"}, {value:"Liability", name:"Liability"}, {value:"Revenue", name:"Revenue"}, {value:"Expense", name:"Expense"}, {value:"Equity", name:"Equity"}]
</script>

<script>
    let { close, curAccount, loadAccounts, initialize = false } = $props()

    let msg = $state("")
    let errors = $state(new Errors())
    let interestMsg = $state("")
    let interestErrors = $state(new Errors())
    let name = $state()
    let startingBalance = $state()
    let accountType = $state()
    let addButtonLabel = $state($_('buttons.add'))
    let transactions = $state([])
    let rollbackDate = $state()
    let interestInfo = $state()
    let curInterestTerms = $state()
    let format = dateFormat($config)
    const zeros = '00000000-0000-0000-0000-000000000000'

    onMount(() => {
        if ($page.mode === modes.EDIT) {
            name = curAccount.name
            startingBalance = curAccount.starting_balance
            accountType = matchAccountType(curAccount.account_type)
            addButtonLabel = $_('buttons.update')
            console.log(curAccount)
            if (curAccount && curAccount.interest_info_id) {
                loadInterestInfo()
            }
        } else {
            addButtonLabel = $_('buttons.add')
            startingBalance = "0"
            curAccount = null
        }
    })

    $effect(() => {
        if (curAccount && curAccount.id) loadTransactions()
    })

    const loadTransactions = async () => {
        console.log("loadTransactions: " + curAccount.id)
        transactions = await invoke('transactions', {accountId: curAccount.id})
        console.log(transactions)
    }

    const loadInterestInfo = async () => {
        console.log("loadInterestInfo: " + curAccount.id)
        if (curAccount.interest_info_id) {
            await invoke('get_interest_info', {interestInfoId: curAccount.interest_info_id}).then(
                (result) => {
                    console.log(result)
                    interestInfo = result
                    
                    if (interestInfo.terms && interestInfo.terms.length > 0) {
                        interestInfo.terms.forEach((term) => {
                            if (term.end_date === "null") {
                                term.end_date = null
                            }
                            term.realStartDate = new Date(term.start_date)
                            term.realEndDate = term.end_date ? new Date(term.end_date) : null
                        })
                        curInterestTerms = null
                    }
                },
                (error) => {
                    console.error("Error loading interest info:", error)
                    interestErrors.addError("interestInfo", $_('errors.genericError', { values: { 0: error } }))
                }
            )
        } else {
            interestInfo = null
            // Initialize empty interest info for new accounts
            // interestInfo = {
            //     paid_to_date: null,
            //     terms: [{
            //         start_date: null,
            //         realStartDate: null,
            //         end_date: null,
            //         realEndDate: null,
            //         rate: 0.05,
            //         calculated: null,
            //         paid_period: null,
            //         paid_frequency: 1,
            //         paid_day: 1,
            //         description: "",
            //         interest_account_id: null
            //     }],
            //     account_id: curAccount.id
            // }
        }
    }

    const matchAccountType = (value) =>  {
        if (!value) return null
        let match = ACCOUNT_TYPES.filter(p => p.value == value)
        return match.length > 0 ? match[0] : null
    }

    const onCancel = () => {
        close()
    }

    const onAdd = async () => {
        msg = ""
        errors = new Errors()
        interestErrors = new Errors()
        interestMsg = ""
        if (!name || name.length < 1) {
            errors.addError("name", $_('account.form.errors.nameRequired'))
        }

        if (!startingBalance || startingBalance.length < 1 || isNaN(startingBalance)) {
            errors.addError("startingBalance", $_('account.form.errors.startingBalanceRequired'))
        }

        if (!accountType || !accountType.value) {
            errors.addError("accountType", $_('account.form.errors.accountTypeRequired'))
        }

        if (!errors.hasErrors()) {
            if ($page.mode === modes.NEW) {
                const account = {
                    name: name,
                    starting_balance: startingBalance,
                    account_type: accountType.value
                }
                addAccount(account)
            } else if ($page.mode === modes.EDIT) {
                const account = {
                    name: name,
                    starting_balance: startingBalance,
                    account_type: curAccount.account_type,
                    id: curAccount.id,
                    balance: 0
                }
                saveAccount(account)
            }
        }
    }

    const saveInterestInfo = async () => {
        msg = ""
        errors = new Errors()
        interestErrors = new Errors()
        interestMsg = ""

        interestInfo.terms.forEach((terms, i) => {
            if (!terms.realStartDate) {
                interestErrors.addError(i + "_startDate", $_('interest.errors.startDateRequired'))
            }

            if (!terms.rate || terms.rate.length < 1 || isNaN(terms.rate)) {
                interestErrors.addError(i + "_rate", $_('interest.errors.rateRequired'))
            }
        })

        console.log(interestErrors)

        
        if (!interestErrors.hasErrors()) {
            //let dateStr = interestInfo.paid_to_date.getFullYear()+ "-" + (interestInfo.paid_to_date.getMonth() + 1) + "-" + interestInfo.paid_to_date.getDate()
            const interestData = {
                id: interestInfo.id,
                paid_to_date: interestInfo.paid_to_date ? interestInfo.paid_to_date : "null",
                account_id: curAccount.id,
                terms: []
            }

            interestInfo.terms.forEach((terms) => {
                let startDate = terms.realStartDate ? terms.realStartDate : new Date(terms.start_date)
                let startDateStr = startDate.getFullYear()+ "-" + (startDate.getMonth() + 1) + "-" + startDate.getDate()
                let endDate = terms.realEndDate ? terms.realEndDate : null
                let endDateStr = endDate ? endDate.getFullYear()+ "-" + (endDate.getMonth() + 1) + "-" + endDate.getDate() : "null"

                interestData.terms.push({
                    id: terms.id,
                    start_date: startDateStr,
                    end_date: endDateStr,
                    rate: terms.rate,
                    calculated: terms.calculated,
                    paid_period: terms.paid_period,
                    paid_frequency: terms.paid_frequency,
                    paid_day: terms.paid_day,
                    description: terms.description,
                    interest_account_id: terms.interest_account_id || null
                })
            })
            
            console.log("Saving interest info:", interestData)
            if (interestInfo && interestInfo.id) {
                await invoke('update_interest_info', {interestInfo: interestData}).then(interestResolved, interestRejected)
            } else {
                interestData.id = zeros
                await invoke('add_interest_info', {interestInfo: interestData}).then(interestAddResolved, interestRejected)
            }
        }
    }

    function resolved(result) {
        msg = $_('account.form.success.saved')
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", $_('errors.genericError', { values: { 0: result } }))
    }

    async function interestResolved(result) {
        interestMsg = $_('interest.saved')
        await loadInterestInfo()
    }

    async function interestAddResolved(result) {
        interestMsg = $_('interest.saved')
        await loadAccounts()
        await loadInterestInfo()
    }

    function interestRejected(result) {
        interestErrors = new Errors()
        interestErrors.addError("all", $_('errors.genericError', { values: { 0: result } }))
    }

    const addAccount = async (account) => {
        await invoke('add_account', {account: account}).then(resolved, rejected)
        loadAccounts()
        initialize = false
    }

    const saveAccount = async (account) => {
        console.log(account)
        await invoke('update_account', {account: account}).then(resolved, rejected)
        loadAccounts()
        close()
    }

    function deleteResolved(result) {
        msg = $_('account.form.success.deleted')
        loadAccounts()
        close()
    }

    const deleteAccount = async (account) => {
        console.log(account)
        if (account) {
            await invoke('delete_account', {account: account}).then(deleteResolved, rejected)
        } else {
            close()
        }
    }

    const rollbackReconciliation = async () => {
        if (!rollbackDate) {
            errors = new Errors()
            errors.addError("rollback", $_('account.form.errors.rollbackDateRequired'))
            return
        }

        // Convert Date to ISO string for backend
        const rollbackDateString = rollbackDate.toISOString().split('T')[0]
        
        await invoke('rollback_reconciliation', { 
            accountId: curAccount.id, 
            toDate: {date: rollbackDateString} 
        }).then(() => {
            msg = $_('account.form.success.rollback')            
        }, rejected)
        loadAccounts()
    }

    const selectTerms = (term) => {
        curInterestTerms = term
    }

    const EMPTY_TERMS = {
        id: zeros,
        start_date: null,
        realStartDate: null,
        end_date: null,
        realEndDate: null,
        rate: "0.0",
        calculated: "Daily",
        paid_period: "Months",
        paid_frequency: 1,
        paid_day: 1,
        description: "",
        interest_account_id: null
    }

    const addTerms = () => {
        if (!interestInfo) { 
            interestInfo = {
                ...interestInfo,
                terms: []
            }
        }
        
        if(interestInfo.terms && interestInfo.terms.length > 0) {
            const lastTerm = interestInfo.terms[interestInfo.terms.length - 1]
            curInterestTerms = {
                ...lastTerm,
                id: zeros
            }
        } else {
            curInterestTerms = Object.assign({}, EMPTY_TERMS)
        }
        
        interestInfo.terms.push(curInterestTerms)                       
    }

    const deleteLastTerms = () => {        
        if (interestInfo && interestInfo.terms && interestInfo.terms.length > 0) {
            interestInfo.terms.pop()
            curInterestTerms = null
        }
    }

    const closeCurrentTerms = () => {
        curInterestTerms = null
    }
    
    const addInterestInfo = () => {
       interestInfo = {
                paid_to_date: "null",
                terms: [Object.assign({}, EMPTY_TERMS)],
                account_id: curAccount.id
            }
        curInterestTerms = interestInfo.terms[0]
    }

</script>

{#if $accounts.length < 1 && $config.recent_files.length < 2}
<div class="message">{$_('account.firstAccountMessage')}</div>
{/if}

<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT ? $_('account.form.title.edit') : $_('account.form.title.new')}</div>
    <div class="toolbar toolbar-right">
        {#if transactions.length < 1}
        <button class="toolbar-icon" onclick={deleteAccount(curAccount)} title={$_('account.form.deleteTooltip')}><Icon icon="mdi:trash-can-outline"  width="24"/></button>
        {/if}
    </div>
    <div class="form-row">
        <div class="widget">
            <label for="name">{$_('labels.name')}</label>
            <input id="name" class="description-input" class:error={errors.isInError("name")} bind:value={name}>
        </div>
        <div class="widget">
            <label for="startingBalance">{$_('account.form.labels.startingBalance')}</label>
            <input id="startingBalance" class="money-input" class:error={errors.isInError("startingBalance")} bind:value={startingBalance}>
        </div>
    </div>
    <div class="form-row">
        <Select bind:item={accountType} items={ACCOUNT_TYPES} label={$_('account.form.labels.type')} none={false} inError={errors.isInError("accountType")} disabled={$page.mode === modes.EDIT} flat={true}/>
    </div>
    <div class="form-button-row">
        <div class="msg-panel">
            {#each errors.getErrorMessages() as e}
            <p class="error-msg selectable-text">{e}</p>
            {/each}
            {#if msg}
            <p class="success-msg selectable-text">{msg}</p>
            {/if}
        </div>
        <div class="widget buttons">
            <button class="og-button" onclick={onCancel}>{$_('buttons.close')}</button>
            <button class="og-button" onclick={onAdd}>{addButtonLabel}</button>
        </div>
    </div>    
    <hr/>
    {#if curAccount && curAccount.reconciliation_info}               
        <div class="info-title">{$_('account.reconciliationInfo')}</div>
        <div class="info-row">
                <div class="info-label">{$_('account.reconciledTo')}&nbsp;</div>
                <div class="info-value">{formatDate(curAccount.reconciliation_info.date)}</div>              
        </div>
        <div class="info-row">
                <div class="info-label">{$_('account.reconciledBalance')}&nbsp;</div>
                <div class="info-value">{formatAmount(curAccount.reconciliation_info.balance)}</div>              
        </div>
        <div class="info-row">&nbsp;</div>
        <div class="rollback-row">
            <div class="widget">
                <label for="rollbackDate">{$_('account.rollbackTo')}</label>
                <div class="date-input" id="rollbackDate">
                    <DateInput bind:value={rollbackDate} placeholder="" closeOnSelection={true}/>
                </div>
            </div>
            <div class="widget">
                <button class="og-button" onclick={rollbackReconciliation} disabled={!rollbackDate}>{$_('account.rollbackButton')}</button>
            </div>
        </div>
        <div class="info-row">&nbsp;</div>
        <hr/>
    {/if}
    
        <div class="info-title">{$_('interest.title')}</div>
        {#if !interestInfo}
            <div class="toolbar " >
                <button class="toolbar-icon" onclick="{addInterestInfo}" title={$_('interest.addTerms')}>
                    <Icon icon="mdi:plus"  width="18"/>
                </button>
            </div>
        {/if}
        {#if interestInfo}
        <div>
            <table class="csv-table" style="max-width: 450px;">
                <tbody>
                    
                    <tr>
                        <th>{$_('interest.startDate')}</th>
                        <th>{$_('interest.endDate')}</th>
                        <th>{$_('interest.rate')}</th>
                    </tr>                    
                    <tr class="spacer"></tr>
                {#each interestInfo.terms as t, i}
                    <tr class="csv-row {curInterestTerms === t ? 'selected-row' : ''}" onclick={() => selectTerms(t)}>                        
                        <td><div class:error={interestErrors.isInError(i + "_startDate")}>{formatDate(t["realStartDate"])}</div></td>
                        <td><div class:error={interestErrors.isInError(i + "_endDate")}>{formatDate(t["realEndDate"])}</div></td>
                        <td><div class:error={interestErrors.isInError(i + "_rate")}>{t.rate ? (t.rate * 100).toFixed(2) + '%' : ''}</div></td>
                    </tr>
                {/each}               
                    <tr>
                        <th colspan="3">
                            <div class="toolbar terms-toolbar" >
                                <button class="toolbar-icon" onclick="{addTerms}" title={$_('interest.addTerms')}>
                                    <Icon icon="mdi:plus"  width="18"/>
                                </button>
                                <button class="toolbar-icon" onclick="{deleteLastTerms}" title={$_('interest.removeTerms')} disabled={!interestInfo.terms || interestInfo.terms.length === 0}>
                                    <Icon icon="mdi:trash-can-outline"  width="17"/>
                                </button>
                            </div>
                        </th>
                    </tr>
                </tbody>
            </table>
        </div>
        {#if curInterestTerms}
        {@const index = interestInfo.terms.indexOf(curInterestTerms)}
        <div class="info-row">&nbsp;</div>
        <div class="interest-form">
            <div class="toolbar toolbar-right" style="padding: 0">
                <button class="toolbar-icon" onclick="{closeCurrentTerms}" title={$_('buttons.close')}><Icon icon="mdi:close-box-outline"  width="24"/></button>
            </div>
            <div class="form-row">        
                <div class="widget">
                    <label for="startDate">{$_('interest.startDate')}</label>
                    <div class="date-input" class:error={interestErrors.isInError(index + "_startDate")} ><DateInput bind:value={curInterestTerms["realStartDate"]} {format} placeholder="" disabled={false} closeOnSelection={true}/></div>
                </div>
                <div class="widget">
                    <label for="endDate">{$_('interest.endDate')}</label>
                    <div class="date-input" class:error={interestErrors.isInError(index + "_endDate")} ><DateInput bind:value={curInterestTerms["realEndDate"]} {format} placeholder="" disabled={false} closeOnSelection={true}/></div>
                </div>
                <div class="widget">
                    <label for="interestRate">{$_('interest.rate')}</label>
                    <input id="interestRate" class="money-input" class:error={interestErrors.isInError(index + "_rate")} bind:value={curInterestTerms.rate} placeholder=""/>
                </div>            
                <div class="widget">
                    <label for="calculatedType">{$_('interest.calculatedType')}</label>
                    <Select id="calculatedType" bind:item={curInterestTerms.calculated} items={[{id:"Daily", name:"Daily"}, {id:"Monthly", name:"Monthly"}]} none={false} valueField="id" inError={errors.isInError("calculatedType")} flat={true}/>
                </div>
            </div>
            <div class="form-row2">
                <div class="widget">
                    {$_('interest.paidEvery')}&nbsp;<input id="amount" class="number-input" type="number" class:error={errors.isInError("frequency")} bind:value={curInterestTerms.paid_frequency}>
                    &nbsp;<Select bind:item={curInterestTerms.paid_period} items={[{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, {value:"Months", name:"Months"}]} none={false} valueField="value" inError={errors.isInError("paidType")} flat={true}/>
                    {$_('interest.paidOn')}&nbsp;<input id="paidDay"  class="number-input" type="number" bind:value={curInterestTerms.paid_day} placeholder="1"/>
                </div>
            </div>
            <div class="form-row">
                <div class="widget">
                    <label for="description">{$_('interest.description')}</label>
                    <input id="description" bind:value={curInterestTerms.description} placeholder=""/>
                </div>
                <div class="widget">
                    <label for="interest_account_id">{$_('interest.interestAccount')}</label>
                    <Select id="interest_account_id" bind:item={curInterestTerms.interest_account_id} items={$accounts.map(a => ({value: a.id, name: a.name}))} none={true} inError={errors.isInError("interestAccountId")} flat={true}/>
                </div>
            </div>            
        </div>
        {/if}
        <div class="form-button-row">
            <div class="msg-panel sele">
                {#each interestErrors.getErrorMessages() as e}
                <p class="error-msg selectable-text">{e}</p>
                {/each}
                {#if interestMsg}
                <p class="success-msg selectable-text">{interestMsg}</p>
                {/if}
            </div>
            <div class="widget buttons">
                <button class="og-button" onclick={saveInterestInfo}>{$_('buttons.update')}</button>
            </div>
        </div>        
        <hr/>        
    {/if}
</div>

<style>

    :global(.date-time-field input) {
        border: 1px solid #CCC !important;
        border-radius: 2px !important;
        height: 33px;
    }

    :root {
        --date-input-width: 110px;
    }

    .msg-panel {
        padding-left: 2px;
        font-size: 0.9em;
        float:left;
    }

    :global(.message) {
        color: #EFEFEF;
        margin-bottom: 20px;
        text-align: left;
        background-color: #303030;
        padding:10px;
        border-radius: 10px;
    }

    .msg-panel p {
        margin: 8px 0;
        max-width: 500px;
    }

    .error-msg {
        color: #FBC969;
    }

    .success-msg {
        color: green;
    }

    .error {
        border: 1px solid #FBC969;
    }

    :global(.error-input input) {
        border: 1px solid #FBC969 !important;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .form-row {
        display: inline-flex;
        float: left;
        width: 100%;
        clear:both;
    }

    .form-button-row {
        display: block;
        text-align: left;
        margin-left: 7px;
        margin-right: 2px;
        clear: both;
    }

    input {
        margin-right: 0px;
    }

    .form {
        float: left;
        border-radius: 10px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .money-input {
        width: 110px;
    }

    .money-input {
        text-align: right;
    }

    .description-input {
        width: 400px;
    }

    .info-row {
        padding: 5px 0 0px 10px;
        margin: 0 0 0 0;
        display: inline-flex;
        float: left;
        width: 100%;
        clear: both;
    }

    .info-label {
        font-size: 0.75em;
        color: #aaa;
    }

    .info-value {
        font-size: 0.75em;
        color: #aaa;
    }
    
    .info-title {
        white-space: nowrap;
        font-weight: 200;
        font-size: 1em;
        color: #757575;
        margin-bottom: 10px;
        display: inline-flex;
        float: left;
        width: 100%;
        clear: both;
    }

    .rollback-row {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 0px 0 0px 10px;
        margin: 0 0 0 0;
        float: left;
        width: 100%;
        clear: both;
    }

    .rollback-row .widget {
        display: flex;
        align-items: center;
        padding: 5px 0px 5px 0px;
    }

    .rollback-row label {
        margin-right: 10px;
        white-space: nowrap;
    }

    .rollback-row button {
        min-height: 33px;
        margin-bottom: 0px;
    }

    hr {
        border-style: none;
        border: 1px solid #363636;
        margin-left: -20px;
        width: 100vw;
    }

    .csv-table td {
        max-width: 18vw;
        overflow: hidden;
        text-overflow: ellipsis;
        font-size: 0.9em;
    }

    .csv-row td {
        font-size: 0.8em;
    }

    .csv-table tr:hover td {
        cursor: pointer;
        color: #e0e0e0 !important;
    }

    .selected-row td {
        background-color: #333 !important;
    }
    
    table {
        padding-right: 10px;
        width: 100%;
    }

    td {
        text-align: left;
        overflow: hidden;
        line-height: 1em;
        color: #ccc;
        background-color: #393939;
        padding: 8px;
        white-space: nowrap;
        font-size: 0.9em;
    }

    th {
        color:#666666;
        background-color: #444;
        font-weight: 400;
        font-size: .8em;
        text-align: left;
    }

    .money-input {
        width: 100px;
    }

    .money-input {
        text-align: right;
    }

    .description-input {
        width: 400px;
    }

    .date-input {
        margin-top: 0px;
    }

    .form-row2 {
        display: block;
        text-align: left;
    }

    .number-input {
        width: 50px;
        text-align: right;
        background-color: #F0F0F0;
    }
     
    .interest-form {
        border-radius: 10px;
        color: #DDDDDD;
        background-color: #333;
        float: left;
        padding: 10px;
    }
    
    .terms-toolbar {
        margin: 3px 0 0 -5px;
    }

    .error {
        border: 1px solid #FBC969 !important;
    }
</style>