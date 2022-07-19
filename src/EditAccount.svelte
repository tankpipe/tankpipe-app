<script>
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    
    export let close
    export let curAccount
    export let loadAccounts
    export let editMode = "NEW"
        
    let msg = ""
    let errors = new Errors()
    let name, startingBalance
    let isDebit = true
    let addButtonLabel = "Add"
    
    onMount(() => { 
        if (editMode == "EDIT") {
            name = curAccount.name 
            startingBalance = curAccount.starting_balance
            isDebit = curAccount.account_type == "Debit"
            addButtonLabel = "Update"
        } else {
            addButtonLabel = "Add"
        }
    });
    
    const onCancel = () => {
        close()
    }

    const onAdd = () => {
        msg = "";
        errors = new Errors();
        if (!name || name.length < 1) {
             errors.addError("name", "Name is required")
        }
      
        if (!startingBalance || startingBalance.length < 1 || isNaN(startingBalance)) {
            errors.addError("startingBalance", "A valid starting balance is required")
        }

        if (!errors.hasErrors()) {

            if (editMode == "ADD") {
                const accountType = isDebit ? "Debit" : "Credit"
                const account = {
                    name: name,
                    starting_balance: startingBalance, 
                    account_type: accountType
                }

                addAccount(account)
            } else if (editMode == "EDIT") {
                const account = {
                    name: name,
                    starting_balance: startingBalance, 
                    account_type: curAccount.account_type,
                    id: curAccount.id,
                    balance: 0
                }
                saveAccount(account)
            } 

        }
        
    }
    function resolved(result) {
      msg = "Account added."
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }
    const addAccount = async (account) => {
   		await invoke('add_account', {account: account}).then(resolved, rejected)
        loadAccounts()
	};

    const saveAccount = async (account) => {
        console.log(account)
   		await invoke('edit_account', {account: account}).then(resolved, rejected)
         loadAccounts()
	};


</script>

<div class="form">
    <div class="panel">
        <div class="form-row">
            <div class="widget">
                <label for="name">Name</label>
                <input id="name" class="description-input" class:error={errors.isInError("name")} bind:value={name}>                
            </div>
            <div class="widget">
                <label for="startingBalance">Starting balance</label>
                <input id="startingBalance" class="money-input" class:error={errors.isInError("startingBalance")} bind:value={startingBalance}>
            </div>
            <div class="widget2">            
                <input id="debit" type="radio" bind:group={isDebit} value={true} class="" name="transactionType" disabled={editMode == "EDIT"}>
                <label for="debit">Debit</label>
                <br/>
                <input id="credit" type="radio" bind:group={isDebit} value={false} class="" name="transactionType" disabled={editMode == "EDIT"}>
                <label for="credit">Credit</label>
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

    .form-button-row {
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
        padding: 5px;
        border-radius: 10px;
        float: left;
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
		width: 110px;
	}

    .money-input {
		text-align: right;
	}

	.description-input {
		width: 400px;
	}

</style>