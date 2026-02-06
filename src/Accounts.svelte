<script>
    import EditAccount from "./EditAccount.svelte"
    import Icon from '@iconify/svelte'
    import {page, isEditMode, modes, views} from "./page"
    import {accounts} from './accounts'
    import { context } from "./context"
    import EditBooks from "./EditBooks.svelte"
    import {onMount} from 'svelte'
    import { _ } from 'svelte-i18n'

    export let curAccount
    export let loadAccounts

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
    
</script>

{#if ! $context.hasBooks}
<EditBooks />
{:else if isEditMode($page)}
<EditAccount {curAccount} {loadAccounts} {close} initialize={accounts.length < 1}/>
{:else}
    <div>
        {#if !isEditMode($page)}
        <div class="toolbar toolbar-right">
            <button class="toolbar-icon" on:click="{handleAddClick}" title={$_('accounts.buttons.createNew')}><Icon icon="mdi:plus-box-outline"  width="24"/></button>
        </div>
        {/if}
        <div class="form-heading">{$_('accounts.title')}</div>
    </div>
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
    
</style>