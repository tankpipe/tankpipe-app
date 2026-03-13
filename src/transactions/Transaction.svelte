<script>
    import {DateInput} from 'date-picker-svelte'
    import Select from '../components/Select.svelte'
    import Icon from '@iconify/svelte'
    import {page, modes, views} from '../stores/page.js'
    import {accounts} from '../stores/accounts'
    import {config, dateFormat} from '../stores/config.js'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'

    let { transactionId, onClose } = $props()

    let curTransaction = $state({})

    const zeros = '00000000-0000-0000-0000-000000000000'
    let format = dateFormat($config)
    let drTotal = $state(0)
    let crTotal = $state(0)
    let compoundMode =  $state(false)
    let recorded = $state(false)
    let entries =  $state([])

    $effect(() => {
        if ($page.mode === modes.VIEW) {
            fetchTransaction(transactionId)
        }
    })

    function fetched(result) {
        console.log("fetched ", result)
        Object.assign(curTransaction, result)
        entries.splice(0)
        entries.push(...curTransaction.entries)

        entries.forEach(e => {
            e.entry_type === "Credit" ? Object.assign(e, {crAmount: e.amount}) : Object.assign(e, {drAmount: e.amount})
            e.realDate = new Date(e.date)
            e.account = matchAccount(e.account_id)
        })

        if (entries.length == 1) {
            entries.push({})
            syncSecondEntry()
            entries[1].account = null
            entries[0].entry_type === "Credit" ? entries[1].drAmount = entries[1].amount : entries[1].crAmount = entries[1].amount
        }

        compoundMode = true
        recorded = curTransaction.status != "Projected"
        calculateTotals()
    }

    function rejected(result) {
        console.error("Failed to fetch transaction:", result)
    }

    const fetchTransaction = async (transactionId) => {
        await invoke('transaction', {transactionId: transactionId}).then(fetched, rejected)
    }

    const matchAccount = (account_id) =>  {
        let match = $accounts.filter(a => a.id == account_id)
        return match.length > 0 ? match[0] : null
    }

    const close = () => {
        onClose()
    }

    const total = (type) => {
        let total = 0
        const amountField = type === "Credit" ? "crAmount" : "drAmount"
        entries.filter(e => isValidAmount(e[amountField])).forEach(e => total += Number(e[amountField]))
        return total
    }

    const isValidAmount = (amount) => {
        return amount && amount.length > 0 && ! isNaN(amount) && Number(amount) > 0
    }

    const calculateTotals = () => {
        drTotal = total("Debit")
        crTotal = total("Credit")
    }

    const syncSecondEntry = () => {
        if (entries[1].id === undefined) entries[1].id = zeros
        entries[1].transaction_id = entries[0].transaction_id
        entries[1].entry_type =  entries[0].entry_type == "Credit" ? "Debit" : "Credit"
        entries[1].realDate = new Date(entries[0].realDate)
        entries[1].description = entries[0].description
        entries[1].amount = entries[0].amount
        entries[1].status = "Recorded"
    }

    const schedule = () => {
        console.log("schedule", curTransaction.entries[0].schedule_id)
        if (curTransaction.entries[0].schedule_id) {
            page.set({view: views.SCHEDULES, mode: modes.LOAD, payload:{schedule_id: curTransaction.entries[0].schedule_id}})
        } else {
            page.set({view: views.SCHEDULES, mode: modes.NEW, payload:{entries: [...entries]}})
        }
    }
</script>

<div class="form">
    <div class="form-heading">{$_('transaction.view')}</div>
    
    {#if curTransaction && curTransaction.entries}
    <div class="toolbar toolbar-right">
        <button class="toolbar-icon" onclick={schedule} title={$_('transaction.schedule')}><Icon icon="mdi:clipboard-text-clock"  width="24"/></button>
    </div>
    {/if}
    {#if (curTransaction.source_type)}
    <div class="indicator source-msg"><span>{$_('transaction.sourceType.' + curTransaction.source_type)}</span></div>
    {/if}
    <div class="indicator recon-msg"><span>{$_('transaction.reconciled')}</span><Icon icon="mdi:check" width="16"/></div>
    {#if compoundMode}
    <div class="entries">
        <table>
            <tbody>
            <tr><td><div class="heading">{$_('labels.date')}</div></td><td><div class="heading">{$_('labels.description')}</div></td><td><div class="heading">{$_('labels.amount')}</div></td><td><div class="heading">{$_('labels.debit')}</div></td><td><div class="heading">{$_('labels.credit')}</div></td></tr>
            {#each entries as e, i}
            <tr>
                <td><div class="date-input"><DateInput bind:value={e["realDate"]} {format} placeholder="" disabled/></div></td>
                <td class="description"><input id="desc" class="description-input-2" bind:value={e.description} disabled/></td>
                <td><div class="select-adjust"><Select bind:item={e["account"]} items={$accounts} label="" none={false} flat={true} disabled/></div></td>
                <td class="money">
                    <input id="dramount" class="money-input" bind:value={e.drAmount} disabled/>
                </td>
                <td class="money">
                    <input id="cramount" class="money-input" bind:value={e.crAmount} disabled/>
                </td>
                <td class="reconciled-cell">{#if e.reconciled_status == "Reconciled"}<Icon icon="mdi:check" width="16"/>{:else if e.reconciled_status == "Outstanding"}<Icon icon="mdi:circle-small" width="16"/>{/if}</td>
            </tr>
            {/each}
            <tr>
                <td><div class="toolbar bottom-toolbar"></div></td>
                <td></td>
                <td><div class="total">{$_('labels.totals')}</div></td>
                <td class="money"><input id="amount" class="money-input" bind:value={drTotal} disabled="disabled"></td>
                <td class="money"><input id="amount" class="money-input" bind:value={crTotal} disabled="disabled"></td></tr>
        </tbody>
    </table>
    </div>
    {/if}
    <div class="form-button-row">
        <div class="widget buttons">
            <button class="og-button" onclick={close}>{$_('buttons.close')}</button>
        </div>
    </div>
</div>

<style>

    :global(.date-time-field input) {
        border: 1px solid #CCC !important;
        border-radius: 2px !important;
        height: 33px;
        background-color: #aaa;
    }

    :root {
        --date-input-width: 110px;
    }

    .form-row2, .form-button-row {
        display: block;
        text-align: left;
    }

    .form-row2{
        min-height: 70px;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .buttons button {
        min-width: 80px;
    }

    

    .form-button-row {
        margin-left: 7px;
        margin-right: 2px;
    }

    input {
        margin-right: 0px;
    }

    .form {
        float: left;
        border-radius: 10px;
    }

    .total {
        text-align: right;
        margin: 0px 5px -3px 0px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
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

    table, td, tr {
        border: 0;
        border-collapse: collapse;
        -webkit-user-select: none; /* Chrome/Safari */
        -moz-user-select: none; /* Firefox */
        -ms-user-select: none; /* IE10+ */
        -o-user-select: none;
        user-select: none;
    }

    .entries input {
        margin: 0px;
    }

    

    .entries {
        padding: 5px 5px 10px 10px;
        clear: both;
    }

    

    

    .indicator span {
        padding-top: 6px;
        font-size: 0.75em;
    }

    .recon-msg {
        color: #74d965;
    }
    
        
    .reconciled-cell {
        background-color: #444 !important;
        color: #ccc;
        font-size: .8em;
        font-weight: bold;
        padding: 0 0 4px 3px;
        text-align: center;
    }

</style>
