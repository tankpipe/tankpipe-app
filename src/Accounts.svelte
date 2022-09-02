<script>
    import EditAccount from "./EditAccount.svelte"

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
	<div class="toolbar"><i class="gg-add-r" on:click="{handleAddClick(curAccount)}"></i></div>
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
        <div class="card" on:click={() => selectAccount(a) }>{a.name}</div>
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
		text-align: left;
		margin-left: 10px;
	}

    .account-heading {
		text-align: left;
	}

    .toolbar {
		float: right;
		color: #C0C0C0;
	}

	.toolbar i:hover{
		color: #F0F0F0;
		border-color: #f0f0f0;
	}

	.card {
		float: left;
		clear: both;
		margin: 10px;
		background-color: #524e4e;
		padding: 10px;
		border-radius: 10px;
		color: #F0F0F0;
		min-width: 300px;
		text-align: left;
	}

	.card:hover {
		cursor: pointer;
		color: #FFF;
	}

    .gg-add-r {
		box-sizing: border-box;
		position: relative;
		display: block;
		width: 22px;
		height: 22px;
		border: 2px solid;
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

</style>