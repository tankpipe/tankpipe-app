<script>
    import EditAccount from "./EditAccount.svelte"    

    export let curAccount
    export let accounts = []
    export let loadAccounts

    let mode = "ACCOUNTS"
    let editMode = "ADD"

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
    <ul>
    {#each accounts as a}
        <li on:click={() => selectAccount(a) }>{a.name}</li>		
    {/each}	
    </ul>
</div>
{/if}
<style>
    .scroller {
        min-width: 200px;
        float: left;
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

    li, ul {
		list-style-type: none;
		margin-bottom: 1em;
		font-weight: bold;
		color: #C0C0C0;
		padding: 0;
		text-align: left;
	}

	li:hover, ul:hover {
		color: #F0F0F0;
		cursor: pointer;
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