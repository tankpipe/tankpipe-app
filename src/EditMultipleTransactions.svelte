<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import {page, modes, views} from './page.js'
    import {settings} from './settings.js'
    import {accounts} from './accounts.js'
    import {config, dateFormat} from './config.js'
    import { invoke } from "@tauri-apps/api/core"

    export let loadTransactions
    export let curEntry
    export let transactions

    const zeros = '00000000-0000-0000-0000-000000000000'
    let msg = ""
    let errors = new Errors()
    let format = dateFormat($config)
    let addButtonLabel = "Add"
    let drTotal = 0
    let crTotal = 0
    let compoundMode = false
    let recorded = false
    let entries = []
    let curTransaction

    onMount(() => {
        console.log($page.mode, curEntry, transactions)
        if ($page.mode === modes.MULTI_EDIT) {
            console.log(curEntry)
            fetchTransaction(curEntry.transaction_id)
        }

        resetChanges()
    })

    const resetChanges = () => {
        entries = [
            {realDate: null, description: "", amount: '', drAmount: '', crAmount: '', entry_type: "Debit", account: {}},
            {realDate: null, description: "", amount: '', drAmount: '', crAmount: '', entry_type: "Credit", account: {}},
        ]
    }

    const handleAddClick = () => {
        entries = [...entries, {id: zeros, transaction_id: curTransaction.id, date: new Date(), description: "", amount: 0, drAmount: '', crAmount: '', account: {}, entry_type: "Debit"}]
    }

    const handleRemoveClick = () => {
        if (entries.length > 2) {
            entries = [...entries.slice(0, entries.length - 1)]
        }
    }

    const close = () => {
        loadTransactions()
        page.set({view: views.TRANSACTIONS, mode: modes.LIST})
    }

    const applyChanges = (entry, changeEntry, index, errors) => {

        if (changeEntry.description && changeEntry.description.length > 0) {
            entry.description = changeEntry.description
        }

        if (changeEntry.account && changeEntry.account.id) {
            entry.account_id = changeEntry.account.id
        }

    }

    const onSave = () => {
        msg = ""
        errors = new Errors()

        const changedTransactions = []
        transactions.forEach(t => {
            const transaction = structuredClone(t)

            if (transaction.entries.length == 1 && (entries[1].account && entries[1].account.id)) {
                createSecondEntry(transaction)
            }

            transaction.entries.forEach((e, i) => applyChanges(e, entries[i], i, errors))

            if (!errors.hasErrors()) {
                changedTransactions.push(transaction)
            }

        })

        if (!errors.hasErrors()) {
            saveTransactions(changedTransactions)
        }
    }


    function resolved(result) {
      msg = "Transactions saved."
      curTransaction = result
      loadTransactions()
      //close()
    }

    const createSecondEntry = (transaction) => {
        transaction.entries.push(
            Object.assign({}, transaction.entries[0], {
                id: zeros,
                transaction_id: transaction.id,
                entry_type: transaction.entries[0].entry_type == "Credit" ? "Debit" : "Credit"
            })
        )
    }

    function fetched(result) {
        curTransaction = result
        addButtonLabel = "Update"
        recorded = curTransaction.status != "Projected"
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }

    const saveTransactions = async (transactions) => {
        console.log(transactions)
        await invoke('update_transactions', {transactions: transactions}).then(resolved, rejected)
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

    /*
     * Transaction list functions.
     */

     const formatter = new Intl.NumberFormat('en-AU', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    })


    const getDebitAmount = (transaction, curAccount) => {
        return transaction.entry_type === "Debit" ? formatter.format(transaction.amount) : ''
    }

    const getCreditAmount = (transaction, curAccount) => {
        return transaction.entry_type === "Credit" ? formatter.format(transaction.amount) : ''
    }

    const getBalance = (transaction) => {
        return formatter.format(transaction.balance)
    }

    const getDate = (transaction) => {
        const date = new Date(transaction.date)

        switch ($config.display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return transaction.date
            default: return date.toLocaleDateString()
        }
    }

    const date_style = () => {
        switch ($config.display_date_format) {
            case "ISO": return ""
            default: return "align_right"
        }
    }

    const getDescription = (transaction) => {
        return transaction.description
    }
    const getEntry = (transaction) => {
        return transaction.entries.find(e => e.account_id == curEntry.account_id)
    }

    const projected = (t) => t.status == 'Projected' ? 'projected' : ''
    const date_class = date_style()
</script>
<div>
<div class="form">
    <div class="form-heading">Edit Multiple Transactions</div>
    {#if curTransaction && curTransaction.entries}
    <div class="toolbar">
    </div>
    {/if}
        {#if entries.length > 0 && !compoundMode}
        <div class="entries">
            <table>
                <tbody>
                <tr><td><div class="heading">Date</div></td><td><div class="heading">Description</div></td><td><div class="heading">Amount</div></td><td></td><td></td></tr>
                <tr>
                    <td><div class="date-input" class:error={errors.isInError("date")} ><DateInput bind:value={entries[0].realDate} {format} placeholder="" disabled="disabled"/></div></td>
                    <td class="description"><input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={entries[0].description}></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("amount")} bind:value={entries[0].amount} disabled="disabled"></td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="form-row2">
            {#if entries.length > 1}
            {#if entries[0].entry_type !== "Credit"}
            <Select bind:item={entries[0].account} items={$accounts} label="Debit" none={true} flat={true} disabled="disabled" />
            <Select bind:item={entries[1].account} items={$accounts} label="Credit" none={true} flat={true} />
            {/if}
            {#if entries[0].entry_type === "Credit"}
            <Select bind:item={entries[1].account} items={$accounts} label="Debit" none={true} flat={true} disabled="disabled" />
            <Select bind:item={entries[0].account} items={$accounts} label="Credit" none={true} flat={true} />
            {/if}
            {/if}
        </div>
        {/if}
        {#if compoundMode}
        <div class="entries">
            <table>
                <tbody>
                <tr><td><div class="heading">Date</div></td><td><div class="heading">Description</div></td><td><div class="heading">Amount</div></td><td><div class="heading">Debit</div></td><td><div class="heading">Credit</div></td></tr>
                {#each entries as e, i}
                <tr>
                    <td><div class="date-input" class:error={errors.isInError(i + "_date")} ><DateInput bind:value={e["realDate"]} {format} placeholder="" disabled="disabled"/></div></td>
                    <td class="description"><input id="desc" class="description-input-2" class:error={errors.isInError(i + "_description")} bind:value={e.description}></td>
                    <td><div class="select-adjust"><Select bind:item={e["account"]} items={$accounts} label="" none={false} flat={true} inError={errors.isInError(i + "_account")} /></div></td>
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
                        <div class="toolbar-icon" on:click="{handleAddClick}" title="Add row"><Icon icon="mdi:table-row-plus-after"  width="24"/></div>
                        <div class="toolbar-icon" class:greyed={entries.length <= 2} on:click="{handleRemoveClick}" title="Remove row"><Icon icon="mdi:table-row-remove"  width="24"/></div>
                    </div></td>
                    <td></td>
                    <td><div class="total">Totals</div></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={drTotal} disabled="disabled"></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={crTotal} disabled="disabled"></td></tr>
                </tbody>
            </table>
        </div>
        {/if}
    <div class="form-button-row">
        <div class="widget2 buttons-left">
            <input id="compound" type=checkbox bind:checked={compoundMode} on:change={afterToggle} disabled="disabled">
            <label for="compound">Compound entry</label>
        </div>
        <div class="widget2 buttons-left">
            <input id="compound" type=checkbox bind:checked={recorded} disabled="disabled">
            <label for="compound">Recorded</label>
        </div>
        <div class="widget buttons">
            <button on:click={close}>Close</button>
            <button on:click={onSave}>{addButtonLabel}</button>
        </div>
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
<div class="section-heading">
    <div class="form-heading">Selected Transactions</div>
</div>
<div class="scroller" id="scroller">
    {#if transactions.length > 0}
    <table>
        <tbody>
        <tr><th></th><th class="justify-left">Date</th><th class="justify-left">Description</th><th>Debit</th><th>Credit</th><th>Balance</th></tr>
        {#each transactions as t}
            {@const e =  getEntry(t)}
            {#if e}
            <tr id={t.id}><!--{t.id}-->
                <td on:click|stopPropagation={() => toggleSelected(t)}><input id={"selected_" + t.id} type=checkbox checked={true} disabled={true}/></td>
                <td class={projected(t) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(t)} title="{e.description}"><div class="description">{e.description}</div>
                    {#each t.entries as en}
                        {#if en.account_id != curEntry.account_id}
                        <div class="description tiny">{$accounts.find(a => a.id == en.account_id).name}</div>
                        {/if}
                    {/each}
                </td>
                <td class="{projected(t)} money">{getDebitAmount(e, curEntry.account_id)}</td>
                <td class="{projected(t)} money">{getCreditAmount(e, curEntry.account_id)}</td>
                <td class="{projected(t)} money">{getBalance(e)}</td>
            </tr>
            {/if}
        {/each}
        </tbody>
    </table>
    {/if}
    {#if transactions.length < 1}
    <div class="message">No transactions</div>
    {/if}
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
        clear: left;
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

    .section-heading {
        width: 100%;
        clear: both;
        padding-top: 50px;
    }

    /* transactions styles*/
    .scroller {
        height: 100%;
        width: 100%;
        overflow: scroll;
    }

    .scroller table {
        padding-right: 10px;
    }

    .scroller td {
        text-align: left;
        overflow: hidden;
        line-height: 1em;
        color: #ccc;
        background-color: #393939;
        padding: 8px;
        white-space: nowrap;
        font-size: 0.9em;
    }

    .align_right {
        text-align: right;
    }

    .projected {
        color: #878787;
    }

    .scroller th {
        color:#666666;
        background-color: #444;
        font-weight: 400;
        font-size: .8em;
    }
    .justify-left {
        text-align: left;
        padding-left: 10px;
    }

    .money {
        text-align: right !important;
        min-width: 92px;
        font-family: 'Courier New', Courier, monospace;
        font-weight: bold;
    }

    .description {
        min-width: 350px;
        max-width: 33vw;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .tiny {
        font-size: 0.5em;
        color: #878787;
        margin: 3px 0 -5px 2px;
    }

    .account {
        float: left;
    }
</style>