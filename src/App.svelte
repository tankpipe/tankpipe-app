<script>
    import Accounts from './Accounts.svelte'
    import Schedules from './Schedules.svelte'
    import Transactions from './Transactions.svelte'
    import Settings from './Settings.svelte'
    import { afterUpdate } from 'svelte'
    import {page, modes, views} from './page.js'
    import './events'
    import {settings} from './settings'
    import {accounts} from './accounts'
    import {config} from './config'
    import {context} from './context'
    import {initializeContext, updateContext} from './context'
    import EditBooks from './EditBooks.svelte'
    import {onDestroy, onMount} from 'svelte'
    import {listen} from '@tauri-apps/api/event'
    import Dialog from './Dialog.svelte'
    import About from './About.svelte';

    export let curAccount = null
    let dialog
    let initializing = true
    initializeContext()

    export let transactions = []

    let unlistenLoaded
    onMount(async () => {
        unlistenLoaded = await listen('file-loaded', (event) => {
            curAccount = null
            loadConfig()
        })
    })

    onDestroy(async () => {
        unlistenLoaded()
    })

    const loadAccounts = async () => {
        curAccount = null
        let result = await invoke('accounts')
        accounts.set(result)
    };

    const loadSettings = async () => {
        let result = await invoke('settings')
        settings.set(result)
    };

    const loadConfig = async () => {
        let result = await invoke('config')
        config.set(result)
        console.log(result)
        resetMenu()
    };

    const resetMenu = () => {
        updateContext({hasBooks: $config.recent_files.length > 0})
        if ($accounts.length < 1) {
            page.set({view: views.ACCOUNTS, mode: modes.NEW})
        }
    }

    (async () => {
        initializing = true
        await loadSettings()
        await loadAccounts()
        await loadConfig()
        resetMenu()
        initializing = false

    })()

    const listener = async () => {

        listen('about', (event) => {
            console.log(event)
            dialog.showModal()
        })

    }

    listener()

    let dialogShown = false
    let closeIcon = true
    afterUpdate(() => {
        if ( ! $context.hasBooks && dialog && ! dialogShown) {
            closeIcon = false
            dialog.showModal()
            dialogShown = true
        }
    })

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
                        <li on:click={() => page.set({view: views.ACCOUNTS, mode: modes.LIST})} class:menu-selected={$page.view === views.ACCOUNTS}>Accounts</li>
                        {#if $accounts.length > 0 }
                        <li on:click={() => page.set({view: views.TRANSACTIONS, mode: modes.LIST})} class:menu-selected={$page.view === views.TRANSACTIONS}>Transactions</li>
                        <li on:click={() => page.set({view: views.SCHEDULES, mode: modes.LIST})} class:menu-selected={$page.view === views.SCHEDULES}>Schedules</li>
                        {:else}
                        <li class="disabled">Transactions</li>
                        <li class="disabled">Schedules</li>
                        {/if}
                    </ul>
                </div>

            </div>
            <div class="column middle">

                    {#if $page.view === views.TRANSACTIONS}
                    <Transactions bind:this={transactions} bind:curAccount={curAccount}/>
                    {/if}
                    {#if $page.view === views.ACCOUNTS}
                    <Accounts bind:curAccount={curAccount} {loadAccounts}/>
                    {/if}
                    {#if $page.view === views.SCHEDULES}
                    <Schedules/>
                    {/if}
                    {#if $page.view === views.SETTINGS}
                    <Settings />
                    {/if}
                    {#if $page.view === views.BOOKS}
                    <EditBooks />
                    {/if}
                    {#if $page.view === views.ABOUT}
                    <About />
                    {/if}
            </div>
            <Dialog bind:dialog on:close={() => console.log('closed')} closeButton={true} {closeIcon}>
                <About />
            </Dialog>
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
        min-width: 150px;
        background-color: #363636;
        min-height:100%;
        position:relative;
        padding-top: 30px;
    }

    .column.middle {
        margin: 40px 20px 30px 20px;
        min-width: 200px;
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

    :global(.account-heading) {
        text-align: left;
        margin: 0;
    }

    :global(.account-heading select) {
        color: #C0C0C0;
        background-color: #444 !important;
    }


    :global(.form label, .heading, .total) {
        text-align: left;
        font-size: .8em;
        margin-bottom: 3px;
        color: #DDDDDD;
    }

    :global(.form input) {
        background-color: #aaa !important;
    }


    :global(.form input, .form select, .form button) {
        background-color: #aaa;

    }

    :global(.form input:focus, .form select:focus) {
        outline: #fff solid 1px;

    }

    :global(.panel-title) {
        font-size: 0.8em;
        font-weight: 500;
        text-align: left;
        color: #757575;
        margin: 1px 0 7px 0;
    }

    :global(.form-heading) {
        font-size: 1.2em;
        font-weight: 500;
        margin: 0px 0 20px 0;
        text-align: left;
        color: #757575;
        float: left;
    }

    :global(.toolbar) {
        float: right;
        color: #C0C0C0;
    }

    :global(.toolbar-icon) {
        margin-left: 5px;
    }

    :global(.toolbar-icon:hover) {
        color: #F0F0F0;
        cursor: pointer;
    }


    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }

</style>