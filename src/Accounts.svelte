<script>
    import EditAccount from "./EditAccount.svelte"
	import Icon from '@iconify/svelte';

    export let curAccount
    export let accounts
    export let loadAccounts
	export let selectMenu

	let ACCOUNT_TYPES = {
		Asset: "Assets",
		Liability: "Liabilities",
		Revenue: "Revenues",
		Expense: "Expenses",
		Equity: "Equity"
	}
    let mode = "ACCOUNTS"
    let editMode = "ADD"
	let lastAccountType = "";

	$: {
		if (accounts)
		console.log("accounts: " + accounts.length)
			if (accounts.length < 1) {
				mode = "EDIT"
			}
	}

	const close = () => {
        console.log("close")
        mode = "ACCOUNTS";
    }

    const handleAddClick = () => {
        editMode = "ADD"
		mode = "EDIT"
	}

    const selectAccount = (account) => {
		console.log(account)
        curAccount = account
		console.log(curAccount.id)
        selectMenu("TRANSACTIONS")
    }

	const editAccount = (account) => {
        curAccount = account
        editMode = "EDIT"
		mode = "EDIT"
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
	{#if mode === "ACCOUNTS"}
	<div class="toolbar"><div class="toolbar-icon" on:click="{handleAddClick(curAccount)}" title="Create a new account"><Icon icon="mdi:plus-box-outline"  width="24"/></div></div>
	{/if}
</div>


{#if mode === "EDIT"}
<EditAccount {curAccount} {loadAccounts} {close} {editMode}/>
{/if}
{#if mode === "ACCOUNTS"}
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
		margin-top: 30px;
	}

	.accounts {
		margin-right: 20px;
	}

	.account-type {
		font-size: 0.8em;
		color: #666;
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

    .account-heading {
		text-align: left;
	}

    .toolbar {
		float: right;
		color: #C0C0C0;
	}

	.toolbar-icon {
		margin-left: 5px;
	}

	.toolbar-icon:hover{
		color: #F0F0F0;
		cursor: pointer;
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