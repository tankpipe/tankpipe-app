<script>
	import Accounts from './Accounts.svelte';	
	import Schedules from './Schedules.svelte';
	import Transactions from './Transactions.svelte';

	let accounts = []; 
	let curAccount = undefined
	let mode = "SCHEDULES"


	const loadAccounts = async () => {
   		accounts = await invoke('accounts');
		if (accounts.length > 0) {			
			curAccount = accounts[0];			
		}	
	};
	loadAccounts();

	const setMode = (newMode) => {
		mode = newMode;
		console.log(mode)
	}
	
</script>

<main>	
	<div class="app">
		<div class="column left">
			<div class="menu-left"> 
				<ul>
					<li on:click={() => setMode("TRANSACTIONS")} class:menu-selected={mode=="TRANSACTIONS"}>Transactions</li>
					<li on:click={() => setMode("ACCOUNTS")} class:menu-selected={mode=="ACCOUNTS"}>Accounts</li>
					<li on:click={() => setMode("SCHEDULES")} class:menu-selected={mode=="SCHEDULES"}>Schedules</li>
				</ul>	
			</div>
		</div>		
		<div class="column middle">			
			{#if mode=="TRANSACTIONS"}
			<Transactions {curAccount} {accounts}/>
			{/if}
			{#if mode=="ACCOUNTS"}
			<Accounts {curAccount} {accounts} {loadAccounts}/>
			{/if}
			{#if mode=="SCHEDULES"}			
			<Schedules {accounts}/>			
			{/if}
		</div>		
	</div>
</main>

<style>
	.column {
		float:left
	}

	.column.left {
		width: 160px;
		background-color: #363636;
		min-height:100%;
    	position:relative;
		padding-top: 30px;
	}

	.column.middle {
		margin: 20px;
		overflow: scroll;
		padding-top: 20px;
		height: 100%;
	}

	.app {
		background-color: #444;
		height: 100%;
	}

	.menu-left {
		margin: 20px 20px;
	}

	.menu-left li, .menu-left ul {
		list-style-type: none;
		margin-bottom: 1em;
		font-weight: bold;
		color: #C0C0C0;
		padding: 0;
		text-align: left;
	}

	.menu-left li:hover, .menu-left ul:hover {
		color: #F0F0F0;
		cursor: pointer;
	}

	.menu-selected {
		color: #F0F0F0 !important;
	}

	main {
		text-align: center;
		padding: 0;
		max-width: 240px;
		margin: 0;		
		height: 100%;
	}


	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}

</style>