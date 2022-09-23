<script>
    import EditAccount from "./EditAccount.svelte"
	import Icon from '@iconify/svelte';
    import { page, isEditMode, modes, views } from "./page";

    export let curAccount
    export let accounts
    export let loadAccounts

	let ACCOUNT_TYPES = {
		Asset: "Assets",
		Liability: "Liabilities",
		Revenue: "Revenues",
		Expense: "Expenses",
		Equity: "Equity"
	}
	let lastAccountType = "";

	$: {
		if (accounts)
			if (accounts.length < 1) {
				page.set({view: views.ACCOUNTS, mode: modes.NEW})
			}
	}

	const close = () => {
        console.log("close")
		page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

    const handleAddClick = () => {
		console.log("addClick")
		curAccount = null
		page.set({view: views.ACCOUNTS, mode:modes.NEW})
	}

    const selectAccount = (account) => {
		console.log(account)
        curAccount = account
		console.log(curAccount.id)
		page.set({view: views.TRANSACTIONS, mode: modes.LIST})
    }

	const editAccount = (account) => {
		console.log(account)
        curAccount = account
		page.set({view: views.ACCOUNTS, mode: modes.EDIT})
        console.log("selected: " + curAccount.name);
    }

	const checkAccountType = (account) => {
		if (account.account_type !== lastAccountType) {
			lastAccountType = account.account_type
			return true
		}
		return false
	}

</script>

<div class="account-heading">
	{#if !isEditMode($page)}
	<div class="toolbar"><div class="toolbar-icon" on:click="{handleAddClick}" title="Create a new account"><Icon icon="mdi:plus-box-outline"  width="24"/></div></div>
	{/if}
</div>


{#if isEditMode($page)}
<EditAccount {curAccount} {loadAccounts} {close} initialize={accounts.length < 1}/>
{/if}
{#if !isEditMode($page)}
<div class="form-heading">Accounts</div>
<div class="scroller">
	<div class="accounts">
    {#each accounts as a}
	{#if checkAccountType(a)}
		<div class="account-type">{ACCOUNT_TYPES[a.account_type]}</div>
	{/if}
        <div class="card" on:click={() => selectAccount(a) }>{a.name}<div class="edit-icon" on:click={() => editAccount(a) }><Icon icon="mdi:pencil" /></div></div>
    {/each}
	</div>
</div>
{/if}
<style>

	.scroller{
		height: 100%;
		width: 100%;
		overflow: scroll;
		margin-top: 15px;
	}

	.accounts {
		margin-right: 20px;
	}

	.account-type {
		font-size: 0.8em;
        font-weight: 500;
        color: #757575;
		margin: 5px 0px -5px 10px;
		float: left;
		clear: both;
	}

	.edit-icon {
		float: right;
		color: #524e4e;
	}

	.card:hover .edit-icon {
		color: #666;
	}

	.edit-icon:hover {
		color: #C0C0C0 !important;
	}

    .card {
		float: left;
		clear: both;
		margin: 10px;
		background-color: #524e4e;
		padding: 10px;
		border-radius: 10px;
		color: #E0E0E0;
		min-width: 300px;
		text-align: left;
	}

	.card:hover {
		cursor: pointer;
		color: #FFF;
	}

</style>