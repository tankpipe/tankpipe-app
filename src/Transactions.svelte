<script>
	import EditTransaction from './EditTransaction.svelte'
    import Select from './Select.svelte'
	import Icon from '@iconify/svelte'
	import { open } from '@tauri-apps/api/dialog'
	import { documentDir } from '@tauri-apps/api/path'
	import { Errors } from './errors'
    import { page, modes, views, isEditMode } from './page'
    import { settings } from './settings'
	import { accounts } from './accounts'

	export let curAccount

	let curTransaction
	let errors = new Errors()
	let msg = ""
	let previousAccount

	const close = () => {
        console.log("close")
		page.set({view: views.TRANSACTIONS, mode: modes.LIST})
		if (curAccount) loadTransactions();
    }

	$: {
		if (curAccount && curAccount !== previousAccount) {
			transactions = []
			loadTransactions()
			previousAccount = curAccount
		}
    }

	const selectTransaction = (transaction) => {
		curTransaction = transaction
		page.set({view: views.TRANSACTIONS, mode: modes.EDIT})
	}

	let transactions = []
	export const loadTransactions = async () => {
		console.log("loadTransactions: " + curAccount.id);
   		transactions = await invoke('transactions', {accountId: curAccount.id})
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

	const getEntry = (transaction) => {
		return transaction.entries.find(e => e.account_id == curAccount.id)
	}

	const handleAddClick = () => {
		page.set({view: views.TRANSACTIONS, mode: modes.NEW})
	}

	const openFile = async () => {
		let appDataDirPath
		await documentDir()
				.then(r => appDataDirPath = r)
				.catch(e => console.log("error : " + e))
		const selected = await open({
			directory: false,
			multiple: false,
			filters: [{name: '*', extensions: ['csv']}],
			defaultPath: appDataDirPath,
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

	const predicted = (t) => t.status == 'Predicted' ? 'predicted' : '';


</script>

<div class="account-heading">
	{#if !isEditMode($page)}
	<Select bind:item={curAccount} items={$accounts} none={settings.require_double_entry} flat={true}/>
	<div class="toolbar">
		{#if curAccount}
		<div class="toolbar-icon import-icon" on:click={openFile} title="Import transactions"><Icon icon="mdi:application-import" width="22"/></div>
		{/if}
		<div class="toolbar-icon" on:click="{handleAddClick(curAccount)}" title="Add a transaction"><Icon icon="mdi:plus-box-outline"  width="24"/></div>
	</div>
	{/if}
</div>
{#if isEditMode($page)}
<EditTransaction {close} {curTransaction}/>
{/if}
{#if !isEditMode($page)}
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
			{@const e =  getEntry(t)}
			{#if e}
			<tr on:click={() => selectTransaction(e)}><!--{t.id}-->
				<td class={predicted(e)}>{e.date}</td>
				<td class={predicted(e)} title="{e.description}"><div class="description">{e.description}</div>
					{#each t.entries as en}
						{#if en.account_id != curAccount.id}
						<div class="description tiny">{$accounts.find(a => a.id == en.account_id).name}</div>
						{/if}
					{/each}
				</td>
				<td class="{predicted(e)} money">{getDebitAmount(e, curAccount)}</td>
				<td class="{predicted(e)} money">{getCreditAmount(e, curAccount)}</td>
				<td class="{predicted(e)} money">{getBalance(e)}</td>
			</tr>
			{/if}
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
		font-size: 0.9em;
	}

	.predicted {
		color: #878787;
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

	tr:hover td .tiny{
		cursor: pointer;
		color: #C0C0C0;
	}

	.money {
		text-align: right !important;
		min-width: 92px;
		font-family: 'Courier New', Courier, monospace;
		font-weight: bold;
	}

	.description {
		min-width: 350px;
		max-width: 33vw;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.tiny {
		font-size: 0.5em;
		color: #878787;
		margin: 3px 0 -5px 0;
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
		margin: 5px 0 20px 0;
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