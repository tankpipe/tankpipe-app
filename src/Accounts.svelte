// In Accounts.svelte:
<script>
    import EditAccount from "./EditAccount.svelte"
    import Icon from '@iconify/svelte'
    import {open} from '@tauri-apps/plugin-dialog'
    import {page, isEditMode, modes, views} from "./page"
    import {config} from './config'
    import {accounts} from './accounts'
    import { context } from "./context"
    import EditBooks from "./EditBooks.svelte"
    import { invoke } from '@tauri-apps/api/core'
    import {onMount} from 'svelte'
    import { _ } from 'svelte-i18n'

    export let curAccount
    export let loadAccounts

    let ACCOUNT_TYPES = {
        Asset: $_('accounts.accountTypes.assets'),
        Liability: $_('accounts.accountTypes.liabilities'),
        Revenue: $_('accounts.accountTypes.revenues'),
        Expense: $_('accounts.accountTypes.expenses'),
        Equity: $_('accounts.accountTypes.equity')
    }
    let lastAccountType

    onMount(() => {
        lastAccountType = ""
        if ($accounts) {
            if ($accounts.length < 1) {
                page.set({view: views.ACCOUNTS, mode: modes.NEW})
            }
        }
    })

    const close = () => {
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

    const handleAddClick = () => {
        curAccount = null
        page.set({view: views.ACCOUNTS, mode:modes.NEW})
    }

    const selectAccount = (account) => {
        curAccount = account
        page.set({view: views.TRANSACTIONS, mode: modes.LIST})
    }

    const editAccount = (account) => {
        curAccount = account
        page.set({view: views.ACCOUNTS, mode: modes.EDIT})
        console.log($_('accounts.messages.selected', { name: curAccount.name }))
    }

    const checkAccountType = (account) => {
        if (account.account_type !== lastAccountType) {
            lastAccountType = account.account_type
            return true
        }
        return false
    }

    const newFile = async () => {
        curAccount = null
        page.set({view: views.BOOKS, mode: modes.NEW})
    };

    const openFile = async () => {
        const selected = await open({
            directory: false,
            multiple: false,
            filters: [{name: '*', extensions: ['json']}],
            defaultPath: $config.data_dir,
        });

        if(selected) {
            console.log(selected)
            loadFile(selected)
        }
    }

    const loadFile = async (path) => {
        curAccount = null
           await invoke('load_file', {path: path}).then(loadFileSuccess, loadFileFailure)
    };

    function loadFileSuccess(result) {
        console.log(result)
        loadConfig()
        accounts.set(result)
    }

    function loadFileFailure(result) {
        console.log(result)
        errors = new Errors()
        errors.addError("all", $_('accounts.errors.loadFailure', { result }))
    }

    const loadConfig = async () => {
        let result = await invoke('config')
        config.set(result)
    };

</script>

{#if ! $context.hasBooks}
<EditBooks />
{:else if isEditMode($page)}
<EditAccount {curAccount} {loadAccounts} {close} initialize={accounts.length < 1}/>
{:else}

    {#if !isEditMode($page)}
    <div class="toolbar">
        <div class="toolbar-icon" on:click="{handleAddClick}" title={$_('accounts.actions.createNew')}><Icon icon="mdi:plus-box-outline"  width="24"/></div>
    </div>
    {/if}
    <div class="form-heading">{$_('accounts.title')}</div>
    <div class="scroller">
    <div class="accounts">
    {#each $accounts as a}
    {#if checkAccountType(a)}
        <div class="account-type">{ACCOUNT_TYPES[a.account_type]}</div>
    {/if}
        <div class="card" on:click={() => selectAccount(a) }>{a.name}<div class="edit-icon" on:click|stopPropagation={() => editAccount(a) }><Icon icon="mdi:pencil" /></div></div>
    {/each}
    </div>
</div>
{/if}
<style>
    .scroller{
        height: 100%;
        width: 100%;
        overflow: scroll;
        margin-top: 15px;
    }

    .accounts {
        margin-right: 20px;
    }

    .account-type {
        font-size: 0.8em;
        font-weight: 500;
        color: #757575;
        margin: 5px 0px -5px 10px;
        float: left;
        clear: both;
    }

    .edit-icon {
        float: right;
        color: #524e4e;
    }

    .card:hover .edit-icon {
        color: #666;
    }

    .edit-icon:hover {
        color: #C0C0C0 !important;
    }

    .card {
        float: left;
        clear: both;
        margin: 10px;
        background-color: #524e4e;
        padding: 10px;
        border-radius: 10px;
        color: #E0E0E0;
        min-width: 300px;
        text-align: left;
    }

    .card:hover {
        cursor: pointer;
        color: #FFF;
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

</style>