<script module>
    import {Errors} from '../utils/errors.js'
    import Select from '../components/Select.svelte'
    import Icon from '@iconify/svelte'
    import MessagePanel from '../components/MessagePanel.svelte'
    import {accounts} from '../stores/accounts.js'
    import {config, formatDate, dateFormat} from '../stores/config.js'
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
    const PAID_PERIODS = [{value:"Days", name:$_('interest.paidPeriods.days')}, {value:"Weeks", name:$_('interest.paidPeriods.weeks')}, {value:"Months", name:$_('interest.paidPeriods.months')}]

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
                    interest.paid_to = interest.paid_to === "null" ? null : new Date(interest.paid_to)
                    if (interest.terms && interest.terms.length > 0) {
                        interest.terms.forEach((term) => {
                            if (term.end_date === "null") {
                                term.end_date = null
                            }
                            term.realStartDate = new Date(term.start_date)
                            term.realEndDate = term.end_date ? new Date(term.end_date) : null
                            term.rate = (term.rate * 100).toFixed(2)
                        })
                        curInterestTerms = null
                    }
                },
                (error) => {
                    console.error("Error loading interest:", error)
                    interestErrors.addError("interest", $_('errors.genericError', { values: { 0: error } }))
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
        })

        console.log(interestErrors)

        
        if (!interestErrors.hasErrors()) {
            const interestData = {
                id: interest.id,
                paid_to: interest.paid_to ? interest.paid_to : "null",
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
                    interest_account_id: terms.interest_account_id || null
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
        interestErrors.addError("all", $_('errors.genericError', { values: { 0: result } }))
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
 <div class="info-row">
    <div class="info-label">{$_('interest.paidTo')}&nbsp;</div>
    <div class="info-value">{interest.paid_to ? formatDate(interest.paid_to) : "-"}</div>
</div>
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
        <div class="widget">
            <label for="calculatedType">{$_('interest.calculatedType')}</label>
            <Select id="calculatedType" bind:item={curInterestTerms.calculated} items={CALCULATED_TYPES} none={false} valueField="id" inError={interestErrors.isInError("calculatedType")} flat={true} disabled={true}/>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            {$_('interest.paidEvery')}&nbsp;<input id="amount" class="number-input" type="number" class:error={interestErrors.isInError("frequency")} bind:value={curInterestTerms.paid_frequency}>
            &nbsp;<Select bind:item={curInterestTerms.paid_period} items={PAID_PERIODS} none={false} valueField="value" inError={interestErrors.isInError("paidType")} flat={true}/>
            {$_('interest.paidOn')}&nbsp;<input id="paidDay"  class="number-input" type="number" bind:value={curInterestTerms.paid_day} placeholder="1"/>
        </div>
    </div>
    <div class="form-row">
        <div class="widget">
            <label for="description">{$_('interest.description')}</label>
            <input id="description" bind:value={curInterestTerms.description} placeholder={$_('interest.descriptionDefault')}/>
        </div>
        <div class="widget">
            <label for="interest_account_id">{$_('interest.interestAccount')}</label>
            <Select id="interest_account_id" bind:item={curInterestTerms.interest_account_id} items={$accounts.map(a => ({id: a.id, name: a.name}))} valueField="id" none={true} inError={interestErrors.isInError("interestAccountId")} flat={true}/>
        </div>
    </div>            
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
        width: 50px;
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
