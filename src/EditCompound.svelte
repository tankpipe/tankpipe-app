<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import { invoke } from '@tauri-apps/api/core'

    export let close
    export let curTransaction
    export let accounts = []
    export let editMode = "ADD"

    let mode = "NONE"
    let msg = ""
    let errors = new Errors()
    let format="yyyy-MM-dd"
    let addButtonLabel = "Add"
    let modeButtonLabel = ""
    let drTotal = 0
    let crTotal = 0
    let simpleAllowed = false
    console.log(accounts)

    let entries = []

    onMount(() => {
        console.log(editMode, curTransaction)
        if (editMode == "EDIT") {
            console.log(curTransaction)
            fetchTransaction(curTransaction.transaction_id)

        } else {
            let date = new Date()
            entries = [
                {realDate: new Date(date), description: "", amount: 0, account: {}},
                {realDate: new Date(date), description: "", amount: 0, account: {}},
            ]
            addButtonLabel = "Add"
        }

    })

    const handleAddClick = () => {
        entries = [...entries, {date: new Date(), description: "", amount: 0, drAmount: '', crAmount: '', account: {}, entry_type: "Debit", status:"Recorded"}]
    }

    const handleRemoveClick = () => {
        if (entries.length > 2) {
            entries = [...entries.slice(0, entries.length - 1)]
        }
    }

    const onCancel = () => {
        close()
    }

    const validateEntry = (entry, index, errors) => {
        console.log(entry)
        if (!entry.description || entry.description.length < 1) {
             errors.addError(index + "_description", "Description is required")
        }

        if (!entry.realDate || entry.realDate.length < 1) {
            errors.addError(index + "_date", "Date is required")
        }

        if (entry.entry_type === "Credit") {
            if (!entry.crAmount || entry.crAmount.length < 1 || isNaN(entry.crAmount)) {
                errors.addError(index + "_crAmount", "A valid amount is required")
            }
        } else {
            if (!entry.drAmount || entry.drAmount.length < 1 || isNaN(entry.drAmount)) {
                errors.addError(index + "_drAmount", "A valid amount is required")
                console.log(entry.drAmount)
            }
        }

        if (!entry.account || !entry.account.id || entry.account.id < 1) {
            errors.addError(index + "_account", "Account is required")
        }

        if (drTotal != crTotal) {
            errors.addError("totals", "Totals should balance")
        }

        return errors
    }

    const toDateStr = (date) => {
        return date.getFullYear()+ "-" + (date.getMonth()+1) + "-" + date.getDate()
    }

    const onAdd = () => {
        msg = ""
        errors = new Errors()
        if (mode === "SIMPLE") syncSecondEntry(entries)
        entries.forEach((e, i) => validateEntry(e, i, errors))

        if (!errors.hasErrors()) {
            const transaction = {
                    date: toDateStr(new Date()),
                    description: "",
                    entries: [...entries]
            }

            transaction.entries.forEach (
                e => {
                    e["date"] = toDateStr(e.realDate)
                    e["account_id"] = e["account"]["id"]
                    e["amount"] = (e["entry_type"] === "Credit") ? e["crAmount"] : e["drAmount"]
                }
            )

            if (editMode == "ADD") {
                const zeros = '00000000-0000-0000-0000-000000000000'
                transaction["id"] = zeros
                transaction.entries.forEach (
                    e => {
                        e["id"] = zeros
                        e["transaction_id"] = zeros
                    }
                )
                addTransaction(transaction)
            } else if (editMode == "EDIT") {
                transaction["id"] = curTransaction.id
                saveTransaction(transaction)
            }
        }

    }

    const matchAccount = (account_id) =>  {
        let match = accounts.filter(a => a.id == account_id)
        return match.length > 0 ? match[0] : null
    }

    function resolved(result) {
      msg = "Transaction saved."
    }

    const syncSecondEntry = (entries) => {
        entries[1].realDate = new Date(entries[0].realDate)
        entries[1].description = entries[0].description
        entries[1].amount = entries[0].amount
    }

    const canBeSimple = (entries) => {
        return (
            entries.length == 1 ||
            (entries.length == 2 &&
            entries[0].description === entries[1].description &&
            entries[0].amount === entries[1].amount &&
            entries[0].realDate.getTime() == entries[1].realDate.getTime())
        )
    }

    function fetched(result) {
        curTransaction = result
        addButtonLabel = "Update"
        console.log(accounts)
        entries = curTransaction.entries
        entries.forEach(e => {
            e.entry_type === "Credit" ? e.crAmount = e.amount : e.drAmount = e.amount
            e.realDate = new Date(e.date)
            e.account = matchAccount(e.account_id)
        })
        simpleAllowed = canBeSimple(entries)
        mode = simpleAllowed ? "SIMPLE" : "COMPOUND"
        modeButtonLabel = simpleAllowed ?  "Compound" : "Simple"
        console.log(entries)
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }
    const addTransaction = async (transaction) => {
        console.log(transaction)
           await invoke('add_transaction', {transaction: transaction}).then(resolved, rejected)
    }

    const saveTransaction = async (transaction) => {
        console.log(transaction)
           await invoke('update_transaction', {transaction: transaction}).then(resolved, rejected)
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


    const toggleMode = () => {
        const isSimple = mode === "SIMPLE"
        if (isSimple) syncSecondEntry(entries)
        mode = isSimple ? "COMPOUND" : "SIMPLE"
        modeButtonLabel = isSimple ? "Simple" : "Compound"
    }

    $: {
        calculateTotals(entries)
    }


</script>
<div class="form">
    <div class="panel">
        {#if entries.length > 0 && mode === "SIMPLE"}
        <div class="form-row">
            <div class="widget date-time-field" class:error-input={errors.isInError("date")}>
                <label for="date">Date</label>
                <DateInput bind:value={entries[0].realDate} {format} placeholder="" />
            </div>
            <div class="widget">
                <label for="desc">Description</label>
                <input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={entries[0].description}>
            </div>
            <div class="widget">
                <label for="amount">Amount</label>
                <input id="amount" class="money-input" class:error={errors.isInError("amount")} bind:value={entries[0].amount}>
            </div>
        </div>
        <div class="form-row2">
            {#if entries[0].entry_type === "Debit"}
            <Select bind:item={entries[0].account} items={accounts} label="Debit" none={true}/>
            <Select bind:item={entries[1].account} items={accounts} label="Credit" none={true} />
            {/if}
            {#if entries[0].entry_type === "Credit"}
            <Select bind:item={entries[1].account} items={accounts} label="Debit" none={true}/>
            <Select bind:item={entries[0].account} items={accounts} label="Credit" none={true} />
            {/if}
        </div>
        {/if}
        {#if mode === "COMPOUND"}
        <div class="entries">
            <table>
                <tr><td><div class="heading">Date</div></td><td><div class="heading">Account</div></td><td><div class="heading">Description</div></td><td><div class="heading">Debit</div></td><td><div class="heading">Credit</div></td></tr>
                {#each entries as e, i}
                <tr>
                    <td><div class="date-input" class:error={errors.isInError(i + "_date")} ><DateInput bind:value={e["realDate"]} {format} placeholder="" /></div></td>
                    <td><div class="select-adjust"><Select bind:item={e["account"]} items={accounts} label="" none={false} flat={true} inError={errors.isInError(i + "_account")}/></div></td>
                    <td class="description"><input id="desc" class="description-input-2" class:error={errors.isInError(i + "_description")} bind:value={e.description}></td>
                    <td class="money">
                        {#if showAmount(e, "Debit")}<input id="amount" class="money-input" class:error={errors.isInError(i + "_drAmount")} bind:value={e.drAmount}>{/if}
                        {#if !showAmount(e, "Debit")}<input id="amount" class="money-input disabled" disabled="disabled">{/if}
                    </td>
                    <td class="money">
                        {#if showAmount(e, "Credit")}<input id="amount" class="money-input" class:error={errors.isInError(i + "_drAmount")} bind:value={e.crAmount}>{/if}
                        {#if !showAmount(e, "Credit")}<input id="amount" class="money-input disabled" disabled="disabled">{/if}
                    </td>
                </tr>
                {/each}
                <tr>
                    <td><div class="toolbar"><i class="gg-add-r" on:click={handleAddClick}></i><i class="gg-remove-r" on:click={handleRemoveClick} class:greyed={entries.length <= 2}></i></div></td>
                    <td></td>
                    <td><div class="total">Totals</div></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={drTotal} disabled="disabled"></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={crTotal} disabled="disabled"></td></tr>
            </table>
        </div>
        {/if}
    </div>
    <div class="form-button-row">
        <div class="widget errors">
            {#each errors.getErrorMessages() as e}
            <div class="error-msg">{e}</div>
            {/each}
            {#if msg}
            <div class="success-msg">{msg}</div>
            {/if}
        </div>
        {#if mode === "COMPOUND" && canBeSimple(entries) || mode === "SIMPLE" && simpleAllowed}
        <div class="buttons-left">
            <button on:click={toggleMode}>{modeButtonLabel}</button>
        </div>
        {/if}
        <div class="widget buttons">
            <button on:click={onCancel}>Close</button>
            <button on:click={onAdd}>{addButtonLabel}</button>
        </div>
    </div>
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

    .form-row {
        display: inline-flex;
        float: left;
        width: 100%;
        clear:both;
    }

    .form-row2, .form-button-row {
        display: block;
        text-align: left;
    }

    .errors {
        float: left;
    }
    .error-msg {
        color: red;
        text-align: left;
        margin-bottom: 3px;
    }

    .success-msg {
        color: green;
        text-align: left;
    }

    .disabled {
        background-color: #F0F0F0;
    }

    .greyed {
        color: #CCC;
        border-color: #CCC;
    }

    .greyed:hover {
        color: #CCC !important;
        border-color: #CCC !important;
        cursor: default !important;
    }

    .error {
        border: 1px solid red !important;
    }

    :global(.error-input input) {
        border: 1px solid red !important;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .buttons button, .buttons-left button {
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
        background-color: #F0f0f0;
        margin-top: 20px;
        border-radius: 10px;
    }

    .form label, .heading, .total {
        text-align: left;
        font-size: .8em;
        color: #333;
        margin-bottom: 3px;
    }

    .total {
        text-align: right;
    }

    .panel {
        background-color: #E0E0E0;
        margin: 15px 15px 0 15px;
        border-radius: 10px;
        padding-right: 10px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .widget p {
        max-width: 500px;
        font-size: 0.9em;
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
        color: #7b7b7b;
        display: flex;
        -webkit-user-select: none; /* Chrome/Safari */
        -moz-user-select: none; /* Firefox */
        -ms-user-select: none; /* IE10+ */

        /* The rule below is not implemented in browsers yet */
        -o-user-select: none;

        /* The rule below is implemented in most browsers by now */
        user-select: none;
    }

    .toolbar i:hover{
        color: #666;
        border-color: #666;
        cursor: pointer;
    }

    .toolbar i {
        margin-right: 5px;
    }

    .gg-add-r {
        box-sizing: border-box;
        position: relative;
        display: block;
        width: 22px;
        height: 22px;
        border: 2px solid currentColor;
        transform: scale(var(--ggs,1));
        border-radius: 4px
    }

    .gg-add-r::after,
    .gg-add-r::before {
        content: "";
        display: block;
        box-sizing: border-box;
        position: absolute;
        width: 10px;
        height: 2px;
        background: currentColor;
        border-radius: 5px;
        top: 8px;
        left: 4px
    }

    .gg-add-r::after {
        width: 2px;
        height: 10px;
        top: 4px;
        left: 8px
    }

    .gg-remove-r {
        box-sizing: border-box;
        position: relative;
        display: block;
        transform: scale(var(--ggs,1));
        width: 22px;
        height: 22px;
        border: 2px solid ;
        border-radius: 4px
    }

    .gg-remove-r::before {
        content: "";
        display: block;
        box-sizing: border-box;
        position: absolute;
        width: 10px;
        height: 2px;
        background: currentColor;
        border-radius: 5px;
        top: 8px;
        left: 4px
    }



</style>