<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import {page, modes, views} from './page.js'
    import {settings} from './settings.js'
    import {accounts} from './accounts'
    import {config, dateFormat} from './config.js'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'

    export let loadTransactions
    export let curEntry
    export let onClose

    let curTransaction

    const zeros = '00000000-0000-0000-0000-000000000000'
    let msg = ""
    let errors = new Errors()
    let format = dateFormat($config)
    let addButtonLabel = $_('buttons.add')
    let drTotal = 0
    let crTotal = 0
    let simpleAllowed = false
    let compoundMode = false
    let recorded = false
    let entries = []

    onMount(() => {
        console.log($page.mode, curEntry)
        if ($page.mode === modes.EDIT) {
            console.log(curEntry)
            fetchTransaction(curEntry.transaction_id)
        } else {
            curTransaction = {id: zeros, status:"Recorded"}
            let date = new Date()
            entries = [
                {realDate: new Date(date), description: "", amount: 0, drAmount: '', crAmount: '', entry_type: "Debit", account: {}},
                {realDate: new Date(date), description: "", amount: 0, drAmount: '', crAmount: '', entry_type: "Credit", account: {}},
            ]
            addButtonLabel = "Add"
            simpleAllowed = true
        }

    })

    const handleAddClick = () => {
        entries = [...entries, {id: zeros, transaction_id: curTransaction.id, date: new Date(), description: "", amount: 0, drAmount: '', crAmount: '', account: {}, entry_type: "Debit"}]
    }

    const handleRemoveClick = () => {
        if (entries.length > 2) {
            entries = [...entries.slice(0, entries.length - 1)]
        }
    }

    const close = () => {
        onClose()
    }

    const validateEntry = (entry, index, errors) => {
        const prefix = compoundMode ? index + "_" : ""
        if (!entry.description || entry.description.length < 1) {
             errors.addError(prefix + "description", $_('transaction.errors.descriptionRequired'))
        }

        if (!entry.realDate || entry.realDate.length < 1) {
            errors.addError(prefix + "date", $_('transaction.errors.dateRequired'))
        }

        if (!compoundMode) {
            if (!entry.amount || entry.amount.length < 1 || isNaN(entry.amount)) {
                errors.addError("amount", $_('transaction.errors.amountRequired'))
            }
        } else if (entry.entry_type === "Credit") {
            if (!entry.crAmount || entry.crAmount.length < 1 || isNaN(entry.crAmount)) {
                errors.addError(prefix + "crAmount", $_('transaction.errors.amountRequired'))
            }
        } else {
            if (!entry.drAmount || entry.drAmount.length < 1 || isNaN(entry.drAmount)) {
                errors.addError(prefix + "drAmount", $_('transaction.errors.amountRequired'))
                console.log(entry.drAmount)
            }
        }

        if (!entry.account || !entry.account.id || entry.account.id < 1) {
            if (settings.require_double_entry || compoundMode) {
                errors.addError(index + "_account", $_('transaction.errors.accountRequired'))
            }
        }

        if (compoundMode && (drTotal != crTotal)) {
            errors.addError("totals", $_('transaction.errors.totalsBalance'))
        }

        return errors
    }

    const toDateStr = (date) => {
        return date.getFullYear()+ "-" + (date.getMonth()+1) + "-" + date.getDate()
    }

    const onSave = () => {
        msg = ""
        errors = new Errors()
        if (!compoundMode) syncSecondEntry(entries)
        entries.forEach((e, i) => validateEntry(e, i, errors))

        if (!errors.hasErrors()) {
            const transaction = {
                    date: toDateStr(new Date()),
                    description: "",
                    entries: [...entries]
            }

            if (!compoundMode && !settings.require_double_entry) {
                 transaction.entries = transaction.entries.filter(e => (e["account"] && e["account"]["id"]))
            }

            transaction.entries.forEach (
                e => {
                    e["date"] = toDateStr(e.realDate)
                    e["account_id"] = e["account"]["id"]
                    if (compoundMode) {
                        e["amount"] = (e["entry_type"] === "Credit") ? e["crAmount"] : e["drAmount"]
                    }
                }
            )

            if (!transaction["status"] || (transaction["status"] != "Recorded")) {
                transaction["status"] = recorded?"Recorded":"Projected"
            }

            if ($page.mode === modes.NEW) {
                transaction["id"] = zeros
                transaction.entries.forEach (
                    e => {
                        e["id"] = zeros
                        e["transaction_id"] = zeros
                    }
                )
                addTransaction(transaction)
            } else if ($page.mode === modes.EDIT) {
                transaction["id"] = curTransaction.id
                saveTransaction(transaction)
            }
        }

    }

    const matchAccount = (account_id) =>  {
        let match = $accounts.filter(a => a.id == account_id)
        return match.length > 0 ? match[0] : null
    }

    function resolved(result) {
      msg = $_('transaction.errors.saved')
      curTransaction = result
      if ($page.mode === modes.EDIT) {
        loadTransactions()
        close()
      }
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

    const canBeSimple = () => {
        return (
            entries.length == 1 ||
            (entries.length == 2 &&
            entries[0].description === entries[1].description &&
            entries[0].amount === entries[1].amount &&
            entries[0].realDate && entries[0].realDate.getTime() == entries[1].realDate.getTime())
        )
    }

    function fetched(result) {
        curTransaction = result
        addButtonLabel = $_('buttons.update')
        console.log(result)
        entries = curTransaction.entries

        entries.forEach(e => {
            e.entry_type === "Credit" ? e.crAmount = e.amount : e.drAmount = e.amount
            e.realDate = new Date(e.date)
            e.account = matchAccount(e.account_id)
        })

        if (entries.length == 1) {
            entries.push({})
            syncSecondEntry()
            entries[1].account = null
            entries[0].entry_type === "Credit" ? entries[1].drAmount = entries[1].amount : entries[1].crAmount = entries[1].amount
        }

        simpleAllowed = canBeSimple(entries)
        if (!simpleAllowed) {
            compoundMode = true
        }

        recorded = curTransaction.status != "Projected"

        console.log(entries)
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }
    const addTransaction = async (transaction) => {
        console.log(transaction)
        await invoke('add_transaction', {transaction: transaction}).then(resolved, rejected)
        loadTransactions()
    }

    const saveTransaction = async (transaction) => {
        console.log(transaction)
        await invoke('update_transaction', {transaction: transaction}).then(resolved, rejected)
        loadTransactions()
    }

    function resolvedDelete(result) {
      msg = "Transaction deleted."
      close()
    }

    const deleteTransaction = async (transaction) => {
        console.log(transaction)
        if (transaction && transaction.id) {
               await invoke('delete_transaction', {id: transaction.id}).then(resolvedDelete, rejected)
        } else {
            close()
        }
    }


    const fetchTransaction = async (transactionId) => {
        console.log(transactionId)
        await invoke('transaction', {transactionId: transactionId}).then(fetched, rejected)
    }

    const showAmount = (entry, type) => {
        if (entry["drAmount"] > 0) {
            entry["entry_type"] = "Debit"
            return type === "Debit"
        }

        if (entry["crAmount"] > 0) {
            entry["entry_type"] = "Credit"
            return type === "Credit"
        }

        return true
    }

    const total = (type) => {
        let total = 0
        entries.filter(e => e.entry_type === type).forEach(e => total += Number(e[type === "Credit" ? "crAmount" : "drAmount"]))
        return total
    }

    const calculateTotals = () => {
        drTotal = total("Debit")
        crTotal = total("Credit")
    }


    const afterToggle = () => {
        if (compoundMode) syncSecondEntry()
    }

    $: {
        calculateTotals()
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
    <div class="form-heading">{$page.mode === modes.EDIT ? $_('transaction.edit') : $_('transaction.new')}</div>
    {#if curTransaction && curTransaction.entries}
    <div class="toolbar">
        <div class="toolbar-icon" on:click="{schedule(curTransaction)}" title={$_('transaction.schedule')}><Icon icon="mdi:clipboard-text-clock"  width="24"/></div>
        <div class="toolbar-icon" on:click="{deleteTransaction(curTransaction)}" title={$_('transaction.delete')}><Icon icon="mdi:trash-can-outline"  width="24"/></div>
    </div>
    {/if}
        {#if entries.length > 0 && !compoundMode}
        <div class="entries">
            <table>
                <tbody>
                <tr><td><div class="heading">{$_('labels.date')}</div></td><td><div class="heading">{$_('labels.description')}</div></td><td><div class="heading">{$_('labels.amount')}</div></td><td></td><td></td></tr>
                <tr>
                    <td><div class="date-input" class:error={errors.isInError("date")} ><DateInput bind:value={entries[0].realDate} {format} placeholder="" /></div></td>
                    <td class="description"><input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={entries[0].description}></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("amount")} bind:value={entries[0].amount}></td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="form-row2">
            {#if entries.length > 1}
            {#if entries[0].entry_type !== "Credit"}
            <Select bind:item={entries[0].account} items={$accounts} label={$_('labels.debit')} none={true} flat={true} />
            <Select bind:item={entries[1].account} items={$accounts} label={$_('labels.credit')} none={true} flat={true} />
            {/if}
            {#if entries[0].entry_type === "Credit"}
            <Select bind:item={entries[1].account} items={$accounts} label={$_('labels.debit')} none={true} flat={true} />
            <Select bind:item={entries[0].account} items={$accounts} label={$_('labels.credit')} none={true} flat={true} />
            {/if}
            {/if}
        </div>
        {/if}
        {#if compoundMode}
        <div class="entries">
            <table>
                <tbody>
                <tr><td><div class="heading">{$_('labels.date')}</div></td><td><div class="heading">{$_('labels.description')}</div></td><td><div class="heading">{$_('labels.amount')}</div></td><td><div class="heading">{$_('labels.debit')}</div></td><td><div class="heading">{$_('labels.credit')}</div></td></tr>
                {#each entries as e, i}
                <tr>
                    <td><div class="date-input" class:error={errors.isInError(i + "_date")} ><DateInput bind:value={e["realDate"]} {format} placeholder="" /></div></td>
                    <td class="description"><input id="desc" class="description-input-2" class:error={errors.isInError(i + "_description")} bind:value={e.description}></td>
                    <td><div class="select-adjust"><Select bind:item={e["account"]} items={$accounts} label="" none={false} flat={true} inError={errors.isInError(i + "_account")}/></div></td>
                    <td class="money">
                        {#if showAmount(e, "Debit")}<input id="amount" class="money-input" class:error={errors.isInError(i + "_drAmount")} bind:value={e.drAmount}>{/if}
                        {#if !showAmount(e, "Debit")}<input id="amount" class="money-input disabled" disabled="disabled">{/if}
                    </td>
                    <td class="money">
                        {#if showAmount(e, "Credit")}<input id="amount" class="money-input" class:error={errors.isInError(i + "_crAmount")} bind:value={e.crAmount}>{/if}
                        {#if !showAmount(e, "Credit")}<input id="amount" class="money-input disabled" disabled="disabled">{/if}
                    </td>
                </tr>
                {/each}
                <tr>
                    <td><div class="toolbar bottom-toolbar">
                        <div class="toolbar-icon" on:click="{handleAddClick}" title={$_('buttons.addRow')}><Icon icon="mdi:table-row-plus-after"  width="24"/></div>
                        <div class="toolbar-icon" class:greyed={entries.length <= 2} on:click="{handleRemoveClick}" title={$_('buttons.removeRow')}><Icon icon="mdi:table-row-remove"  width="24"/></div>
                    </div></td>
                    <td></td>
                    <td><div class="total">{$_('labels.totals')}</div></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={drTotal} disabled="disabled"></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={crTotal} disabled="disabled"></td></tr>
                </tbody>
            </table>
        </div>
        {/if}
    <div class="form-button-row">
        <div class="widget2 buttons-left">
            <input id="compound" type=checkbox bind:checked={compoundMode} on:change={afterToggle} disabled={!(compoundMode && canBeSimple(entries) || !compoundMode && simpleAllowed)}>
            <label for="compound">{$_('transaction.compound')}</label>
        </div>
        <div class="widget2 buttons-left">
            <input id="compound" type=checkbox bind:checked={recorded}>
            <label for="compound">{$_('transaction.recorded')}</label>
        </div>
        <div class="widget buttons">
            <button on:click={close}>{$_('buttons.close')}</button>
            <button on:click={onSave}>{addButtonLabel}</button>
        </div>
        <div class="widget errors">
            {#each errors.getErrorMessages() as e}
            <div class="error-msg">{e}</div>
            {/each}
            {#if msg}
            <div class="success-msg">{msg}</div>
            {/if}
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

    .errors {
        float: right;
        margin: 10px 12px 0 0;
    }
    .error-msg {
        color: #FBC969;
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
        max-width: 350px;
    }

    .success-msg {
        color: green;
        text-align: left;
    }

    .disabled {
        background-color: #F0F0F0;
    }

    .greyed {
        color: #666;
        border-color: #666;
    }

    .greyed:hover {
        color: #666 !important;
        border-color: #666 !important;
        cursor: default !important;
    }

    .error {
        border: 1px solid #FBC969 !important;
    }

    :global(.error-input input) {
        border: 1px solid #FBC969 !important;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .buttons button {
        min-width: 80px;
    }

    .buttons-left {
        float: left;
        margin: 10px 12px 0 0;
        padding: 5px 0px 5px 0px;
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

    .widget2 {
        padding: 5px 0px 5px 10px;
        margin: 13px 12px 0px 0px;
    }

    .widget2 label {
        display: inline-block;
        margin-left: 3px;
    }

    .widget2 input {
        margin: 0px;
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

    .select-adjust {
        margin-bottom: -8px;
    }

    .entries {
        padding: 5px 5px 10px 10px;
    }

    .toolbar {
        color: #9b9b9b;
        display: flex;
        -webkit-user-select: none; /* Chrome/Safari */
        -moz-user-select: none; /* Firefox */
        -ms-user-select: none; /* IE10+ */

        /* The rule below is not implemented in browsers yet */
        -o-user-select: none;

        /* The rule below is implemented in most browsers by now */
        user-select: none;
    }

    .bottom-toolbar {
        float: left;
    }

    .bottom-toolbar div {
        margin: 6px 0 0 0 !important;
    }

</style>