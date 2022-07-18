`<script>
	import Accounts from './Accounts.svelte';	
	import Transactions from './Transactions.svelte';

	let accounts = []; 
	let curAccount = undefined
	let mode = "ACCOUNTS"


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
					<li on:click={() => setMode("TRANSACTIONS")}>Transactions</li>
					<li on:click={() => setMode("ACCOUNTS")}>Accounts</li>
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
		</div>		
	</div>
</main>

<style>
	.column {
		float:left
	}

	.column.left {
		width: 160px;
		background-color: #404040;
		min-height:100%;
    	position:relative;
	}

	.column.middle {
		margin: 20px;
	}

	.app {
		float:left;
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