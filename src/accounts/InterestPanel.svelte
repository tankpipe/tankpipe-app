<script module>
    import {Errors} from '../errors.js'
    import Select from '../Select.svelte'
    import Icon from '@iconify/svelte'
    import {accounts} from '../accounts.js'
    import {config, formatDate, dateFormat} from '../config.js'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import {DateInput} from 'date-picker-svelte'
</script>

<script>
    let { curAccount, loadAccounts } = $props()

    let interestMsg = $state("")
    let interestErrors = $state(new Errors())
    let interestInfo = $state()
    let curInterestTerms = $state()
    let format = dateFormat($config)
    const zeros = '00000000-0000-0000-0000-000000000000'

    $effect(() => {
        if (curAccount && curAccount.id) {
            loadInterestInfoInternal()
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
        interest_account_id: null
    }

    const loadInterestInfoInternal = async () => {
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
        }
    }

    const saveInterestInfo = async () => {
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
            
            if (interestInfo && interestInfo.id) {
                await invoke('update_interest_info', {interestInfo: interestData}).then(interestResolved, interestRejected)
            } else {
                interestData.id = zeros
                await invoke('add_interest_info', {interestInfo: interestData}).then(interestAddResolved, interestRejected)
            }
        }
    }

    async function interestResolved(result) {
        interestMsg = $_('interest.saved')
        await loadInterestInfoInternal()
    }

    async function interestAddResolved(result) {
        interestMsg = $_('interest.saved')
        await loadAccounts()
        await loadInterestInfoInternal()
    }

    function interestRejected(result) {
        interestErrors = new Errors()
        interestErrors.addError("all", $_('errors.genericError', { values: { 0: result } }))
    }

    const selectTerms = (term) => {
        curInterestTerms = term
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

<div class="info-title">{$_('interest.title')}</div>
{#if !interestInfo}
    <div class="toolbar" >
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
            <Select id="calculatedType" bind:item={curInterestTerms.calculated} items={[{id:"Daily", name:"Daily"}, {id:"Monthly", name:"Monthly"}]} none={false} valueField="id" inError={interestErrors.isInError("calculatedType")} flat={true}/>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            {$_('interest.paidEvery')}&nbsp;<input id="amount" class="number-input" type="number" class:error={interestErrors.isInError("frequency")} bind:value={curInterestTerms.paid_frequency}>
            &nbsp;<Select bind:item={curInterestTerms.paid_period} items={[{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, {value:"Months", name:"Months"}]} none={false} valueField="value" inError={interestErrors.isInError("paidType")} flat={true}/>
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
            <Select id="interest_account_id" bind:item={curInterestTerms.interest_account_id} items={$accounts.map(a => ({value: a.id, name: a.name}))} none={true} inError={interestErrors.isInError("interestAccountId")} flat={true}/>
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
{/if}

<style>
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

    
    .date-input {
        margin-top: 0px;
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

    .msg-panel {
        padding-left: 2px;
        font-size: 0.9em;
        float:left;
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
        border: 1px solid #FBC969 !important;
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

    :global(.date-time-field input) {
        border: 1px solid #CCC !important;
        border-radius: 2px !important;
        height: 33px;
    }

    :root {
        --date-input-width: 110px;
    }
</style>
