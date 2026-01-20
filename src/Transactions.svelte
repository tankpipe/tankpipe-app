<script>
    import EditTransaction from './EditTransaction.svelte'
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import { Errors } from './errors'
    import { page, modes, isEditMode, isMultiEditMode, isSingleEditMode, isListMode } from './page'
    import { settings } from './settings'
    import { accounts } from './accounts'
    import { invoke } from "@tauri-apps/api/core"
    import { chart } from "svelte-apexcharts"
    import EditMultipleTransactions from './EditMultipleTransactions.svelte'
    import { _ } from 'svelte-i18n'
    import Importer from './Importer.svelte'
    import { selector, toggleAllSelected, toggleMultipleSelect, clearSelected, isSelected, getSelected } from './selector'
    import { chartOptions } from './chart-options'
    import TransactionList from './TransactionList.svelte'

    let { curAccount, journalMode = false } = $props()

    let curEntry = $state({})
    let errors = $state(new Errors())
    let msg = $state("")
    let previousAccount
    let topScroll
    let showFilter = $state(false)
    let descriptionFilter = $state("")
    let allTransactions = []
    let transactions = $derived([])
    let chartValues = []

    $effect(() => {
        if (journalMode && !curAccount) {
            curAccount = {}
        } else if (!journalMode && (!curAccount || !curAccount.id) && $accounts.length > 0) {
            curAccount = $accounts[0]
        } else if (curAccount && curAccount !== previousAccount) {
            topScroll = null
            transactions = []
            clearSelected()
            errors = new Errors()
            msg = ""
            loadTransactions()
            previousAccount = curAccount
        }

        if (!$page.isEditMode) {
            if (!topScroll) {
                const closest = findClosestTransaction()
                if (closest) {
                    topScroll = getScrollPosition(closest.id)
                }
            }
            scrollToPosition()
        }
    });

    const setCurrentScroll = () => {
        topScroll = document.getElementById("scroller").scrollTop
    }

    const scrollToPosition = () => {
        const scroller = document.getElementById("scroller")
        if (scroller && !isEditMode($page)) {
            document.getElementById("scroller").scrollTo(0, topScroll)
        }
    }

    const getScrollPosition = (id) => {
        var myElement = document.getElementById(id);
        if (myElement) {
            return myElement.offsetTop;
        }
        return 0
    }

    const selectTransaction = (entry) => {
        curEntry = entry
        page.set({view: $page.view, mode: modes.EDIT})
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
        console.log("loadTransactions: " + curAccount)

        if (!curAccount || !curAccount.id) {
            allTransactions = await invoke("all_transactions", {})
        } else {
            allTransactions = await invoke("transactions", { accountId: curAccount.id })
        }

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
        console.log("filterList")
        transactions = allTransactions.filter(
            t => descriptionFilter == "" ||
            (journalMode && t.entries.filter(e => e.description.toLowerCase().includes(descriptionFilter.toLowerCase())).length > 0) ||
            (!journalMode && getEntry(t).description.toLowerCase().includes(descriptionFilter.toLowerCase())))

        console.log("filterList: " + transactions.length)
    }

    const chartBalance = (balance) => {
        return curAccount.account_type == "Liability" ? balance * -1 : balance
    }

    const findClosestTransaction = () => {
        const today = new Date().setUTCHours(0,0,0,0)
        let tDate

        if (transactions) {
            for (const t of transactions) {
                tDate = new Date(t.entries[0].date)
                if (tDate >= today) {
                    return t
                }
            }
        }

        return null
    }


    const getEntry = (transaction) => {
        return transaction.entries.find(e => e.account_id == curAccount.id)
    }

    const handleAddClick = () => {
        setCurrentScroll()
        page.set({view: $page.view, mode: modes.NEW})
    }

    function evaluationResult(result) {
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

    const onCloseMultiEdit = () => {
        loadTransactions()
        page.set({view: $page.view, mode: modes.LIST})
    }

    const onCloseEdit = () => {
        loadTransactions()
        page.set({view: $page.view, mode: modes.LIST})
    }

</script>

<div class="account-heading">
    {#if isListMode($page)}
    <div class="account">
        <Select bind:item={curAccount} items={$accounts} none={journalMode || settings.require_double_entry} flat={true}/>
    </div>
    <div class="toolbar">
        <button type="button" class="toolbar-icon" onclick="{() => handleAddClick(curAccount)}" title={$_('transactions.addTransaction')}><Icon icon="mdi:plus-box-outline"  width="24"/></button>
        <button type="button" class="{showFilter ? 'toolbar-icon-on' : 'toolbar-icon'}" onclick="{() => toggleShowFilter()}" title="{showFilter ? $_('transactions.hideFilter') : $_('transactions.showFilter')}"><Icon icon="mdi:filter-outline"  width="24"/></button>
        <button type="button" class="{$selector.showMultipleSelect ? 'toolbar-icon-on' : 'toolbar-icon'}" onclick="{() => toggleMultipleSelect()}" title="{$selector.showMultipleSelect ? $_('transactions.hideSelect') : $_('transactions.showSelect')}"><Icon icon="mdi:checkbox-multiple-marked-outline"  width="24"/></button>
        <button type="button" class="{$selector.showMultiEdit && $selector.shapeMatch ? 'toolbar-icon' : 'toolbar-icon-disabled'}" onclick="{() => {if ($selector.showMultiEdit && $selector.shapeMatch) editTransactions()}}" title={$_('transactions.editSelected')}><Icon icon="mdi:edit-box-outline"  width="24"/></button>
        <button type="button" class="{$selector.showMultiEdit ? 'toolbar-icon' : 'toolbar-icon-disabled'} warning" onclick="{() => {if ($selector.showMultiEdit) deleteTransactions()}}" title={$_('transactions.deleteSelected')}><Icon icon="mdi:trash-can-outline"  width="24"/></button>
        {#if curAccount}
        <button type="button" class="toolbar-icon import-icon" onclick={evaluationResult} title={$_('transactions.openCsv')}><Icon icon="mdi:folder-upload" width="22"/></button>
        {/if}
    </div>
    {#if transactions.length > 0}
    <div class="chart"><div use:chart={chartOptions}></div></div>
    {/if}
    {/if}
</div>
{#if isSingleEditMode($page)}
<EditTransaction {loadTransactions} {curEntry} onClose={onCloseEdit} />
{/if}
{#if isMultiEditMode($page)}
<EditMultipleTransactions {loadTransactions} onClose={onCloseMultiEdit} {curAccount} transactions={getSortedSelectedTransactions()}/>
{/if}
{#if $page.mode == modes.LOAD}
<Importer {curAccount} onClose={onCloseEdit} />
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
<TransactionList {curAccount} {journalMode} transactions={transactions} onSelect={selectTransaction} />
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
        margin: 0 0 5px 10px;
        display: flex;
        padding-right: 9px;
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
        color: #F0F0F0;
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
    }

    @media (min-width: 1010px) {
        .description {
            max-width: calc(70vw - 350px);
        }
    }

</style>