<script>
    import EditAccount from "./EditAccount.svelte"
    import Icon from '@iconify/svelte'
    import {page, isEditMode, modes, views} from "../stores/page"
    import {context} from '../stores/context'
    import {accounts} from '../stores/accounts'
    import {onMount} from 'svelte'
    import { _ } from 'svelte-i18n'
    import { invoke } from "@tauri-apps/api/core"
    import { save } from '@tauri-apps/plugin-dialog'
    import { documentDir } from '@tauri-apps/api/path'

    let { curAccount, loadAccounts } = $props()

    let ACCOUNT_TYPES = {
        Asset: $_('accountTypes.assets'),
        Liability: $_('accountTypes.liabilities'),
        Revenue: $_('accountTypes.revenues'),
        Expense: $_('accountTypes.expenses'),
        Equity: $_('accountTypes.equity')
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

    $effect(() => {
        if ($page.mode === modes.EDIT && $page.payload && $page.payload.accountId) {
            curAccount = $accounts.find(account => account.id === $page.payload.accountId)
        }
    })

    const close = () => {
        if ($page.payload && $page.payload.previousView) {
            page.set({view: $page.payload.previousView, mode: modes.LIST, payload: {accountId: curAccount?.id}})
        } else {
            page.set({view: views.ACCOUNTS, mode: modes.LIST})
        }
    }

    const handleAddClick = () => {
        curAccount = null
        page.set({view: views.ACCOUNTS, mode:modes.NEW})
    }

    const selectAccount = (account) => {
        curAccount = account
        page.set({view: views.TRANSACTIONS, mode: modes.LIST, payload: {accountId: curAccount.id}})
    }

    const editAccount = (account) => {
        curAccount = account
        page.set({view: views.ACCOUNTS, mode: modes.EDIT})
        if (curAccount && curAccount.name) {
            console.log($_('accounts.messages.selected', { name: curAccount.name }))
        }
    }

    const checkAccountType = (account) => {
        if (account.account_type !== lastAccountType) {
            lastAccountType = account.account_type
            return true
        }
        return false
    }

    const csvExport = async () => {
        let defaultPath
        await documentDir()
            .then(path => defaultPath = path)
            .catch(e => console.log("error getting document dir: " + e))

        const defaultFileName = `accounts_${new Date().toISOString().split('T')[0]}.csv`

        const selected = await save({
            filters: [{name: 'CSV Files', extensions: ['csv']}],
            defaultPath: `${defaultPath}/${defaultFileName}`,
        })

        if (selected) {
            try {
                await invoke('export_accounts_csv', { path: selected })
                // Success message could be added here if needed
            } catch (err) {
                console.log(err)
                // Error handling could be added here if needed
            }
        }
    }
    
</script>

{#if isEditMode($page)}
<EditAccount {curAccount} {loadAccounts} {close} initialize={accounts.length < 1}/>
{:else if ($context.hasBooks)}
    <div>
        <div class="toolbar toolbar-right">
            <button class="toolbar-icon" onclick="{handleAddClick}" title={$_('accounts.buttons.createNew')}><Icon icon="mdi:plus-box-outline"  width="24"/></button>
            <button class="toolbar-icon" onclick={csvExport} title={$_('accounts.exportCsv')}><Icon icon="mdi:folder-download" width="22"/></button>
        </div>
        <div class="form-heading">{$_('accounts.title')}</div>
    </div>
    <div class="scroller">
    <div class="accounts">
    {#each $accounts as a}
    {#if checkAccountType(a)}
        <div class="account-type">{ACCOUNT_TYPES[a.account_type]}</div>
    {/if}
        <div class="card" onclick={() => selectAccount(a) } onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectAccount(a); } }} tabindex="0" role="button">{a.name}<div class="edit-icon" onclick={(e) => {e.stopPropagation(); editAccount(a)} } onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); editAccount(a); } }} tabindex="0" role="button"><Icon icon="mdi:pencil" /></div></div>
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
        color: var(--color-text-subtle);
        margin: 5px 0px -5px 10px;
        float: left;
        clear: both;
    }

    .edit-icon {
        float: right;
        color: var(--color-surface);
    }

    .card:hover .edit-icon {
        color: var(--color-border);
    }

    .edit-icon:hover {
        color: var(--color-text-muted) !important;
    }

    .card {
        padding: 10px;
        border-radius: 10px;
        color: var(--color-text-strong-2);
        min-width: 300px;
        text-align: left;
    }

    .card:hover {
        cursor: pointer;
        color: var(--color-white);
    }
    
</style>