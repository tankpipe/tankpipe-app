<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    
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
            description = curTransaction.description
            amount = curTransaction.amount
            addButtonLabel = "Update"
            drAccount = matchAccount(curTransaction.dr_account_id)
            crAccount = matchAccount(curTransaction.cr_account_id)
        } else {
            drAccount = null
            crAccount = curAccount
            addButtonLabel = "Add"
        }
    });

    const matchAccount = (accountId) =>  {
        if (!accountId) return null
        let match = accounts.filter(a => a.id == accountId)
        return match.length > 0 ? match[0] : null
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

            if (editMode == "ADD") {
                const transaction = {
                    date: dateStr, 
                    description: description, 
                    amount: amount, 
                    dr_account_id: drAccountId,
                    cr_account_id: crAccountId,
                    status: "Recorded"
                }

                addTransaction(transaction)
            } else if (editMode == "EDIT") {
                const transaction = {
                    id: curTransaction.id,
                    date: dateStr, 
                    description: description, 
                    amount: amount, 
                    dr_account_id: drAccountId,
                    cr_account_id: crAccountId,
                    status: "Recorded"
                }
                saveTransaction(transaction)
            } 
        }
        
    }
    function resolved(result) {
      msg = "Transaction added."
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
            <div class="widget">
                <label for="account1">Debit</label>
                <select bind:value={drAccount} >
                    <option value={null}>None</option>
                    {#each accounts as a}
                    <option value={a}>{a.name}</option>
                    {/each}	
                </select>
            </div>
            <div class="widget">
                <label for="account2">Credit</label>
                <select bind:value={crAccount} >
                    <option value={null}>None</option>
                    {#each accounts as a}
                    <option value={a}>{a.name}</option>
                    {/each}	
                </select>
            </div>

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

    .widget2 {
        padding: 5px 0px 5px 10px;  
        margin: 13px 12px 0px 0px;
    }
    
    .widget2 label {
        display: inline-block;
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

</style>