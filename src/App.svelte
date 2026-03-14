<script>
    import Accounts from './accounts/Accounts.svelte'
    import Schedules from './schedules/Schedules.svelte'
    import Modifiers from './schedules/Modifiers.svelte'
    import Transactions from './transactions/Transactions.svelte'
    import Settings from './views/Settings.svelte'
    import {page, modes, views} from './stores/page.js'
    import {initialiseBooks, initialiseFailed} from'./events'
    import {accounts, updateAccounts} from './stores/accounts'
    import {initializeContext, context} from './stores/context'
    import {config} from './stores/config'
    import EditBooks from './views/EditBooks.svelte'
    import {onDestroy, onMount} from 'svelte'
    import {listen} from '@tauri-apps/api/event'
    import Dialog from './components/Dialog.svelte'
    import About from './views/About.svelte'
    import { invoke } from '@tauri-apps/api/core'
    import NetAssets from './views/NetAssets.svelte'
    import { _, waitLocale, isLoading } from 'svelte-i18n'
    import './utils/i18n'
    import ErrorMsg from './components/ErrorMsg.svelte'
    import { updateConfig } from './stores/config'
    import { settings } from './stores/settings'
    import { get } from 'svelte/store'
    import Icon from '@iconify/svelte'

    let curAccount = $state(null)
    let dialog = $state(null)
    let supportedVersion = $state(false);    
    
    initializeContext()

    let transactions = $state([])

    let unlistenLoaded
    let themeMediaQuery
    let removeThemeListener = null
    onMount(async () => {
        unlistenLoaded = await listen('file-loaded', (event) => {
            curAccount = null
        })         
    })

    onDestroy(async () => {
        unlistenLoaded()
        if (removeThemeListener) removeThemeListener()
    })

    const initialise = async () => {        
        await invoke('initialise').then(initialiseBooks, initialiseFailed)
    };

    const loadAccounts = async () => {
        curAccount = null
        let result = await invoke('accounts')
        updateAccounts(result)
    };

    (async () => {
        supportedVersion = (typeof HTMLDialogElement === 'function')

        if (supportedVersion) {
             await waitLocale()
             //await initialise()
             await invoke('load_config').then(loadConfigSuccess, loadConfigFailed)
             if ($config && ($config.current_books_id || $config.current_file)) {
                 initialise()
             } else {
                 console.log('No books history found')                 
                 page.set({view: views.BOOKS, mode: modes.NEW}) 
             }
        }

    })()

    const loadConfigSuccess = (result) => {
        console.log(result)
        updateConfig(result)        
    }

    const loadConfigFailed = (error) => {
        console.log(error)
    }

    const listener = async () => {

        listen('about', (event) => {
            console.log(event)
            dialog.showModal()
        })

    }

    listener()

    let closeIcon = true

    const getSystemTheme = () => {
        if (typeof window === 'undefined' || !window.matchMedia) return 'dark'
        return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
    }

    const applyTheme = (theme) => {
        if (typeof document === 'undefined') return
        const resolved = theme === 'system' ? getSystemTheme() : theme
        document.documentElement.dataset.theme = resolved
    }

    const toggleTheme = async () => {
        let currentTheme = $config?.theme || 'System'
        if (currentTheme === 'System') {
            currentTheme = getSystemTheme()
        }
        const newTheme = (currentTheme === 'Light' || currentTheme === 'light') ? 'Dark' : 'Light'
        
        const configSettings = {
            display_date_format: $config.display_date_format,
            import_date_format: $config.import_date_format,
            theme: newTheme
        }
        await updateConfig(configSettings)
        invoke('update_config', {configSettings: configSettings})
    }

    $effect(() => {
        const theme = $config?.theme?.toLowerCase() || 'system'
        applyTheme(theme)
    })

    onMount(() => {
        if (typeof window === 'undefined' || !window.matchMedia) return
        themeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
        const handler = () => {
            const theme = get(config)?.theme?.toLowerCase() || 'system'
            if (theme === 'system') applyTheme(theme)
        }
        if (themeMediaQuery.addEventListener) {
            themeMediaQuery.addEventListener('change', handler)
            removeThemeListener = () => themeMediaQuery.removeEventListener('change', handler)
        } else {
            themeMediaQuery.addListener(handler)
            removeThemeListener = () => themeMediaQuery.removeListener(handler)
        }
        handler()
    })

</script>

<main>
    <div class="app">                
        {#if !supportedVersion}
        <div class="column middle">
            <div class="loading">Unfortunately the webview version on this computer is not supported for running Tankpipe. Updating your OS to a more recent version may help.</div>
            <div class="loading">User Agent: {window.navigator.userAgent}</div>
        </div>
        {/if}
        {#if supportedVersion && !$isLoading}
            <div class="column left">
                <div class="menu-left">
                    <ul>
                        {#if $context.hasBooks}
                        <li><button class="og-button" type="button" onclick={() => page.set({view: views.ACCOUNTS, mode: modes.LIST})} class:menu-selected={$page.view === views.ACCOUNTS}>{$_('app.accounts')}</button></li>
                        {:else}
                        <li class="disabled">{$_('app.accounts')}</li>
                        {/if}
                        {#if $accounts.length > 0 }
                        <li><button class="og-button" type="button" onclick={() => page.set({view: views.TRANSACTIONS, mode: modes.LIST})} class:menu-selected={$page.view === views.TRANSACTIONS}>{$_('app.transactions')}</button></li>
                        <li><button class="og-button" type="button" onclick={() => page.set({view: views.JOURNAL, mode: modes.LIST})} class:menu-selected={$page.view === views.JOURNAL}>{$_('app.journal')}</button></li>
                        <li><button class="og-button" type="button" onclick={() => page.set({view: views.SCHEDULES, mode: modes.LIST})} class:menu-selected={$page.view === views.SCHEDULES}>{$_('app.schedules')}</button></li>
                        <li><button class="og-button" type="button" onclick={() => page.set({view: views.MODIFIERS, mode: modes.LIST})} class:menu-selected={$page.view === views.MODIFIERS}>{$_('app.modifiers')}</button></li>
                        <li><button class="og-button" type="button" onclick={() => page.set({view: views.NET_ASSETS, mode: modes.LIST})} class:menu-selected={$page.view === views.NET_ASSETS}>{$_('app.net_assets')}</button></li>
                        {:else}
                        <li class="disabled">{$_('app.transactions')}</li>
                        <li class="disabled">{$_('app.schedules')}</li>
                        <li class="disabled">{$_('app.modifiers')}</li>                        
                        {/if}                    
                    </ul>
                </div>
                <div class="theme-toggle">
                    <button class="toolbar-icon" onclick={toggleTheme} title="{$_('app.toggleTheme')}">
                        {#if $config?.theme === 'Light'}
                            <Icon icon="mdi:weather-sunny" width="20"/>
                        {:else}
                            <Icon icon="mdi:weather-night" width="20"/>
                        {/if}
                    </button>
                </div>

            </div>
            <div class="column middle">
                <ErrorMsg/>
                {#if $page.view === views.TRANSACTIONS}
                <Transactions bind:this={transactions} bind:curAccount={curAccount} journalMode={false}/>
                {:else if $page.view === views.JOURNAL}
                <Transactions bind:this={transactions} bind:curAccount={curAccount} journalMode={true}/>
                {:else if $page.view === views.ACCOUNTS}
                <Accounts bind:curAccount={curAccount} {loadAccounts}/>
                {:else if $page.view === views.SCHEDULES}
                <Schedules/>
                {:else if $page.view === views.MODIFIERS}
                <Modifiers/>
                {:else if $page.view === views.NET_ASSETS}
                <NetAssets bind:curAccount={curAccount} {loadAccounts}/>
                {:else if $page.view === views.SETTINGS}
                <Settings />
                {:else if $page.view === views.BOOKS}
                <EditBooks />
                {:else if $page.view === views.ABOUT}
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
        color: var(--color-text-muted);
        text-align: left;
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
        background-color: var(--color-bg-alt);
        min-height:100%;
        position:relative;
        padding-top: 30px;
    }

    .column.middle {
        display: flex;
        flex-direction: column;
        margin: 40px 20px 0px 20px;
        min-width: 200px;
    }
    
    .app {
        background-color: var(--color-bg);
        height: 100%;
        display: flex;
    }

    .menu-left {
        margin: 20px 20px;
    }

    .menu-left li, .menu-left ul {
        list-style-type: none;
        margin-bottom: 1em;
        padding: 0;
        text-align: left;
    }

    .menu-left button {
        background: none !important;
        border: none;
        font-weight: bold;
        color: var(--color-text-muted);
        padding: 0;
        text-align: left;
        cursor: pointer;
        font-family: inherit;
        font-size: inherit;
        height: 0 !important;
        padding-left: 0 !important;
    }

    .menu-left button:hover {
        color: var(--color-text-strong);
    }

    .menu-selected {
        color: var(--color-text-strong) !important;
    }

    .disabled {
        color: var(--color-text-disabled) !important;
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
        color: var(--color-text-muted);
        background-color: var(--color-bg) !important;
    }

    :global(.form label, .heading, .total) {
        text-align: left;
        font-size: .8em;
        margin-bottom: 3px;
        color: var(--color-text);
    }

    :global(.form input) {
        background-color: var(--color-input-bg) !important;
    }

    :global(.form input, .form select) {
        background-color: var(--color-input-bg);

    }

    :global(.form input:focus, .form select:focus) {
        outline: var(--color-focus) solid 1px;

    }

    :global(.panel-title) {
        font-size: 0.8em;
        font-weight: 500;
        text-align: left;
        color: var(--color-text-subtle);
        margin: 1px 0 7px 0;
    }

    :global(.form-heading) {
        font-size: 1.2em;
        font-weight: 500;
        margin: 0px 0 20px 0;
        text-align: left;
        color: var(--color-text-subtle);
        float: left;
    }
   
    :global(.selectable-text) {
        -webkit-user-select: all; /* Chrome/Safari */
        -moz-user-select: all; /* Firefox */
        -ms-user-select: all; /* IE10+ */
        -o-user-select: all;
        user-select: all;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }

    .theme-toggle {
        position: absolute;
        bottom: 20px;
    }

    .theme-toggle .toolbar-icon {
        margin-left: 5px;
        color: var(--color-text-muted);
    }

    .theme-toggle .toolbar-icon:hover {
        color: var(--color-text-strong);
        cursor: pointer;
    }

    .theme-toggle button {
        background-color: transparent;
        border: none;
    }

</style>
