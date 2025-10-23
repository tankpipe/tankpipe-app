<script>
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import {page, modes} from './page.js'
    import {accounts} from './accounts.js'
    import {config} from './config.js'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'

    export let close
    export let curAccount
    export let loadAccounts
    export let initialize = false

    const ACCOUNT_TYPES = [{value:"Asset", name:"Asset"}, {value:"Liability", name:"Liability"}, {value:"Revenue", name:"Revenue"}, {value:"Expense", name:"Expense"}, {value:"Equity", name:"Equity"}]

    let msg = ""
    let errors = new Errors()
    let name, startingBalance, accountType
    let addButtonLabel = $_('buttons.add')

    onMount(() => {
        if ($page.mode === modes.EDIT) {
            name = curAccount.name
            startingBalance = curAccount.starting_balance
            accountType = matchAccountType(curAccount.account_type)
            addButtonLabel = $_('buttons.update')
        } else {
            addButtonLabel = $_('buttons.add')
            startingBalance = "0"
            curAccount = null
        }
    })

    $: {
        if (curAccount && curAccount.id) loadTransactions()
    }

    let transactions = []
    export const loadTransactions = async () => {
        console.log("loadTransactions: " + curAccount.id)
        transactions = await invoke('transactions', {accountId: curAccount.id})
        console.log(transactions)
    }

    const matchAccountType = (value) =>  {
        if (!value) return null
        let match = ACCOUNT_TYPES.filter(p => p.value == value)
        return match.length > 0 ? match[0] : null
    }

    const onCancel = () => {
        close()
    }

    const onAdd = () => {
        msg = ""
        errors = new Errors()
        if (!name || name.length < 1) {
            errors.addError("name", $_('account.form.errors.nameRequired'))
        }

        if (!startingBalance || startingBalance.length < 1 || isNaN(startingBalance)) {
            errors.addError("startingBalance", $_('account.form.errors.startingBalanceRequired'))
        }

        if (!accountType || !accountType.value) {
            errors.addError("accountType", $_('account.form.errors.accountTypeRequired'))
        }

        if (!errors.hasErrors()) {
            if ($page.mode === modes.NEW) {
                const account = {
                    name: name,
                    starting_balance: startingBalance,
                    account_type: accountType.value
                }
                addAccount(account)
            } else if ($page.mode === modes.EDIT) {
                const account = {
                    name: name,
                    starting_balance: startingBalance,
                    account_type: curAccount.account_type,
                    id: curAccount.id,
                    balance: 0
                }
                saveAccount(account)
            }
        }
    }

    function resolved(result) {
        msg = $_('account.form.success.saved')
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", $_('account.form.errors.genericError', { values: { 0: result } }))
    }

    const addAccount = async (account) => {
        await invoke('add_account', {account: account}).then(resolved, rejected)
        loadAccounts()
        initialize = false
    }

    const saveAccount = async (account) => {
        console.log(account)
        await invoke('update_account', {account: account}).then(resolved, rejected)
        loadAccounts()
    }

    function deleteResolved(result) {
        msg = $_('account.form.success.deleted')
        loadAccounts()
        close()
    }

    const deleteAccount = async (account) => {
        console.log(account)
        if (account) {
            await invoke('delete_account', {account: account}).then(deleteResolved, rejected)
        } else {
            close()
        }
    }
</script>

{#if $accounts.length < 1 && $config.recent_files.length < 2}
<div class="message">{$_('account.firstAccountMessage')}</div>
{/if}

<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT ? $_('account.form.title.edit') : $_('account.form.title.new')}</div>
    <div class="toolbar">
        {#if transactions.length < 1}
        <div class="toolbar-icon" on:click="{deleteAccount(curAccount)}" title={$_('account.form.deleteTooltip')}><Icon icon="mdi:trash-can-outline"  width="24"/></div>
        {/if}
    </div>
    <div class="form-row">
        <div class="widget">
            <label for="name">{$_('labels.name')}</label>
            <input id="name" class="description-input" class:error={errors.isInError("name")} bind:value={name}>
        </div>
        <div class="widget">
            <label for="startingBalance">{$_('account.form.labels.startingBalance')}</label>
            <input id="startingBalance" class="money-input" class:error={errors.isInError("startingBalance")} bind:value={startingBalance}>
        </div>
    </div>
    <div class="form-row">
        <Select bind:item={accountType} items={ACCOUNT_TYPES} label={$_('account.form.labels.type')} none={false} inError={errors.isInError("accountType")} disabled={$page.mode === modes.EDIT} flat={true}/>
    </div>
    <div class="form-button-row">
        <div class="msg-panel">
            {#each errors.getErrorMessages() as e}
            <p class="error-msg">{e}</p>
            {/each}
            {#if msg}
            <p class="success-msg">{msg}</p>
            {/if}
        </div>
        <div class="widget buttons">
            <button class="og-button" on:click={onCancel}>{$_('buttons.close')}</button>
            <button class="og-button" on:click={onAdd}>{addButtonLabel}</button>
        </div>
    </div>
</div>

<style>

    :global(.date-time-field input) {
        border: 1px solid #CCC !important;
        border-radius: 2px !important;
        height: 33px;
    }

    :root {
        --date-input-width: 110px;
    }

    .msg-panel {
        padding-left: 2px;
        font-size: 0.9em;
        float:left;
    }

    :global(.message) {
        color: #EFEFEF;
        margin-bottom: 20px;
        text-align: left;
        background-color: #303030;
        padding:10px;
        border-radius: 10px;
    }

    .msg-panel p {
        margin: 8px 0;
        max-width: 500px;
    }

    .error-msg {
        color: #FBC969;
    }

    .success-msg {
        color: green;
    }

    .error {
        border: 1px solid #FBC969 !important;
    }

    :global(.error-input input) {
        border: 1px solid #FBC969 !important;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .buttons button {
        min-width: 80px;
    }

    .form-row {
        display: inline-flex;
        float: left;
        width: 100%;
        clear:both;
    }

    .form-button-row {
        display: block;
        text-align: left;
    }

    .form-button-row {
        margin-left: 7px;
        margin-right: 2px;
    }

    input {
        margin-right: 0px;
    }

    .form {
        float: left;
        border-radius: 10px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .money-input {
        width: 110px;
    }

    .money-input {
        text-align: right;
    }

    .description-input {
        width: 400px;
    }

</style>