<script>
	import EditTransaction from './EditTransaction.svelte'
    import Select from './Select.svelte'

	export let curAccount
	export let accounts = []
	let mode = "TRANSACTIONS"
	let editMode = "ADD"
	let curTransaction

	const close = () => {
        console.log("close")
        mode = "TRANSACTIONS";
		if (curAccount) loadTransactions();
    }

	$: {
		if (curAccount) loadTransactions();
    }

	const selectTransaction = (transaction) => {
		curTransaction = transaction
		editMode = "EDIT"
		mode = "EDIT"
	}

	let transactions = [];
	export const loadTransactions = async () => {
		console.log("loadTransactions: " + curAccount.id);
   		transactions = await invoke('transactions', {accountId: curAccount.id});
		console.log(transactions)
	};

	const formatter = new Intl.NumberFormat('en-AU', {
		minimumFractionDigits: 2,
		maximumFractionDigits: 2,
	});
	const getDebitAmount = (transaction, curAccount) => {
		return transaction.transaction_type === "Debit" ? formatter.format(transaction.amount) : ''
	}

	const getCreditAmount = (transaction, curAccount) => {
		return transaction.transaction_type === "Credit" ? formatter.format(transaction.amount) : ''
	}

	const getBalance = (transaction) => {
		return formatter.format(transaction.balance)
	}

	const handleAddClick = () => {
		editMode = "ADD"
		mode = "EDIT"
	}

</script>


<div class="account-heading">
	{#if mode === "TRANSACTIONS" && curAccount}
	<Select bind:item={curAccount} items={accounts}/>
	<div class="toolbar"><i class="gg-add-r" on:click="{handleAddClick(curAccount)}"></i></div>
	{/if}
</div>
{#if mode === "EDIT"}
<EditTransaction {close} {curAccount} {accounts} {editMode} {curTransaction} />
{/if}
{#if mode === "TRANSACTIONS"}
<div class="scroller">
	<table>
		<tr><th>Date</th><th>Description</th><th>Debit</th><th>Credit</th><th>Balance</th></tr>
		{#each transactions as t}
			<tr on:click={() => selectTransaction(t)}><!--{t.id}--><td>{t.date}</td><td class="description">{t.description}</td><td class="money">{getDebitAmount(t, curAccount)}</td><td class="money">{getCreditAmount(t, curAccount)}</td><td class="money">{getBalance(t)}</td></tr>
		{/each}
	</table>
</div>
{/if}

<style>
    .scroller{
		height: 100%;
		overflow: scroll;
	}
	td {
		text-align: left;
		overflow: hidden;
		line-height: 1em;
		color: #444;
		background-color: #f0f0f0;
		padding: 8px;
	}

	th {
		color:#f0f0f0;
		background-color: #444;
		font-weight: 400;
		font-size: .8em;
	}

	tr:hover td {
		cursor: pointer;
		background-color: #FFF;
	}

	.money {
		text-align: right !important;
		min-width: 100px;
		font-family: 'Courier New', Courier, monospace;
		font-weight: bold;
	}

	.description {
		min-width: 300px;
		font-size: 1.05em;
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