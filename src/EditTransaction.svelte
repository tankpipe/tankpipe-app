<script>
    import { DateInput } from 'date-picker-svelte'
    import {Errors} from './errors.js'
    
    export let close
    export let curAccount
    export let accounts = []
    
    let otherAccount
    let msg = ""
    let errors = new Errors();
    let date = new Date(), description, amount 
    let isDebit = !curAccount || curAccount.account_type === 'Debit'
    let otherAccountLabel = isDebit ? "From" : "To"
    let format="yyyy-MM-dd"

    $: {
        otherAccountLabel = isDebit ? "From" : "To"
    }
    const onCancel = () => {
        close()
    }

    const handleSelectAccount = (e) => {
		console.log("selected: " + e);   
		console.log(otherAccount)			
	};

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
            let otherAccountId = otherAccount ? otherAccount.id : null
            let drAccountId = isDebit? curAccount.id : otherAccountId
            let crAccountId = isDebit? otherAccountId : curAccount.id
            let dateStr = date.getFullYear()+ "-" + (date.getMonth()+1) + "-" + date.getDate()

            const transaction = {
                date: dateStr, 
                description: description, 
                amount: amount, 
                dr_account_id: drAccountId,
                cr_account_id: crAccountId,
                status: "Recorded"
            }

            addTransaction(transaction)
        }
        
    }
    function resolved(result) {
      msg = "Transaction added."
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "Transaction contains an error: " + result)
    }
    const addTransaction = async (transaction) => {
        console.log(transaction)
   		await invoke('add_transaction', {transaction: transaction}).then(resolved, rejected)
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
            <div class="widget2">            
                <input id="debit" type="radio" bind:group={isDebit} value={true} class="" name="transactionType" >
                <label for="debit">Debit</label>
                <br/>
                <input id="credit" type="radio" bind:group={isDebit} value={false} class="" name="transactionType">
                <label for="credit">Credit</label>
            </div>
        </div>
        <div class="form-row2">
            <div class="widget">
                <label for="account2">{otherAccountLabel}</label>
                <select bind:value={otherAccount} on:change="{handleSelectAccount}">
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
            <button on:click={onAdd}>Add</button>
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