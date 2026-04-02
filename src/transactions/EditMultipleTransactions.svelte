<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from '../utils/errors'
    import Select from '../components/Select.svelte'
    import Icon from '@iconify/svelte'
    import MessagePanel from '../components/MessagePanel.svelte'
    import {accounts} from '../stores/accounts'
    import {config, dateFormat} from '../stores/config'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'
    import { disabledItemsByIndex } from '../utils/disabledItems'

    export let loadTransactions
    export let onClose
    export let curAccount
    export let transactions

    const zeros = '00000000-0000-0000-0000-000000000000'
    let msg = ""
    let errors = new Errors()
    let format = dateFormat($config)
    let addButtonLabel = "Update"
    let drTotal = 0
    let crTotal = 0
    let simpleAllowed = false
    let compoundMode = false
    let recorded = false
    let entries = []
    let curTransaction
    let disabledAccountsByEntryIndex = []

    $: disabledAccountsByEntryIndex = disabledItemsByIndex(entries, (e) => e?.account?.id)

    const resetChanges = () => {
        entries = [
            {realDate: null, description: "", amount: '', drAmount: '', crAmount: '', entry_type: 'Debit', account: {}},
            {realDate: null, description: "", amount: '', drAmount: '', crAmount: '', entry_type: 'Credit', account: {}},
        ]
    }

    // Ensure form fields render immediately (tests + UI) without waiting for onMount.
    resetChanges()

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

    const resolvedEntryType = (entry) => {
        if (Number(entry?.drAmount) > 0) return "Debit"
        if (Number(entry?.crAmount) > 0) return "Credit"
        return entry?.entry_type
    }

    const applyChanges = (entry, changeEntry, index, errors) => {

        const updateDescription = (changeEntry) => {
            if (changeEntry.description && changeEntry.description.length > 0) {
                entry.description = changeEntry.description
            }
        }

        if (compoundMode) {
            updateDescription(entries[index])
        } else {
            updateDescription(entries[0])
        }

        if (entries[index].account && entries[index].account.id) {
            entry.account_id = entries[index].account.id
            entry.entry_type = resolvedEntryType(entries[index])
        }

        console.log(entry, entries[index])
    }

    const needSecondEntry = (transaction) =>  {
        if (transaction.entries.length == 1) {
            console.log(entries.filter(e => e.account && e.account.id && resolvedEntryType(e) !== transaction.entries[0].entry_type))
            return entries.filter(e => e.account && e.account.id && resolvedEntryType(e) !== transaction.entries[0].entry_type).length > 0
        }
        return false
    }

    const sortEntries = (entries) => {
        return [...entries].sort((a, b) => {
            if (a.entry_type === "Debit" && b.entry_type === "Credit") return -1
            if (a.entry_type === "Credit" && b.entry_type === "Debit") return 1
            return 0
        })
    }

    const onSave = () => {
        msg = ""
        errors = new Errors()

        const changedTransactions = []
        transactions.forEach(t => {
            // `transactions` may come from reactive proxy objects; use JSON cloning
            // to produce a plain payload for safe processing.
            const transaction = JSON.parse(JSON.stringify(t))

            if (needSecondEntry(transaction)) {
                console.log("Creating second entry ", transaction)
                createSecondEntry(transaction)
            } else if (transaction.entries.length > 1) {
                sortEntries(transaction.entries)
            }

            transaction.entries.forEach((e, i) => applyChanges(e, entries[i], i, errors))

            if (!errors.hasErrors()) {
                changedTransactions.push(transaction)
            }

        })

        if (!errors.hasErrors()) {
            console.log(changedTransactions)
            saveTransactions(changedTransactions)
        }
    }

    function resolved(result) {
      msg = "Transactions saved."
      curTransaction = result
      loadTransactions()
      close()
    }

    const createSecondEntry = (transaction) => {
            const newEntry = Object.assign({}, transaction.entries[0], {
                id: zeros,
                transaction_id: transaction.id,
                account_id: '',
                entry_type: transaction.entries[0].entry_type == "Credit" ? "Debit" : "Credit"
            })

        if (transaction.entries[0].entry_type == "Debit") {
            transaction.entries.push(newEntry)
        } else {
            transaction.entries.unshift(newEntry)
        }
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }

    const saveTransactions = async (transactions) => {
        console.log(transactions)
        await invoke('update_transactions', {transactions: transactions}).then(resolved, rejected)
    }

    const total = (type) => {
        let total = 0
        entries
            .filter(e => resolvedEntryType(e) === type)
            .forEach(e => total += Number(e[type === "Credit" ? "crAmount" : "drAmount"]))
        return Number(total).toFixed(2)
    }

    $: drTotal = total("Debit")
    $: crTotal = total("Credit")

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

    const projected = (t) => t.status == 'Projected' ? 'projected' : ''
    const date_class = date_style()
</script>
<div class="form">
    <div class="form-heading">{$_('editMultiple.form.editMultiple')}</div>
    <div class="toolbar toolbar-right">
        <button class="toolbar-icon" on:click={close} title={$_('buttons.close')}>
            <Icon icon="mdi:close-box-outline" width="24"/>
        </button>
    </div>
     <div class="form-row">
        <div class="small-text">{$_('editMultiple.overview')}</div>
    </div>
        {#if entries.length > 0 && !compoundMode}
        <div class="entries">
            <table>
                <tbody>
                <tr><td><div class="heading">{$_('labels.date')}</div></td><td><div class="heading">{$_('labels.description')}</div></td><td><div class="heading">{$_('labels.amount')}</div></td><td></td><td></td></tr>

                <tr>
                    <td><div class="date-input" class:error={errors.isInError("date")} ><DateInput value={entries[0].realDate} {format} placeholder="" disabled="disabled"/></div></td>
                    <td><input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={entries[0].description}></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("amount")} value={entries[0].amount} disabled="disabled"></td>
                </tr>
                </tbody>
            </table>
        </div>
        <div class="form-row2">
            <Select bind:item={entries[0].account} items={$accounts} disabledItems={disabledAccountsByEntryIndex[0] || []} label={$_('labels.debit')} none={true} flat={true}/>
            <Select bind:item={entries[1].account} items={$accounts} disabledItems={disabledAccountsByEntryIndex[1] || []} label={$_('labels.credit')} none={true} flat={true}/>
        </div>
        {/if}
        {#if compoundMode}
        <div class="entries">
            <table>
                <tbody>
                <tr><td><div class="heading">{$_('labels.date')}</div></td><td><div class="heading">{$_('labels.description')}</div></td><td><div class="heading">{$_('labels.account')}</div></td><td><div class="heading">{$_('labels.debit')}</div></td><td><div class="heading">{$_('labels.credit')}</div></td></tr>
                {#each entries as e, i}
                <tr>
                    <td><div class="date-input" class:error={errors.isInError(i + "_date")} ><DateInput value={entries[i]["realDate"]} {format} placeholder="" disabled="disabled"/></div></td>
                    <td><input id="desc" class="description-input-2" class:error={errors.isInError(i + "_description")} bind:value={entries[i].description}></td>
                    <td><div class="select-adjust">
                        <Select
                            bind:item={entries[i].account}
                            items={$accounts}
                            disabledItems={disabledAccountsByEntryIndex[i] || []}
                            label=""
                            none={true}
                            flat={true}
                            inError={errors.isInError(i + "_account")}
                        />
                    </div></td>
                    <td class="money">
                        <input id="amount" class="money-input" class:error={errors.isInError(i + "_drAmount")} bind:value={entries[i].drAmount}>
                    </td>
                    <td class="money">
                        <input id="amount" class="money-input" class:error={errors.isInError(i + "_crAmount")} bind:value={entries[i].crAmount}>
                    </td>
                </tr>
                {/each}
                <tr>
                    <td><div class="toolbar bottom-toolbar">
                        <button class="toolbar-icon" on:click="{handleAddClick}" title={$_('editMultiple.table.addRow')}><Icon icon="mdi:table-row-plus-after"  width="24"/></button>
                        <button class="toolbar-icon" class:greyed={entries.length <= 2} on:click="{handleRemoveClick}" title={$_('editMultiple.table.removeRow')}><Icon icon="mdi:table-row-remove"  width="24"/></button>
                    </div></td>
                    <td></td>
                    <td><div class="total">{$_('labels.totals')}</div></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} value={drTotal} disabled="disabled"></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} value={crTotal} disabled="disabled"></td></tr>
                </tbody>
            </table>
        </div>
        {/if}
    <div class="form-button-row">
        <div class="widget2 buttons-left">
            <input
                id="compound"
                type=checkbox
                checked={compoundMode}
                on:change={(event) => compoundMode = event.currentTarget.checked}
                disabled={false}
            >
            <label for="compound">{$_('editMultiple.form.compoundEntry')}</label>
        </div>
        <div class="widget2 buttons-left">
            <input id="recorded" type=checkbox bind:checked={recorded} disabled="disabled">
            <label for="recorded">{$_('editMultiple.form.recorded')}</label>
        </div>
        <div class="widget buttons">
            <button class="og-button" on:click={close}>{$_('buttons.close')}</button>
            <button class="og-button" on:click={onSave}>{addButtonLabel}</button>
        </div>
    </div>
    <MessagePanel {errors} {msg} />
</div>
<div class="section-heading">
    <div class="form-heading">{$_('editMultiple.form.selectedTransactions')}</div>
</div>

<div class="scroller" id="scroller">
    <table>
        <tbody>
        <tr>
            <th></th><th class="justify-left">{$_('labels.date')}</th><th class="justify-left">{$_('labels.description')}</th><th>{$_('labels.debit')}</th><th>{$_('labels.credit')}</th>
        </tr>
        {#each transactions as t}
            {@const sortedEntries = sortEntries(t.entries)}
            <tr style="height: 8px;"></tr>
            {#each sortedEntries as e}
            <tr id={t.id + "_" + e.id}><!--{t.id}-->
                <td><input id={"selected_" + t.id} type=checkbox checked={true} disabled={true}></td>
                <td class={projected(t) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(t)} style="{e.entry_type == 'Credit' ? 'padding-left: 30px' : ''}" title="{e.description}"><div class="description">{$accounts.find(a => a.id == e.account_id).name}</div>
                    <div class="description tiny">{e.description}</div>
                </td>
                <td class="{projected(t)} money">{getDebitAmount(e, curAccount)}</td>
                <td class="{projected(t)} money">{getCreditAmount(e, curAccount)}</td>
            </tr>
            {/each}
        {/each}
        </tbody>
    </table>
</div>

<style>

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
        color: var(--color-warning);
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
    }

    .success-msg {
        color: var(--color-success);
        text-align: left;
    }

    .disabled {
        background-color: var(--color-text-strong);
    }

    .greyed:hover {
        color: var(--color-border) !important;
        border-color: var(--color-border) !important;
        cursor: default !important;
    }

    .error {
        border: 1px solid var(--color-warning) !important;
    }

    :global(.error-input input) {
        border: 1px solid var(--color-warning) !important;
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

    .total {
        text-align: right;
        margin: 0px 5px -3px 0px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
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
        clear: left;
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
        color: var(--color-table-cell-text);
        background-color: var(--color-table-cell-bg);
        padding: 8px;
        white-space: nowrap;
        font-size: 0.9em;
    }

    .projected {
        color: var(--color-text-dim);
    }

    .scroller th {
        color:var(--color-border);
        background-color: var(--color-bg);
        font-weight: 400;
        font-size: .8em;
    }

    .money {
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
        color: var(--color-text-dim);
        margin: 3px 0 -5px 2px;
    }

</style>
