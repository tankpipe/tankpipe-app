<script>
	import EditTransaction from './EditTransaction.svelte'
    import Select from './Select.svelte'
	import { open } from '@tauri-apps/api/dialog';
	import { appDir } from '@tauri-apps/api/path';
	import { Errors } from './errors';

	export let curAccount
	export let accounts = []
	export let settings
	let mode = "TRANSACTIONS"
	let editMode = "ADD"
	let curTransaction
	let errors = new Errors()
	let msg = ""

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

	const openFile = async () => {
		const selected = await open({
			directory: false,
			multiple: false,
			filters: [{name: '*', extensions: ['csv']}],
			defaultPath: await appDir(),
		});

		if(selected) {
			console.log(selected)
			loadCsv(selected, curAccount)
		}
	}

	function loaded(result) {
		console.log(result)
		loadTransactions()
    }

    function rejected(result) {
		console.log(result)
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }
    const loadCsv = async (path, account) => {
        console.log(path)
   		await invoke('load_csv', {path: path, account: account}).then(loaded, rejected)
	};

</script>


<div class="account-heading">
	{#if mode === "TRANSACTIONS" && curAccount}
	<Select bind:item={curAccount} items={accounts} none={settings.require_double_entry}/>
	<div class="toolbar">
		<div class="toolbar-icon import-icon"><i class="gg-import" on:click={openFile}></i></div>
		<div class="toolbar-icon"><i class="gg-add-r" on:click="{handleAddClick(curAccount)}"></i></div>
	</div>
	{/if}
</div>
{#if mode === "EDIT"}
<EditTransaction {close} {accounts} {editMode} {curTransaction} {settings}/>
{/if}
{#if mode === "TRANSACTIONS"}
<div class="widget errors">
	{#each errors.getErrorMessages() as e}
	<div class="error-msg">{e}</div>
	{/each}
	{#if msg}
	<div class="success-msg">{msg}</div>
	{/if}
</div>
<div class="scroller">
	{#if transactions.length > 0}
	<table>
		<tr><th>Date</th><th>Description</th><th>Debit</th><th>Credit</th><th>Balance</th></tr>
		{#each transactions as t}
			<tr on:click={() => selectTransaction(t)}><!--{t.id}--><td>{t.date}</td><td class="description">{t.description}</td><td class="money">{getDebitAmount(t, curAccount)}</td><td class="money">{getCreditAmount(t, curAccount)}</td><td class="money">{getBalance(t)}</td></tr>
		{/each}
	</table>
	{/if}
	{#if transactions.length < 1}
	<div class="message">No transactions</div>
	{/if}
</div>
{/if}

<style>
    .scroller{
		height: 100%;
		width: 100%;
		overflow: scroll;
	}
	td {
		text-align: left;
		overflow: hidden;
		line-height: 1em;
		color: #444;
		background-color: #f0f0f0;
		padding: 8px;
		white-space: nowrap;
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
		margin-left: 10px;
		display: flex;
	}

	.toolbar i:hover{
		color: #F0F0F0;
		border-color: #f0f0f0;
	}

	.toolbar-icon {
		margin-left: 5px;
	}

	.import-icon {
		margin-top: 7px
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
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
        max-width: 350px;
    }

    .success-msg {
        color: green;
        text-align: left;
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

	.gg-import {
		box-sizing: border-box;
		position: relative;
		display: block;
		transform: scale(var(--ggs,1));
		width: 18px;
		height: 14px;
		border: 2px solid;
		border-top: 0;
		box-shadow:
			-6px -8px 0 -6px,
			6px -8px 0 -6px
	}

	.gg-import::after,
	.gg-import::before {
		content: "";
		display: block;
		box-sizing: border-box;
		position: absolute
	}

	.gg-import::before {
		background: currentColor;
		width: 2px;
		height: 14px;
		right: 6px;
		bottom: 5px
	}

	.gg-import::after {
		width: 6px;
		height: 6px;
		border-right: 2px solid;
		border-bottom: 2px solid;
		right: 4px;
		bottom: 4px;
		transform: rotate(45deg)
	}
</style>