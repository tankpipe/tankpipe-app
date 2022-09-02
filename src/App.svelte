<script>
	import Accounts from './Accounts.svelte';
	import Schedules from './Schedules.svelte';
	import Transactions from './Transactions.svelte';
	import Settings from './Settings.svelte';

	let accounts
	export let curAccount = undefined
	let mode = "ACCOUNTS"
	let initializing = true

	export let transactions = []
	let settings

	const loadAccounts = async () => {
   		accounts = await invoke('accounts');
		if (accounts.length > 0) {
			curAccount = accounts[0];
		}
	};

	const loadSettings = async () => {
   		settings = await invoke('settings');
	};

	const initialize = async () => {
		initializing = true
		await loadSettings()
		await loadAccounts()
		if (accounts.length < 1) {
			mode = "ACCOUNTS"
		}
		initializing = false
	}
	initialize()

	const selectMenu = (newMode) => {
		mode = newMode;
	}

</script>

<main>
	<div class="app">
		{#if initializing}
			<div class="loading">Loading...</div>
		{/if}

		{#if !initializing}
			<div class="column left">
				<div class="menu-left">
					<ul>
						{#if accounts.length < 1 }
						<li class="disabled">Transactions</li>
						{/if}
						<li on:click={() => selectMenu("ACCOUNTS")} class:menu-selected={mode=="ACCOUNTS"}>Accounts</li>
						{#if accounts.length > 0 }
						<li on:click={() => selectMenu("TRANSACTIONS")} class:menu-selected={mode=="TRANSACTIONS"}>Transactions</li>
						{/if}
						{#if accounts.length < 1 }
						<li class="disabled">Schedules</li>
						{/if}
						{#if accounts.length > 0 }
						<li on:click={() => selectMenu("SCHEDULES")} class:menu-selected={mode=="SCHEDULES"}>Schedules</li>
						{/if}
						<li on:click={() => selectMenu("SETTINGS")} class:menu-selected={mode=="SETTINGS"}>Settings</li>
					</ul>
				</div>
			</div>
			<div class="column middle">

					{#if mode=="TRANSACTIONS"}
					<Transactions bind:this={transactions} {curAccount} {accounts} {settings}/>
					{/if}
					{#if mode=="ACCOUNTS"}
					<Accounts bind:curAccount={curAccount} {accounts} {loadAccounts} {selectMenu}/>
					{/if}
					{#if mode=="SCHEDULES"}
					<Schedules {accounts}/>
					{/if}
					{#if mode=="SETTINGS"}
					<Settings bind:settings={settings}/>
					{/if}

			</div>
		{/if}
	</div>
</main>

<style>
	.loading {
		margin: 50px 50px;
		color: #C0C0C0;
	}

	.column {
		float:left;
		-webkit-user-select: none; /* Chrome/Safari */
		-moz-user-select: none; /* Firefox */
		-ms-user-select: none; /* IE10+ */
		-o-user-select: none;
		user-select: none;
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
		padding-top: 20px;
	}

	.app {
		background-color: #444;
		height: 100%;
		display: flex;
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

	.disabled {
		color: #555555 !important;
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