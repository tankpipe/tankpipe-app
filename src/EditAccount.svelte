<script>
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte';

    export let close
    export let curAccount
    export let loadAccounts
    export let editMode = "NEW"

    const ACCOUNT_TYPES = [{value:"Asset", name:"Asset"}, {value:"Liability", name:"Liability"}, {value:"Revenue", name:"Revenue"}, {value:"Expense", name:"Expense"}, {value:"Equity", name:"Equity"}]

    let msg = ""
    let errors = new Errors()
    let name, startingBalance, accountType
    let addButtonLabel = "Add"
    let initialize = !curAccount

    onMount(() => {
        if (editMode == "EDIT") {
            name = curAccount.name
            startingBalance = curAccount.starting_balance
            accountType = matchAccountType(curAccount.account_type)
            addButtonLabel = "Update"
        } else {
            addButtonLabel = "Add"
            startingBalance = "0"
        }
    });

    const matchAccountType = (value) =>  {
        if (!value) return null
        let match = ACCOUNT_TYPES.filter(p => p.value == value)
        return match.length > 0 ? match[0] : null
    }

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

        if (!accountType || !accountType.value) {
            errors.addError("accountType", "Account type needs to be selected")
        }

        if (!errors.hasErrors()) {

            if (editMode == "ADD") {
                const account = {
                    name: name,
                    starting_balance: startingBalance,
                    account_type: accountType.value
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
        initialize = false
	};

    const saveAccount = async (account) => {
        console.log(account)
   		await invoke('update_account', {account: account}).then(resolved, rejected)
         loadAccounts()
	};


</script>
{#if initialize}
<div class="message">To get started add your first account. Typically this will be your main bank account used for everyday transactions.</div>
{/if}

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
        </div>
        <div class="form-row">
            <Select bind:item={accountType} items={ACCOUNT_TYPES} label="Account type" none={false} inError={errors.isInError("accountType")}/>
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

    .message {
		color: #EFEFEF;
		margin-bottom: 20px;
		text-align: left;
		background-color: #303030;
		padding:10px;
		border-radius: 10px;
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
        min-width: 53px;
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