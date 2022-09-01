<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'

    export let close
    export let curTransaction
    export let accounts = []
    export let editMode = "ADD"
    export let settings

    const zeros = '00000000-0000-0000-0000-000000000000'
    let msg = ""
    let errors = new Errors();
    let format="yyyy-MM-dd"
    let addButtonLabel = "Add"
    let drTotal = 0
    let crTotal = 0
    let simpleAllowed = false
    let compound = false
    console.log(accounts)

    let entries = []

    onMount(() => {
        console.log(editMode, curTransaction)
        if (editMode == "EDIT") {
            console.log(curTransaction)
            fetchTransaction(curTransaction.transaction_id)
        } else {
            let date = new Date();
            entries = [
                {realDate: new Date(date), description: "", amount: 0, drAmount: '', crAmount: '', transaction_type: "Debit", account: {}, status:"Recorded"},
                {realDate: new Date(date), description: "", amount: 0, drAmount: '', crAmount: '', transaction_type: "Credit", account: {}, status:"Recorded"},
            ]
            addButtonLabel = "Add"
            simpleAllowed = true
        }

    });

    const handleAddClick = () => {
        entries = [...entries, {id: zeros, transaction_id: curTransaction.id, date: new Date(), description: "", amount: 0, drAmount: '', crAmount: '', account: {}, transaction_type: "Debit", status:"Recorded"}]
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
        const prefix = compound ? index + "_" : ""
        if (!entry.description || entry.description.length < 1) {
             errors.addError(prefix + "description", "Description is required")
        }

        if (!entry.realDate || entry.realDate.length < 1) {
            errors.addError(prefix + "date", "Date is required")
        }

        if (!compound) {
            if (!entry.amount || entry.amount.length < 1 || isNaN(entry.amount)) {
                errors.addError("amount", "A valid amount is required")
            }
        } else if (entry.transaction_type === "Credit") {
            if (!entry.crAmount || entry.crAmount.length < 1 || isNaN(entry.crAmount)) {
                errors.addError(prefix + "crAmount", "A valid amount is required")
            }
        } else {
            if (!entry.drAmount || entry.drAmount.length < 1 || isNaN(entry.drAmount)) {
                errors.addError(prefix + "drAmount", "A valid amount is required")
                console.log(entry.drAmount)
            }
        }

        if (!entry.account || !entry.account.id || entry.account.id < 1) {
            if (settings.require_double_entry || compound) {
                errors.addError(index + "_account", "Account is required")
            }
        }

        if (compound && (drTotal != crTotal)) {
            errors.addError("totals", "Totals should balance")
        }

        return errors
    }

    const toDateStr = (date) => {
        return date.getFullYear()+ "-" + (date.getMonth()+1) + "-" + date.getDate()
    }

    const onAdd = () => {
        msg = "";
        errors = new Errors();
        if (!compound) syncSecondEntry(entries)
        entries.forEach((e, i) => validateEntry(e, i, errors))

        if (!errors.hasErrors()) {
            const transaction = {
                    date: toDateStr(new Date()),
                    description: "",
                    entries: [...entries]
            }

            if (!compound && !settings.require_double_entry) {
                transaction.entries = transaction.entries.filter(e => (e["account"] && e["account"]["id"]))
            }

            transaction.entries.forEach (
                e => {
                    e["date"] = toDateStr(e.realDate)
                    e["account_id"] = e["account"]["id"]
                    if (compound) {
                        e["amount"] = (e["transaction_type"] === "Credit") ? e["crAmount"] : e["drAmount"]
                    }
                }
            )

            if (editMode == "ADD") {
                transaction["id"] = zeros
                transaction.entries.forEach (
                    e => {
                        e["id"] = zeros
                        e["transaction_id"] = zeros
                    }
                )
                addTransaction(transaction)
            } else if (editMode == "EDIT") {
                transaction["id"] = curTransaction.id;
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
        entries[1].id = zeros
        entries[1].transaction_id = entries[0].transaction_id
        entries[1].transaction_type =  "Credit"
        entries[1].realDate = new Date(entries[0].realDate)
        entries[1].description = entries[0].description
        entries[1].amount = entries[0].amount
        entries[1].status = "Recorded"
    }

    const canBeSimple = (entries) => {
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
        addButtonLabel = "Update"
        console.log(result)
        entries = curTransaction.entries

        entries.forEach(e => {
            e.transaction_type === "Credit" ? e.crAmount = e.amount : e.drAmount = e.amount
            e.realDate = new Date(e.date)
            e.account = matchAccount(e.account_id)
        })

        if (entries.length == 1) {
            entries.push({})
            syncSecondEntry(entries)
            entries[1].account = null
            entries[0].transaction_type === "Credit" ? entries[1].drAmount = entries[1].amount : entries[1].crAmount = entries[1].amount
        }

        simpleAllowed = canBeSimple(entries)
        compound = !simpleAllowed
        console.log(entries)
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }
    const addTransaction = async (transaction) => {
        console.log(transaction)
   		await invoke('add_transaction', {transaction: transaction}).then(resolved, rejected)
	};

    const saveTransaction = async (transaction) => {
        console.log(transaction)
   		await invoke('update_transaction', {transaction: transaction}).then(resolved, rejected)
	};

    const fetchTransaction = async (transactionId) => {
        console.log(transactionId)
   		await invoke('transaction', {transactionId: transactionId}).then(fetched, rejected)
	};

    const showAmount = (entry, type) => {
        if (entry["drAmount"] > 0) {
            entry["transaction_type"] = "Debit"
            return type === "Debit"
        }

        if (entry["crAmount"] > 0) {
            entry["transaction_type"] = "Credit"
            return type === "Credit"
        }

        return true
    }

    const total = (type) => {
        let total = 0
        entries.filter(e => e.transaction_type === type).forEach(e => total += Number(e[type === "Credit" ? "crAmount" : "drAmount"]))
        return total
    }

    const calculateTotals = () => {
        drTotal = total("Debit")
        crTotal = total("Credit")
    }


    const toggleMode = () => {
        if (!compound) syncSecondEntry(entries)
    }

    $: {
    	calculateTotals(entries)
	}


</script>
<div class="form">
    <div class="panel">
        {#if entries.length > 0 && !compound}
        <div class="entries">
            <table>
                <tr><td><div class="heading">Date</div></td><td><div class="heading">Description</div></td><td><div class="heading">Amount</div></td><td></td><td></td></tr>
                <tr>
                    <td><div class="date-input" class:error={errors.isInError("date")} ><DateInput bind:value={entries[0].realDate} {format} placeholder="" /></div></td>
                    <td class="description"><input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={entries[0].description}></td>
                    <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("amount")} bind:value={entries[0].amount}></td>
                </tr>
            </table>
        </div>
        <div class="form-row2">
            {#if entries.length > 1}
            {#if entries[0].transaction_type !== "Credit"}
            <Select bind:item={entries[0].account} items={accounts} label="Debit" none={true}/>
            <Select bind:item={entries[1].account} items={accounts} label="Credit" none={true} />
            {/if}
            {#if entries[0].transaction_type === "Credit"}
            <Select bind:item={entries[1].account} items={accounts} label="Debit" none={true}/>
            <Select bind:item={entries[0].account} items={accounts} label="Credit" none={true} />
            {/if}
            {/if}
        </div>
        {/if}
        {#if compound}
        <div class="entries">
            <table>
                <tr><td><div class="heading">Date</div></td><td><div class="heading">Description</div></td><td><div class="heading">Amount</div></td><td><div class="heading">Debit</div></td><td><div class="heading">Credit</div></td></tr>
                {#each entries as e, i}
                <tr>
                    <td><div class="date-input" class:error={errors.isInError(i + "_date")} ><DateInput bind:value={e["realDate"]} {format} placeholder="" /></div></td>
                    <td class="description"><input id="desc" class="description-input-2" class:error={errors.isInError(i + "_description")} bind:value={e.description}></td>
                    <td><div class="select-adjust"><Select bind:item={e["account"]} items={accounts} label="" none={false} flat={true} inError={errors.isInError(i + "_account")}/></div></td>
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
        <div class="widget2 buttons-left">
            <input id="compound" type=checkbox bind:checked={compound} on:change={toggleMode} disabled={!(compound && canBeSimple(entries) || !compound && simpleAllowed)}>
            <label for="compound">Compound entry</label>
        </div>
        <div class="widget buttons">
            <button on:click={onCancel}>Close</button>
            <button on:click={onAdd}>{addButtonLabel}</button>
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
    }
    .error-msg {
        color: red;
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
        margin: 0px 5px -3px 0px;
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