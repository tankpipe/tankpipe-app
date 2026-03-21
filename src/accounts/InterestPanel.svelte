<script module>
    import {Errors} from '../utils/errors.js'
    import Select from '../components/Select.svelte'
    import Icon from '@iconify/svelte'
    import MessagePanel from '../components/MessagePanel.svelte'
    import {accounts, normalBalance} from '../stores/accounts.js'
    import {config, formatDate, dateFormat, formatAmount} from '../stores/config.js'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import {DateInput} from 'date-picker-svelte'
</script>

<script>
    let { curAccount, loadAccounts } = $props()

    let interestMsg = $state("")
    let interestErrors = $state(new Errors())
    let interest = $state()
    let curInterestTerms = $state()
    let format = dateFormat($config)
    const zeros = '00000000-0000-0000-0000-000000000000'
    const CALCULATED_TYPES = [{id:"Daily", name:$_('interest.calculatedTypes.daily')}]
    const PAID_PERIODS = [{value:"Months", name:$_('interest.paidPeriods.months')}]
    let curAccountNormalBalance = $derived.by(() => {
        return normalBalance(curAccount.account_type)
    })
    let showAdvanced = $state(false)

    $effect(() => {
        if (curAccount && curAccount.id) {
            loadInterestInternal()
        }
    })

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
        income_account_id: null,
        interest_account_id: null
    }

    const loadInterestInternal = async () => {
        if (!curAccount || !curAccount.id) return
        console.log("loadInterest: " + curAccount.interest_id)
        if (curAccount.interest_id) {
            await invoke('get_interest', {interestId: curAccount.interest_id}).then(
                (result) => {
                    console.log(result)
                    interest = result
                    if (interest.terms && interest.terms.length > 0) {
                        interest.terms.forEach((term) => {
                            if (term.end_date === "null") {
                                term.end_date = null
                            }
                            term.realStartDate = new Date(term.start_date)
                            term.realEndDate = term.end_date ? new Date(term.end_date) : null
                            term.rate = (term.rate * 100).toFixed(2)
                            term.interest_account_id = term.interest_account_id == null ? curAccount.id : term.interest_account_id
                            term.min_balance = term.min_balance ? Number(term.min_balance).toFixed(2) : "0.00"
                            term.max_balance = term.max_balance ? Number(term.max_balance).toFixed(2) : ""
                            if ((term.min_balance && term.min_balance > 0) || term.max_balance || term.interest_account_id !== curAccount.id) {
                                showAdvanced = true
                            }
                        })
                        curInterestTerms = null
                    }
                },
                (error) => {
                    console.error("Error loading interest:", error)
                    interestErrors.addError("interest", error)
                }
            )
        } else {
            interest = null
        }
    }

    const saveInterest = async () => {
        interestErrors = new Errors()
        interestMsg = ""

        interest.terms.forEach((terms, i) => {
            if (!terms.realStartDate) {
                interestErrors.addError(i + "_startDate", $_('interest.errors.startDateRequired'))
            }

            if (!terms.rate || terms.rate.length < 1 || isNaN(terms.rate)) {
                interestErrors.addError(i + "_rate", $_('interest.errors.rateRequired'))
            }

            if (!terms.paid_day || (terms.paid_day < 1 || terms.paid_day > 31)) {
                interestErrors.addError(i + "_paidDay", $_('interest.errors.paidDayRequired'))
            }

            if (!terms.description || terms.description.length < 1) {
                interestErrors.addError(i + "_description", $_('interest.errors.descriptionRequired'))
            }

        })

        console.log(interestErrors)

        
        if (!interestErrors.hasErrors()) {
            const interestData = {
                id: interest.id,                
                account_id: curAccount?.id,
                terms: []
            }

            interest.terms.forEach((terms) => {
                let startDate = terms.realStartDate ? terms.realStartDate : new Date(terms.start_date)
                let startDateStr = startDate.getFullYear()+ "-" + (startDate.getMonth() + 1) + "-" + startDate.getDate()
                let endDate = terms.realEndDate ? terms.realEndDate : null
                let endDateStr = endDate ? endDate.getFullYear()+ "-" + (endDate.getMonth() + 1) + "-" + endDate.getDate() : "null"

                interestData.terms.push({
                    id: terms.id,
                    start_date: startDateStr,
                    end_date: endDateStr,
                    rate: terms.rate / 100,
                    calculated: terms.calculated,
                    paid_period: terms.paid_period,
                    paid_frequency: terms.paid_frequency,
                    paid_day: terms.paid_day,
                    description: terms.description,
                    income_account_id: terms.income_account_id || null,
                    interest_account_id: terms.interest_account_id == curAccount.id ? null : terms.interest_account_id || null,
                    min_balance: Number(terms.min_balance),
                    max_balance: terms.max_balance && terms.max_balance.length > 0 ? Number(terms.max_balance) : null
                })
            })
            
            
            if (interest && interest.id) {
                console.log(interestData)
                await invoke('update_interest', {interest: interestData}).then(interestResolved, interestRejected)
            } else {
                interestData.id = zeros
                console.log(interestData)
                await invoke('add_interest', {interest: interestData}).then(interestAddResolved, interestRejected)
            }
        }
    }

    async function interestResolved(result) {
        interestMsg = $_('interest.saved')
        await loadInterestInternal()
    }

    async function interestAddResolved(result) {
        interestMsg = $_('interest.saved')
        console.log(curAccount.interest_id)
        await loadAccounts()
        console.log(curAccount.interest_id)
        await loadInterestInternal()
    }

    function interestRejected(result) {
        interestErrors = new Errors()
        interestErrors.addError("all", result)
    }

    const selectTerms = (term) => {
        curInterestTerms = term
    }

    const addTerms = () => {
        if (!interest) { 
            interest = {
                ...interest,
                terms: []
            }
        }
        
        if(interest.terms && interest.terms.length > 0) {
            const lastTerm = interest.terms[interest.terms.length - 1]
            curInterestTerms = {
                ...lastTerm,
                id: zeros
            }
        } else {
            curInterestTerms = Object.assign({}, EMPTY_TERMS)
        }
        
        interest.terms.push(curInterestTerms)                       
    }

    const deleteTerm = (index) => {
        if (interest && interest.terms && interest.terms.length > 0) {
            interest.terms.splice(index, 1)
            curInterestTerms = null
        }
    }

    const closeCurrentTerms = () => {
        curInterestTerms = null
    }
    
    const addInterest = () => {
       if (!curAccount || !curAccount.id) return
       interest = {
                paid_to_date: "null",
                terms: [Object.assign({}, EMPTY_TERMS)],
                account_id: curAccount.id
            }
        curInterestTerms = interest.terms[0]
    }

</script>

<div class="info-title">{$_('interest.title')}</div>
{#if !interest}
    <div class="toolbar" >
        <button class="toolbar-icon" onclick="{addInterest}" title={$_('interest.addTerms')}>
            <Icon icon="mdi:plus"  width="18"/>
        </button>
    </div>
{/if}
{#if interest}
<div>
    <table class="csv-table" style="max-width: 450px;">
        <tbody>
            <tr>
                <th colspan="2"></th>
                <th>
                    <div class="toolbar terms-toolbar" style="float: right; ">
                        <button class="toolbar-icon" onclick="{addTerms}" title={$_('interest.addTerms')}>
                            <Icon icon="mdi:plus"  width="18"/>
                        </button>
                    </div>
                </th>
            </tr>
            <tr>
                <th>{$_('interest.startDate')}</th>
                <th>{$_('interest.endDate')}</th>
                <th>{$_('interest.rate')}</th>                
            </tr>                    
            <tr class="spacer"></tr>
        {#each interest.terms as t, i}
            <tr class="csv-row {curInterestTerms === t ? 'selected-row' : ''}" onclick={() => selectTerms(t)}>                        
                <td><div class:error={interestErrors.isInError(i + "_startDate")}>{formatDate(t["realStartDate"])}</div></td>
                <td><div class:error={interestErrors.isInError(i + "_endDate")}>{formatDate(t["realEndDate"])}</div></td>
                <td><div class:error={interestErrors.isInError(i + "_rate")}>{t.rate ? t.rate + '%' : ''}</div></td>
            </tr>
        {/each}               
        </tbody>
    </table>
</div>
{#if curInterestTerms}
{@const index = interest.terms.indexOf(curInterestTerms)}
<div class="info-row">&nbsp;</div>
<div class="interest-form">
    <div class="toolbar toolbar-right" style="padding: 0">
        <button class="toolbar-icon" onclick={() => deleteTerm(index)} title={$_('interest.removeTerms')} disabled={!interest.terms || interest.terms.length === 0}>
            <Icon icon="mdi:trash-can-outline"  width="24"/>
        </button>
        <button class="toolbar-icon" onclick="{closeCurrentTerms}" title={$_('buttons.close')}>
            <Icon icon="mdi:close-box-outline"  width="24"/>
        </button>
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
    </div>
    <div class="form-row">
        <div class="widget">
            <label for="description">{$_('interest.description')}</label>
            <input id="description" bind:value={curInterestTerms.description} class:error={interestErrors.isInError(index + "_description")} />
        </div>
        <div class="widget">
            <label for="income_account_id">{$_('interest.' + curAccountNormalBalance + '.incomeAccount')}</label>
            <Select id="income_account_id" bind:item={curInterestTerms.income_account_id} items={$accounts.map(a => ({id: a.id, name: a.name}))} valueField="id" none={true} inError={interestErrors.isInError("incomeAccountId")} flat={true} />
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            {$_('interest.' + curAccountNormalBalance + '.paidEvery')}&nbsp;{$_('interest.' + curAccountNormalBalance + '.paidOn')}&nbsp;&nbsp;<input id="paidDay"  class="number-input" type="number" class:error={interestErrors.isInError(index + "_paidDay")}  bind:value={curInterestTerms.paid_day} placeholder="1" max="31"/>
        </div>
    </div>
    <hr/>    
    <div class="toolbar" style="margin: 0px" >
        <label for="advanced" class="toggle-label">{$_('interest.advanced')}</label>
        <button class="toolbar-icon" onclick={() => showAdvanced = !showAdvanced}>
            <Icon icon={showAdvanced ? "mdi:chevron-up" : "mdi:chevron-down"} width="24"/>
        </button>
    </div>
    {#if showAdvanced}
    <div id="advanced">
        <div class="form-row">
            <div class="widget">
                <label for="min">{$_('interest.minBalance')}</label>
                <input id="min" class="money-input" class:error={interestErrors.isInError(index + "_min")} bind:value={curInterestTerms.min_balance}>
            </div>
            <div class="widget">
                <label for="max">{$_('interest.maxBalance')}</label>
                <input id="max" class="money-input" class:error={interestErrors.isInError(index + "_max")} bind:value={curInterestTerms.max_balance}>
            </div>
        </div>            
        <div class="form-row">
            <div class="widget">
                <label for="interest_account_id">{$_('interest.' + curAccountNormalBalance + '.interestAccount')}</label>
                <Select id="interest_account_id" bind:item={curInterestTerms.interest_account_id} items={$accounts.map(a => ({id: a.id, name: a.name}))} valueField="id" none={true} inError={interestErrors.isInError("incomeAccountId")} flat={true} />
            </div>
        </div>                
    </div>
    {/if}
</div>
{/if}
<div class="form-button-row">
    <div class="msg-panel sele">
        <MessagePanel errors={interestErrors} msg={interestMsg} />
    </div>
    <div class="widget buttons">
        <button class="og-button" onclick={saveInterest}>{$_('buttons.update')}</button>
    </div>
</div>         
{/if}

<style>
    .info-title {
        white-space: nowrap;
        font-weight: 200;
        font-size: 1em;
        color: var(--color-text-subtle);
        margin-bottom: 10px;
        display: inline-flex;
        float: left;
        width: 100%;
        clear: both;
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
        color: var(--color-text-strong-2) !important;
    }

    .selected-row td {
        background-color: var(--color-surface) !important;
    }
    
    table {
        padding-right: 10px;
        width: 100%;
    }

    td {
        text-align: left;
        overflow: hidden;
        line-height: 1em;
        color: var(--color-table-cell-text);
        background-color: var(--color-table-cell-bg);
        padding: 8px;
        white-space: nowrap;
        font-size: 0.9em;
    }

    th {
        color:var(--color-border);
        background-color: var(--color-bg);
        font-weight: 400;
        font-size: .8em;
        text-align: left;
    }

    .money-input {
        width: 120px;    
        text-align: right;
    }
    
    .date-input {
        margin-top: 0px;
    }

    .form-row {
        display: inline-flex;
        width: 100%;
    }

    .form-button-row {
        display: block;
        text-align: left;
        margin-left: 7px;
        margin-right: 2px;
        clear: both;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .form-row2 {
        display: block;
        text-align: left;
    }

    .number-input {
        width: 55px;
        text-align: right;
        background-color: var(--color-text-strong);
    }
     
    .interest-form {
        border-radius: 10px;
        color: var(--color-text);
        background-color: var(--color-surface);
        float: left;
        padding: 10px;
    }
    
    .interest-form hr {
        border-style: none;
        border: 1px solid #444444;
        margin-left: -10px;
        margin-right: -10px;
        width: auto;
    }

    .toggle-label {
        color: var(--color-text-subtle);
        margin: 5px -5px 0 5px;
    }

    .terms-toolbar {
        margin: 3px 0 0 -5px;
    }

    :global(.date-time-field input) {
        border: 1px solid var(--color-border-light) !important;
        border-radius: 2px !important;
        height: 33px;
    }

    :root {
        --date-input-width: 110px;
    }
</style>
