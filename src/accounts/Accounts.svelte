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
    import { config, formatDate } from "../stores/config"

    let { curAccount, loadAccounts } = $props()

    let showDetails = $state(false)

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
            } catch (err) {
                console.log(err)
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
            <button class="{showDetails ? 'toolbar-icon-on' : 'toolbar-icon'}" onclick="{() => showDetails = !showDetails}" title={showDetails ? $_('accounts.buttons.hideDetails') : $_('accounts.buttons.showDetails')}><Icon icon="mdi:eye-outline"  width="24"/></button>
            <button class="toolbar-icon" onclick={csvExport} title={$_('accounts.exportCsv')}><Icon icon="mdi:folder-download" width="22"/></button>
        </div>
        <div class="form-heading">{$_('accounts.title')}</div>
        <div class="small-sub-heading" title={$config.current_file?.path || ''}>{$config.current_file?.name || ''}</div>
    </div>
    <div class="scroller">
    <div class="accounts">
    {#each $accounts as a}
    {#if checkAccountType(a)}
        <div class="account-type">{ACCOUNT_TYPES[a.account_type]}</div>
    {/if}
        <div class="card" onclick={() => selectAccount(a) } onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectAccount(a); } }} tabindex="0" role="button">
            {a.name}
            <div class="edit-icon" onclick={(e) => {e.stopPropagation(); editAccount(a)} } onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); editAccount(a); } }} tabindex="0" role="button"><Icon icon="mdi:pencil" /></div>
            {#if showDetails}
            <hr/>
            {#if a.reconciliation_info}
            <div class="row">
                <div class="card-bottom">
                    <div class="label card-label label-column">{$_('account.reconciledTo')}&nbsp;</div>
                    <div class="last-date">{a.reconciliation_info && a.reconciliation_info.date ? formatDate(a.reconciliation_info.date) : '-'}</div>
                </div>
            </div>
            {/if}
            {/if}
        </div>
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
        display: grid;
    }

    .account-type {
        font-size: 0.8em;
        font-weight: 500;
        color: var(--color-text-subtle);
        margin: 5px 0px -5px 10px;
        float: left;
        clear: both;
        display: flex;
    }

    .edit-icon {
        float: right;
        color: var(--color-surface);
    }

    .card:hover .edit-icon {
        color: var(--color-icon-card-hover);
    }

    .edit-icon:hover {
        color: var(--color-icon-hover) !important;
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
        color: var(--color-text-hover);
    }

    .small-sub-heading {
        font-size: 1.5em;
        color: var(--color-text-dim);
        margin: -8px 0 -5px 10px;
        float: left;
        clear: both;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        max-width: 350px;
    }

    .card-bottom {
        display: inline-flex;
        text-align: left;
        margin: 0px 10px 5px 1px;
        color: var(--color-text-strong);
        vertical-align: top;
    }

    .last-date {
        min-width: 200px;
        white-space: nowrap;
        display: inline-block;
        font-size: .6em;
        line-height: 1em;
        color: var(--color-text-muted);
    }

    hr {
        border: 1px solid var(--color-bg);
        margin: 7px -10px;
        width: calc(100% + 20px);
    }

    .label {
        font-size: .8em;
        margin: 0 !important;
    }

    .card-label {
        padding: 0 !important;
        font-size: .6em !important;
        line-height: 1em !important;
        color: var(--color-text-muted);
    }

</style>