<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'

    export let close
    export let curAccount
    export let curTransaction
    export let accounts = []
    export let editMode = "ADD"

    let drAccount
    let crAccount
    let msg = ""
    let errors = new Errors();
    let date = new Date(), description, amount
    let format="yyyy-MM-dd"
    let addButtonLabel = "Add"

    onMount(() => {
        console.log(editMode, curTransaction)
        if (editMode == "EDIT") {
            console.log(curTransaction)
            fetchTransaction(curTransaction.transaction_id)

        } else {
            drAccount = null
            crAccount = curAccount
            addButtonLabel = "Add"
        }
    });

    const matchAccount = (transaction, type) =>  {
        let typeEntries = transaction["entries"].filter(e => e.transaction_type === type)
        if (typeEntries.length == 0) return null
        let accountId = typeEntries[0].account_id
        let match = accounts.filter(a => a.id == accountId)
        return match.length > 0 ? match[0] : null
    }

    const getAmount = (transaction) =>  {
        let entries = transaction["entries"].filter(e => e.account_id === curAccount.id)
        if (entries.length == 0) return 0
        return entries[0].amount
    }

    const onCancel = () => {
        close()
    }

    const onAdd = () => {
        msg = "";
        errors = new Errors();
        if (!description || description.length < 1) {
             errors.addError("description", "Description is required")
        }

        if (!date || date.length < 1) {
            errors.addError("date", "Date is required")
        }

        if (!amount || amount.length < 1 || isNaN(amount)) {
            errors.addError("amount", "A valid amount is required")
        }

        if (!errors.hasErrors()) {
            let drAccountId = drAccount? drAccount.id : null
            let crAccountId = crAccount? crAccount.id : null
            let dateStr = date.getFullYear()+ "-" + (date.getMonth()+1) + "-" + date.getDate()
            const transaction = {
                    date: dateStr,
                    description: description,
                    entries: [
                    {
                        date: dateStr,
                        description: description,
                        account_id: drAccountId,
                        transaction_type: "Debit",
                        amount: amount,
                        status: "Recorded",
                        schedule_id: null
                    },
                    {
                        date: dateStr,
                        description: description,
                        account_id: crAccountId,
                        transaction_type: "Credit",
                        amount: amount,
                        status: "Recorded",
                        schedule_id: null
                    }
                ]
            }

            if (editMode == "ADD") {
                transaction["id"] = "00000000-0000-0000-0000-000000000000";
                transaction["entries"][0]["id"] = "00000000-0000-0000-0000-000000000000";
                transaction["entries"][0]["transaction_id"] = "00000000-0000-0000-0000-000000000000";
                transaction["entries"][1]["id"] = "00000000-0000-0000-0000-000000000000";
                transaction["entries"][1]["transaction_id"] = "00000000-0000-0000-0000-000000000000";
                addTransaction(transaction)
            } else if (editMode == "EDIT") {
                transaction["id"] = curTransaction.id;
                transaction["entries"][0]["id"] = curTransaction["entries"][0]["id"];
                transaction["entries"][0]["transaction_id"] = curTransaction.id;
                transaction["entries"][1]["id"] = curTransaction["entries"][1]["id"];
                transaction["entries"][1]["transaction_id"] = curTransaction.id;
                saveTransaction(transaction)
            }
        }

    }
    function resolved(result) {
      msg = "Transaction saved.."
    }

    function fetched(result) {
        curTransaction = result
        description = curTransaction.description
        amount = getAmount(curTransaction)
        addButtonLabel = "Update"
        drAccount = matchAccount(curTransaction, "Debit")
        crAccount = matchAccount(curTransaction, "Credit")
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


</script>

<div class="form">
    <div class="panel">
        <div class="form-row">
            <div class="widget date-time-field" class:error-input={errors.isInError("date")}>
                <label for="date">Date</label>
                <DateInput bind:value={date} {format} placeholder="" />
            </div>
            <div class="widget">
                <label for="desc">Description</label>
                <input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={description}>
            </div>
            <div class="widget">
                <label for="amount">Amount</label>
                <input id="amount" class="money-input" class:error={errors.isInError("amount")} bind:value={amount}>
            </div>
        </div>
        <div class="form-row2">
            <Select bind:item={drAccount} items={accounts} label="Debit" none={true}/>
            <Select bind:item={crAccount} items={accounts} label="Credit" none={true} />
        </div>
    </div>
    <div class="form-button-row">
        <div class="widget">
            {#each errors.getErrorMessages() as e}
            <p class="error-msg">{e}</p>
            {/each}
            {#if msg}
            <p class="success-msg">{msg}</p>
            {/if}
        </div>
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

    .error-msg {
        color: red;
    }

    .success-msg {
        color: green;
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

    .form label {
        text-align: left;
        font-size: .8em;
        color: #333;
        margin-bottom: 3px;
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

</style>