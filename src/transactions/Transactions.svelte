<script>
    import EditTransaction from './EditTransaction.svelte'
    import Transaction from './Transaction.svelte'
    import Select from '../components/Select.svelte'
    import Icon from '@iconify/svelte'
    import { Errors } from '../utils/errors'
    import { page, modes, views, isMultiEditMode, isSingleEditMode, isListMode, isViewMode } from '../stores/page'
    import { settings } from '../stores/settings'
    import { accounts, updateAccounts } from '../stores/accounts'
    import { invoke } from "@tauri-apps/api/core"
    import { save } from '@tauri-apps/plugin-dialog'
    import { documentDir } from '@tauri-apps/api/path'
    import { chart } from "svelte-apexcharts"
    import EditMultipleTransactions from './EditMultipleTransactions.svelte'
    import { _ } from 'svelte-i18n'
    import Importer from './Importer.svelte'
    import { selector, toggleAllSelected, toggleMultipleSelect, clearSelected, isSelected, getSelected } from './selector'
    import { chartOptions } from './chart-options'
    import TransactionList from './TransactionList.svelte'
    import { ReconciliationMode as RM } from './reconciliation.js'

    let { curAccount, journalMode = false } = $props()

    let curTransaction = $state({})
    let editSource = $state(null)
    let errors = $state(new Errors())
    let msg = $state("")
    let previousAccountId
    let topScroll = $state(null)
    let showFilter = $state(false)
    let descriptionFilter = $state("")
    let allTransactions = []
    let transactions = $derived([])
    let reconciliationResults = $state([])
    let reconciliationAccountId = $state(null)
    // Keep the last reconciliation request so we can rerun it if needed (as changes are made)
    let lastReconciliationRequest = $state(null)
    let reconciliationMode = $state(RM.NONE)
    let chartValues = []

    $effect(() => {
        if (journalMode && !curAccount) {
            curAccount = null
            loadTransactions()
        } else if (!journalMode && (!curAccount || !curAccount.id) && $accounts.length > 0) {
            curAccount = $accounts[0]
        } else if (curAccount && curAccount.id !== previousAccountId) {
            console.log("curAccount changed", curAccount.id)
            reconciliationMode = RM.NONE
            reconciliationResults = []
            reconciliationAccountId = null
            topScroll = null
            transactions = []
            clearSelected()
            errors = new Errors()
            msg = ""
            loadTransactions()
            previousAccountId = curAccount.id
        }
       
    });

    $effect(() => {
        if ($page.payload && $page.payload.accountId) {
            curAccount = $accounts.find(account => account.id === $page.payload.accountId)
        }
    })

    const setCurrentScroll = () => {
        topScroll = document.getElementById("scroller").scrollTop
    }

    const isAllReconciled = (transaction) => {                
        return transaction.entries.every(e => e.reconciled_status)
    }

    const selectTransaction = (transaction) => {
        curTransaction = transaction
        editSource = transaction?.prefillEdit
            ? {
                ...transaction.editPatch,
                mergeAccountId: transaction.mergeAccountId,
                targetTransactionId: transaction.targetTransactionId
            }
            : null
        
        // Route to view-only mode for reconciled transactions
        if (isAllReconciled(transaction)) {
            page.set({view: $page.view, mode: modes.VIEW})
        } else if (transaction.isReconciliationResult) {
            page.set({view: $page.view, mode: modes.NEW})
        } else {
            page.set({view: $page.view, mode: modes.EDIT})
        }
    }

    const editTransactions = () => {
        setCurrentScroll()
        page.set({view: $page.view, mode: modes.MULTI_EDIT})
    }

    const deleteTransactions = async () => {
        const selectedIds = getSelected()
        if (selectedIds.length > 0) {
            await invoke('delete_transactions', {ids: selectedIds}).then(resolvedDelete, rejected)
        }
    }

    function resolvedDelete(result) {
       msg = $_('transactions.transactionsDeleted')
      loadTransactions()
    }

    export const loadTransactions = async () => {
        console.log("loadTransactions for", curAccount?.id)

        if (!curAccount || !curAccount.id) {
            allTransactions = await invoke("all_transactions", {})
        } else {
            allTransactions = await invoke("transactions", { accountId: curAccount.id })
        }

        console.log("allTransactions", allTransactions)
        chartValues = []
        if (!journalMode) {
            for (const t of allTransactions) {
                let entry = getEntry(t)
                chartValues.push([new Date(entry.date).valueOf(), chartBalance(entry.balance)])
            }
            chartOptions["series"] = [{data: chartValues}]
        }
        filterList()
    }

    const filterList = () => {
        transactions = allTransactions.filter(
            t => descriptionFilter == "" ||
            (journalMode && t.entries.filter(e => e.description.toLowerCase().includes(descriptionFilter.toLowerCase())).length > 0) ||
            (!journalMode && getEntry(t).description.toLowerCase().includes(descriptionFilter.toLowerCase())))
    }

    const chartBalance = (balance) => {
        return curAccount.account_type == "Liability" ? balance * -1 : balance
    }

    const getEntry = (transaction) => {
        return transaction.entries.find(e => e.account_id == curAccount.id)
    }

    const editAccount = () => {
        if (curAccount && curAccount.id) {
            page.set({view: views.ACCOUNTS, mode: modes.EDIT, payload: {accountId: curAccount.id, previousView: views.TRANSACTIONS}})
        }
    }

    const handleAddClick = (account) => {
        setCurrentScroll()
        page.set({view: $page.view, mode: modes.NEW})
    }

    function evaluationResult(result) {        
        topScroll = null
        page.set({view: $page.view, mode: modes.LOAD})
    }

    function rejected(result) {
        console.log(result)
        errors = new Errors()
        errors.addError("all", $_('transactions.error', { values: {error: result} }))
    }

    const toggleShowFilter = () => {
        showFilter = !showFilter

        if (!showFilter) {
            descriptionFilter = ""
            filterList()
        }
    }

    const getSortedSelectedTransactions = () => {
        let selected = []
        for (const t of transactions) {
            if (isSelected(t)) {
                selected.push(t)
            }
        }

        return selected;
    }

    const clearFilter = () => {
        descriptionFilter = ''
        if ($selector.isSelectAll) {
            toggleAllSelected(transactions)
        } else {
            clearSelected()
        }
        filterList();
    }

    const onCloseMultiEdit = async () => {
        await loadTransactions()
        await rerunReconciliationIfNeeded()
        page.set({view: $page.view, mode: modes.LIST})
    }

    const onCloseEdit = async () => {
        await loadTransactions()
        await rerunReconciliationIfNeeded()
        page.set({view: $page.view, mode: modes.LIST})
    }

    const handleReconciliationResults = (results, request) => {        
        reconciliationResults = results ?? []
        const safeRequest = request ?? null
        lastReconciliationRequest = safeRequest
        reconciliationAccountId = safeRequest?.accountId ?? curAccount?.id ?? null        
        reconciliationMode = RM.GUIDED
        page.set({view: $page.view, mode: modes.LIST})
    }

    const exitReconciliationMode = () => {
        reconciliationMode = RM.NONE
        reconciliationResults = []
        reconciliationAccountId = null
        lastReconciliationRequest = null
        loadTransactions()
    }
    
    const loadAccounts = async () => {
        let result = await invoke('accounts')
        updateAccounts(result)
        curAccount = $accounts.find(a => a.id === curAccount.id)
        loadTransactions()
    };   

    const onManualReconciliationMode = () => {
        reconciliationMode = reconciliationMode === RM.MANUAL ? RM.NONE : RM.MANUAL;
        reconciliationResults = [];
        reconciliationAccountId = reconciliationMode === RM.MANUAL ? (curAccount?.id ?? null) : null;
    }

    const rerunReconciliationIfNeeded = async () => {
        if (!lastReconciliationRequest) return false
        if (!curAccount || curAccount.id !== lastReconciliationRequest.accountId) return false
        try {
            const results = await invoke('reconcile_csv', {
                path: lastReconciliationRequest.path,
                accountId: curAccount.id,
                columnTypes: lastReconciliationRequest.columnTypes,
                hasHeaders: lastReconciliationRequest.hasHeaders
            })
            handleReconciliationResults(results, lastReconciliationRequest)
            return true
        } catch (err) {
            console.log(err)
            errors = new Errors()
            errors.addError("all", $_('transactions.error', { values: {error: err} }))
            return false
        }
    }

    const csvExport = async () => {
        let defaultPath
        await documentDir()
            .then(path => defaultPath = path)
            .catch(e => console.log("error getting document dir: " + e))

        const defaultFileName = journalMode 
            ? "all_transactions.csv" 
            : `${curAccount.name || 'transactions'}_${new Date().toISOString().split('T')[0]}.csv`

        const selected = await save({
            filters: [{name: 'CSV Files', extensions: ['csv']}],
            defaultPath: `${defaultPath}/${defaultFileName}`,
        })

        if (selected) {
            try {
                if (journalMode && !curAccount?.id) {
                    await invoke('export_csv_all', { path: selected })
                } else {
                    await invoke('export_csv', { path: selected, accountId: curAccount.id })
                }
                msg = $_('transactions.exportSuccess')
            } catch (err) {
                console.log(err)
                errors = new Errors()
                errors.addError("all", $_('transactions.error', { values: {error: err} }))
            }
        }
    }

</script>

<div class="account-heading">
    {#if isListMode($page)}
    <div class="account">
        <Select bind:item={curAccount} items={$accounts} none={journalMode || settings.require_double_entry} flat={true} onChange={() => {console.log("onChange"); reconciliationMode = RM.NONE; reconciliationResults = []; reconciliationAccountId = null}}/>
    </div>
    <div class="toolbar">
        <button type="button" class="toolbar-icon" onclick={editAccount} title={$_('transactions.editAccount')}><Icon icon="mdi:text-box-edit-outline"  width="24"/></button>
        <button type="button" class="toolbar-icon" onclick="{() => handleAddClick(curAccount)}" title={$_('transactions.addTransaction')}><Icon icon="mdi:plus-box-outline"  width="24"/></button>
        <button type="button" class="{showFilter ? 'toolbar-icon-on' : 'toolbar-icon'}" onclick="{() => toggleShowFilter()}" title="{showFilter ? $_('transactions.hideFilter') : $_('transactions.showFilter')}"><Icon icon="mdi:filter-outline"  width="24"/></button>
        <button type="button" class="{$selector.showMultipleSelect ? 'toolbar-icon-on' : 'toolbar-icon'}" onclick="{() => toggleMultipleSelect()}" title="{$selector.showMultipleSelect ? $_('transactions.hideSelect') : $_('transactions.showSelect')}"><Icon icon="mdi:checkbox-multiple-marked-outline"  width="24"/></button>
        <button type="button" class="{$selector.showMultiEdit && $selector.shapeMatch ? 'toolbar-icon' : 'toolbar-icon-disabled'}" onclick="{() => {if ($selector.showMultiEdit && $selector.shapeMatch) editTransactions()}}" title={$_('transactions.editSelected')}><Icon icon="mdi:edit-box-outline"  width="24"/></button>
        <button type="button" class="{$selector.showMultiEdit ? 'toolbar-icon' : 'toolbar-icon-disabled'} warning" onclick="{() => {if ($selector.showMultiEdit) deleteTransactions()}}" title={$_('transactions.deleteSelected')}><Icon icon="mdi:trash-can-outline"  width="24"/></button>
        {#if curAccount && ! journalMode}
        <button type="button" class="{reconciliationMode === RM.MANUAL ? 'toolbar-icon-on' : 'toolbar-icon'}" onclick={onManualReconciliationMode} title={$_('transactions.manualReconciliation')}
        >
            <Icon icon="mdi:check" width="24"/>
        </button>
        <button type="button" class="toolbar-icon import-icon" onclick={evaluationResult} title={$_('transactions.openCsv')}><Icon icon="mdi:folder-upload" width="22"/></button>        
        {/if}
        {#if (curAccount?.id && ! journalMode || (journalMode && ! curAccount?.id)) && transactions.length > 0}
        <button type="button" class="toolbar-icon import-icon" onclick={csvExport} title={$_('transactions.exportCsv')}><Icon icon="mdi:folder-download" width="22"/></button>
        {/if}
    </div>
    {#if transactions.length > 0}
    <div class="chart"><div use:chart={chartOptions}></div></div>
    {/if}
    {/if}
</div>

{#if isViewMode($page)}
<Transaction transactionId={curTransaction.id} onClose={onCloseEdit} />
{/if}
{#if isSingleEditMode($page)}
<EditTransaction {loadTransactions} transactionId={curTransaction.id} onClose={onCloseEdit} reconciliationSource={curTransaction} {editSource}/>
{/if}
{#if isMultiEditMode($page)}
<EditMultipleTransactions {loadTransactions} onClose={onCloseMultiEdit} {curAccount} transactions={getSortedSelectedTransactions()}/>
{/if}
{#if $page.mode == modes.LOAD}
<Importer {curAccount} onClose={onCloseEdit} onReconciliationResults={handleReconciliationResults} />
{/if}
{#if isListMode($page)}
<div class="widget errors">
    {#each errors.getErrorMessages() as e}
    <div class="error-msg">{e}</div>
    {/each}
    {#if msg}
    <div class="success-msg">{msg}</div>
    {/if}
</div>
{#if reconciliationMode === RM.GUIDED}
<div class="reconciliation-header">
    <div class="reconciliation-title">{$_('transactions.reconciliationHeader')}</div>
    <button class="exit-reconciliation" onclick={exitReconciliationMode}>
        <Icon icon="mdi:close" width="16"/>
    </button>
</div>
{/if}
{#if showFilter}
<div class="" id="filter">
    <table>
        <tbody>
        <tr><th class="justify-left">{$_('transactions.filter')}</th></tr>
        <tr class="form">
            <td class="description">
                <input id="desc" class="description-input-2" style="width: 60%" bind:value={descriptionFilter} oninput={() => {filterList()} }>
                <button class="single-button" onclick={clearFilter} title={$_('transactions.clearFilter')}><Icon icon="mdi:eraser"  width="16"/></button>
            </td>
        </tr>
        </tbody>
    </table>
</div>
{/if}
<TransactionList
    {curAccount}
    {journalMode}
    transactions={transactions}
    reconciliationResults={reconciliationAccountId === curAccount?.id ? reconciliationResults : []}
    reconciliationMode={reconciliationAccountId === curAccount?.id ? reconciliationMode : RM.NONE}
    onSelect={selectTransaction}
    loadAccounts={loadAccounts}
    rerunReconciliationIfNeeded={rerunReconciliationIfNeeded}
    topScroll={topScroll}
    setTopScroll={(value) => (topScroll = value)}
    descriptionFilter={descriptionFilter}
/>
{/if}

<style>
    table {
        padding-right: 10px;
        width: 100%;
    }

    td {
        text-align: left;
        overflow: hidden;
        line-height: 1em;
        color: #ccc;
        background-color: #393939;
        padding: 8px;
        white-space: nowrap;
        font-size: 0.9em;
    }

    th {
        color:#666666;
        background-color: #444;
        font-weight: 400;
        font-size: .8em;
    }
    .justify-left {
        text-align: left;
        padding-left: 10px;
    }

    .form input {
        margin: 0px;
    }

    .description {
        min-width: 350px;
        max-width: 33vw;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .account {
        float: left;
    }

    .chart {
        float: right;
        color: #c0c0c0;
        margin: 0 30px 5px 10px;
        display: flex;
        padding-right: 13px;
        width: 105px;
    }

    :global(.toolbar) {
        float: left;
        color: #c0c0c0;
        margin-left: 10px;
        display: flex;
        padding-right: 9px;
    }

    :global(.toolbar-right) {
        float: right;
    }
    :global(.toolbar button, .single-button) {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0px;
    }

    :global(.toolbar-icon) {
        margin-left: 5px;
        color: #c0c0c0;
    }

    :global(.toolbar-icon:hover) {
        color: #F0F0F0;
        cursor: pointer;
    }

    :global(.toolbar-icon-disabled) {
        margin-left: 5px;
        color: #303030;
    }

    :global(.toolbar-icon:disabled) {
        margin-left: 5px;
        color: #303030;
        cursor: default;
    }

    :global(.toolbar-icon-on) {
        margin-left: 5px;
        color: #43bd6e; /*#55e688*/
    }

    :global(.toolbar-icon-on:hover) {
        color: #55e688;
        cursor: pointer;
    }

    :global(.warning:hover) {
        color: #e68843;
    }

    :global(.import-icon) {
        margin-top: 1px
    }

    :global(.single-button) {
        display: inline-flex;
        vertical-align: top;
        margin-left: 0;
        background: none;
        border: none;
        cursor: pointer;
        padding: 0px;
        color: #c0c0c0;
    }

    :global(.single-button:hover) {
        cursor: pointer;
        color: #e0e0e0;
    }

    .reconciliation-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        background-color: #2a2a2a;
        padding: 10px 15px;
        margin: 10px 35px 10px 0;
        border-radius: 5px;
        border-left: 4px solid #4CAF50;
    }

    .reconciliation-title {
        color: #4CAF50;
        font-weight: bold;
        font-size: 1.1em;
    }

    .exit-reconciliation {
        background-color: #555;
        border: none;
        color: #fff;
        padding: 5px 10px;
        border-radius: 3px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 5px;
        font-size: 0.9em;
    }

    .exit-reconciliation:hover {
        background-color: #666;
    }

    .errors {
        padding-bottom: 10px;
    }

    .error-msg {
        color: red;
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
    }

    .success-msg {
        color: green;
        text-align: left;
        font-size: 0.85em;
    }

    @media (min-width: 1010px) {
        .description {
            max-width: calc(70vw - 350px);
        }
    }

</style>
