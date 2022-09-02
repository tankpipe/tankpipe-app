<script>
	import EditTransaction from './EditTransaction.svelte'
    import Select from './Select.svelte'
	import Icon from '@iconify/svelte'
	import { open } from '@tauri-apps/api/dialog'
	import { appDir } from '@tauri-apps/api/path'
	import { Errors } from './errors'

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
	<Select bind:item={curAccount} items={accounts} none={settings.require_double_entry} flat={true}/>
	<div class="toolbar">
		<div class="toolbar-icon import-icon" on:click={openFile} title="Import transactions"><Icon icon="mdi:application-import" width="22"/></div>
		<div class="toolbar-icon" on:click="{handleAddClick(curAccount)}" title="Add a transaction"><Icon icon="mdi:plus-box-outline"  width="24"/></div>
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
		<tr><th class="justify-left">Date</th><th class="justify-left">Description</th><th>Debit</th><th>Credit</th><th>Balance</th></tr>
		{#each transactions as t}
			<tr on:click={() => selectTransaction(t)}><!--{t.id}-->
				<td>{t.date}</td>
				<td title="{t.description}"><div class="description">{t.description}</div></td>
				<td class="money">{getDebitAmount(t, curAccount)}</td>
				<td class="money">{getCreditAmount(t, curAccount)}</td>
				<td class="money">{getBalance(t)}</td>
			</tr>
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

	table {
		padding-right: 10px;
	}

	td {
		text-align: left;
		overflow: hidden;
		line-height: 1em;
		color: #ccc;
		background-color: #393939;
		padding: 8px;
		white-space: nowrap;
		font-size: 0.95em;
	}

	th {
		color:#666666;
		background-color: #444;
		font-weight: 400;
		font-size: .8em;
	}
	.justify-left {
		text-align: left;
		padding-left: 10px;
	}

	tr:hover td {
		cursor: pointer;
		color: #FFF;
	}

	.money {
		text-align: right !important;
		min-width: 100px;
		font-family: 'Courier New', Courier, monospace;
		font-weight: bold;
	}

	.description {
		min-width: 350px;
		max-width: 33vw;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.account-heading {
		text-align: left;
		margin-bottom: 5px;
	}

	:global(.account-heading select) {
		color: #C0C0C0;
		background-color: #444 !important;
	}

	.toolbar {
		float: right;
		color: #C0C0C0;
		margin-left: 10px;
		display: flex;
		padding-right: 9px;
	}

	.toolbar-icon {
		margin-left: 5px;
	}

	.toolbar-icon:hover{
		color: #F0F0F0;
		cursor: pointer;
	}

	.import-icon {
		margin-top: 1px
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

	@media (min-width: 1010px) {
		.description {
			max-width: calc(70vw - 350px);
		}
	}

</style>