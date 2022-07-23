<script>
	import EditSchedule from './EditSchedule.svelte'
	
	export let accounts = []
	let mode = "SCHEDULES"
	let editMode = "ADD"
	let curSchedule

	const close = () => {
        console.log("close")
        mode = "SCHEDULES";
		loadSchedules()        
    }
	
	$: {
		loadSchedules()
    }

	const selectSchedule = (transaction) => {
		curSchedule = transaction
		editMode = "EDIT"
		mode = "EDIT"
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
		editMode = "ADD"
		mode = "EDIT"		
	}
	
</script>

<div class="account-heading">
	{#if mode === "SCHEDULES"}
	<div class="toolbar"><i class="gg-add-r" on:click={handleAddClick}></i></div>
	{/if}
</div>
{#if mode === "EDIT"}
<EditSchedule {close} {accounts} {editMode} {curSchedule} />
{/if}
{#if mode === "SCHEDULES"}
<div class="scroller">
		{#each schedules as t}
			<div class="card" on:click={() => selectSchedule(t)}>
				<div class="row">
					<div class="widget">
						<div class="description">{t.name}</div>
					</div>
					<div class="widget">
						<div class="money">{formatter.format(t.amount)}</div>
					</div>	
				</div>		
				<div class="row">
					<div class="widget">
						<div class="label">Debit</div>
						<div class="account">{getAccountName(t.dr_account_id)}</div>
					</div>
					<div class="widget">
						<div class="label">Credit</div>
						<div class="account">{getAccountName(t.cr_account_id)}</div>
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
	
	.date {
		min-width: 100px;
	}

	.account {
		min-width: 200px;
		white-space: nowrap;		
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