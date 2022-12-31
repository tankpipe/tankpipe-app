<script>
	import EditSchedule from './EditSchedule.svelte'
	import Icon from '@iconify/svelte'
    import { page, isEditMode, views, modes } from './page';

	export let accounts = []
	let curSchedule

	const close = () => {
        console.log("close")
        page.set({view: views.SCHEDULES, mode: modes.LIST})
		loadSchedules()
    }

	$: {
		loadSchedules()
    }

	const selectSchedule = (transaction) => {
		curSchedule = transaction
		page.set({view: views.SCHEDULES, mode: modes.EDIT})
	}

	let schedules = [];
	const loadSchedules = async () => {
		console.log("loadSchedules");
   		schedules = await invoke('schedules');
	};

	const formatter = new Intl.NumberFormat('en-AU', {
		minimumFractionDigits: 2,
		maximumFractionDigits: 2,
	});

	const matchAccount = (accountId) =>  {
        if (!accountId) return null
        let match = accounts.filter(a => a.id == accountId)
        return match.length > 0 ? match[0] : null
    }

	const getAccountName = (accountId) => {
		const account = matchAccount(accountId)
		return account ? account.name : "None"
	}

	const handleAddClick = () => {
		page.set({view: views.SCHEDULES, mode: modes.NEW})
	}

</script>

<div class="account-heading">
	{#if !isEditMode($page)}
	<div class="toolbar"><div class="toolbar-icon" on:click={handleAddClick} title="Create a new schedule"><Icon icon="mdi:plus-box-outline"  width="24"/></div></div>
	<div class="form-heading">Schedules</div>
	{/if}
</div>
{#if isEditMode($page)}
<EditSchedule {close} {accounts} {curSchedule} {loadSchedules}/>
{/if}
{#if !isEditMode($page)}
<div class="scroller">
	{#if schedules.length < 1}
	<div class="message">No schedules</div>
	{/if}
		{#each schedules as s}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div class="card" on:click={() => selectSchedule(s)}>
				<div class="row">
					<div class="widget">
						<div class="description">{s.name}</div>
					</div>
				</div>
				<hr/>
				<div class="row">
					<div class="widget schedule-entries">
					<table>
						{#each s.entries as e}
						<tr>
							<td>{getAccountName(e.account_id)}</td>
							<td>{#if e.transaction_type == "Debit"} <div class="money">{formatter.format(e.amount)}</div>{/if}</td>
							<td>{#if e.transaction_type == "Credit"} <div class="money">{formatter.format(e.amount)}</div>{/if}</td>
						</tr>
						{/each}
					</table>
				</div>
				</div>
				<div class="row">
					<div class="widget">
						<div class="label">Last</div>
						<div class="account">{s.last_date == "null"?"-":s.last_date}</div>
					</div>
				</div>
			</div>
		{/each}
</div>
{/if}

<style>
	.widget {
		display: inline-block;
		text-align: left;
		margin: 10px 10px;
		color: #F0F0F0;
		vertical-align: top;
	}

	.schedule-entries {
		font-size: .9em;
		color: #C0C0C0;
	}

	.row {
		display: block;
		text-align: left;
	}

	.label {
		font-size: .8em;
		color: #aaa !important;
		margin-bottom: 5px;
	}

	.scroller {
		clear: both;
	}

	.card {
		float: left;
		clear: both;
		margin: 10px;
		background-color: #524e4e;
		padding: 5px;
		border-radius: 10px;
	}

	.card:hover {
		cursor: pointer;
	}

	.money {
		text-align: right !important;
		min-width: 100px;
	}

	.description {
		min-width: 500px;
		white-space: nowrap;
		font-weight: bold;
	}

	.account {
		min-width: 200px;
		white-space: nowrap;
	}

	hr {
		border: 1px solid #444;
		margin: 0 -5px;
	}

	.message {
		color: #EFEFEF;
		margin-bottom: 20px;
		text-align: left;
		background-color: #303030;
		padding:10px;
		border-radius: 10px;
	}

	.toolbar {
		float: right;
		color: #C0C0C0;
		margin-bottom: 10px;
	}

	.toolbar-icon:hover{
		color: #F0F0F0;
		cursor: pointer;
	}

</style>