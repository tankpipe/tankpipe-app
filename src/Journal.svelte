<script>
    import EditTransaction from './EditTransaction.svelte'
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import { open } from '@tauri-apps/plugin-dialog'
    import { documentDir } from '@tauri-apps/api/path'
    import { Errors } from './errors'
    import { page, modes, views, isEditMode, isMultiEditMode, isSingleEditMode } from './page'
    import { settings } from './settings'
    import { config } from './config.js'
    import { accounts } from './accounts'
    import { afterUpdate } from 'svelte'
    import { invoke } from "@tauri-apps/api/core"
    import { chart } from "svelte-apexcharts"
    import EditMultipleTransactions from './EditMultipleTransactions.svelte';
    import { SvelteSet } from 'svelte/reactivity';

    export let curAccount

    let curEntry
    let errors = new Errors()
    let msg = ""
    let previousAccount
    let topScroll
    let showMultipleSelect = false
    let showMultiEdit = false
    let isSelectAll = false
    let showFilter = true
    let descriptionFilter = ""
    let deleteUnlocked = false

    $: {
        if (!curAccount && $accounts.length > 0) {
            curAccount = {}
        }
        if (curAccount && curAccount !== previousAccount) {
            topScroll = null
            transactions = []
            selectedTransactions.clear()
            errors = new Errors()
            msg = ""
            loadTransactions()
            previousAccount = curAccount
        }
    }

    afterUpdate(() => {
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
        console.log("setCurrentScroll: ", document.getElementById("scroller").scrollTop)
        topScroll = document.getElementById("scroller").scrollTop
    }

    const scrollToPosition = () => {
        const scroller = document.getElementById("scroller")
        if (scroller && !isEditMode($page)) {
            console.log("scrollToPosition: ", topScroll)
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
        setCurrentScroll()
        curEntry = entry
        page.set({view: views.JOURNAL, mode: modes.EDIT})
    }

    const editTransactions = () => {
        setCurrentScroll()
        page.set({view: views.JOURNAL, mode: modes.MULTI_EDIT})
    }

    const deleteTransactions = async () => {
        const selectedIds = Array.from(selectedTransactions)
        if (selectedIds.length > 0) {
            await invoke('delete_transactions', {ids: selectedIds}).then(resolvedDelete, rejected)
        }
    }

    function resolvedDelete(result) {
      msg = "Transactions deleted."
      loadTransactions()
    }

    let chartOptions = {
        series: [
            {
                name: "foo",
                data: [],
            },
        ],
        chart: {
            type: "area",
            height: 50,
            width: 100,
            sparkline: {
                enabled: true,
            },
        },
        stroke: {
            curve: "stepline",
            width: 2,
            colors:["#efefef"],
        },
        fill: {
            opacity: 0.6,
            colors: ["#efefef"],
        },
        xaxis: {
            crosshairs: {
                width: 1,
            },
            type: "datetime",
        },
        tooltip: {
            enabled: false
        },
    }

    let allTransactions = []
    let transactions = []
    let selectedTransactions = new SvelteSet()
    let chartValues = []

    export const loadTransactions = async () => {
        console.log("loadTransactions: " + curAccount)
        if (!curAccount || !curAccount.id) {
            allTransactions = await invoke("all_transactions", {})
        } else {
            allTransactions = await invoke("transactions", { accountId: curAccount.id })
        }
        console.log(allTransactions)
        chartValues = []
        filterList()
    }
    const filterList = () => {
        transactions = allTransactions.filter(t => descriptionFilter == "" || getEntry(t).description.toLowerCase().includes(descriptionFilter.toLowerCase()))
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

    const formatter = new Intl.NumberFormat('en-AU', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    })


    const getDebitAmount = (transaction, curAccount) => {
        return transaction.entry_type === "Debit" ? formatter.format(transaction.amount) : ''
    }

    const getCreditAmount = (transaction, curAccount) => {
        return transaction.entry_type === "Credit" ? formatter.format(transaction.amount) : ''
    }

    const getBalance = (transaction) => {
        return formatter.format(transaction.balance)
    }

    const getDate = (transaction) => {
        const date = new Date(transaction.date)

        switch ($config.display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return transaction.date
            default: return date.toLocaleDateString()
        }
    }

    const date_style = () => {
        switch ($config.display_date_format) {
            case "ISO": return ""
            default: return "align_right"
        }
    }

    const getEntry = (transaction) => {
        return transaction.entries.find(e => e.account_id == curAccount.id)
    }

    const handleAddClick = () => {
        setCurrentScroll()
        page.set({view: views.JOURNAL, mode: modes.NEW})
    }

    const openFile = async () => {
        let appDataDirPath
        await documentDir()
                .then(r => appDataDirPath = r)
                .catch(e => console.log("error : " + e))
        const selected = await open({
            directory: false,
            multiple: false,
            filters: [{name: '*', extensions: ['csv']}],
            defaultPath: appDataDirPath,
        })

        if(selected) {
            console.log(selected)
            loadCsv(selected, curAccount)
        }
    }

    function loaded(result) {
        console.log(result)
        loadTransactions()
    }

    function rejected(result) {
        console.log(result)
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }
    const loadCsv = async (path, account) => {
        console.log(path)
        errors = new Errors()
        await invoke('load_csv', {path: path, account: account}).then(loaded, rejected)
    }

    const projected = (t) => t.status == 'Projected' ? 'projected' : ''
    const date_class = date_style()

    const toggleSelected = (transaction) => {
        if (selectedTransactions.has(transaction)) {
            selectedTransactions.delete(transaction)
        } else {
            selectedTransactions.add(transaction)
            setCurrentScroll()
        }

        showMultiEdit = showMultipleSelect && selectedTransactions.size > 0
    }

    const toggleAllSelected = () => {
        if (isSelectAll) {
            selectedTransactions.clear()
        } else {
            transactions.forEach(t => selectedTransactions.add(t.id))
        }

        isSelectAll = !isSelectAll
        showMultiEdit = showMultipleSelect && selectedTransactions.size > 0
    }

    const toggleMultipleSelect = () => {
        showMultipleSelect = !showMultipleSelect
        if (!showMultipleSelect) {
            isSelectAll = false
            selectedTransactions.clear()
        }
        showMultiEdit = showMultipleSelect && selectedTransactions.size > 0
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
            if (selectedTransactions.has(t.id)) {
                selected.push(t)
            }
        }

        if (selected.length > 0) {
            curEntry = getEntry(selected[0])
        }
        return selected;
    }

    const clearFilter = () => {
        descriptionFilter = ''
        if (isSelectAll) {
            toggleAllSelected()
        } else {
             selectedTransactions.clear()
        }
        filterList();
    }

    const onCloseMultiEdit = () => {
        selectedTransactions.clear()
    }

    const onCloseEdit = () => {
        loadTransactions()
        page.set({view: views.JOURNAL, mode: modes.LIST})
    }

    const sortEntries = (entries) => {
        entries.sort((a, b) => {
            if (a.entry_type === "Debit" && b.entry_type === "Credit") return -1
            if (a.entry_type === "Credit" && b.entry_type === "Debit") return 1
            return 0
        })
    }

</script>
<div class="account-heading">
    {#if !isEditMode($page)}
    <div class="account">
        <Select bind:item={curAccount} items={$accounts} none={true} flat={true}/>
    </div>
    <div class="toolbar">
        <div class="toolbar-icon" on:click="{handleAddClick(curAccount)}" title="Add a transaction"><Icon icon="mdi:plus-box-outline"  width="24"/></div>
        <div class="{showFilter ? 'toolbar-icon-on' : 'toolbar-icon'}" on:click="{toggleShowFilter}" title="{showFilter ? 'Hide filter' : 'Show filter'}"><Icon icon="mdi:filter-outline"  width="24"/></div>
        <div class="{showMultipleSelect ? 'toolbar-icon-on' : 'toolbar-icon'}" on:click="{toggleMultipleSelect}" title="{showMultipleSelect ? 'Hide select transactions' : 'Show select transactions'}"><Icon icon="mdi:checkbox-multiple-marked-outline"  width="24"/></div>
        <div class="{showMultiEdit ? 'toolbar-icon' : 'toolbar-icon-disabled'}" on:click="{() => {if (showMultiEdit) editTransactions()}}" title="Edit selected transactions"><Icon icon="mdi:edit-box-outline"  width="24"/></div>
        <div class="{showMultiEdit ? 'toolbar-icon' : 'toolbar-icon-disabled'} warning" on:click="{() => {if (showMultiEdit) deleteTransactions()}}" title="Delete selected transactions"><Icon icon="mdi:trash-can-outline"  width="24"/></div>
        {#if curAccount}
        <div class="toolbar-icon import-icon" on:click={openFile} title="Import transactions"><Icon icon="mdi:application-import" width="22"/></div>
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
{#if isMultiEditMode($page)}
<EditMultipleTransactions {loadTransactions} onClose={onCloseMultiEdit} {curAccount} transactions={getSortedSelectedTransactions()}/>
{/if}
{/if}
{#if !isEditMode($page)}
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
        <tr><th class="justify-left">Filter</th></tr>
        <tr class="form">
            <td class="description">
                <input id="desc" class="description-input-2" style="width: 60%" bind:value={descriptionFilter} on:input={() => {filterList()} }>
                <div class="filter-icon" on:click={clearFilter} title="Clear filter"><Icon icon="mdi:eraser"  width="16"/></div>
            </td>
        </tr>
        </tbody>
    </table>
</div>
{/if}
<div class="scroller" id="scroller">
    <table>
        <tbody>
        <tr>
            {#if showMultipleSelect}
            <th on:click|stopPropagation={() => toggleAllSelected()}><input id="selectAll" type=checkbox checked={isSelectAll}></th>
            {/if}
            <th class="justify-left">Date</th><th class="justify-left">Description</th><th>Debit</th><th>Credit</th></tr>
        {#each transactions as t}
            {@const _entries = sortEntries(t.entries)}
            {@const selected = selectedTransactions.has(t.id)}
            <tr style="height: 8px;"></tr>
            {#each t.entries as e}
            <tr class="{selected ? 'selected' : ''} {t.entries.length == 1 ? 'single-entry' : ''}"  on:click={() => selectTransaction(e)} id={t.id}><!--{t.id}-->
                {#if showMultipleSelect}<td on:click|stopPropagation={() => toggleSelected(t.id)}><input id={"selected_" + t.id} type=checkbox checked={selected}></td>{/if}
                <td class={projected(t) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(t)} style="{e.entry_type == 'Credit' ? 'padding-left: 30px' : ''}" title="{e.description}"><div class="description">{$accounts.find(a => a.id == e.account_id).name}</div>
                    <div class="description tiny">{e.description}</div>
                </td>
                <td class="{projected(t)} money">{getDebitAmount(e, curAccount)}</td>
                <td class="{projected(t)} money">{getCreditAmount(e, curAccount)}</td>
            </tr>
            {/each}
        {/each}
        </tbody>
    </table>
    {#if transactions.length < 1}
    <div class="message">No transactions</div>
    {/if}
</div>
{/if}

<style>
    .filter {
        width: 100%;
    }

    .scroller{
        height: 100%;
        width: 100%;
        overflow: scroll;
    }

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

    .align_right {
        text-align: right;
    }

    .projected {
        color: #878787;
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

    .scroller tr:hover td {
        cursor: pointer;
        color: #FFF;
    }

    tr:hover td .tiny{
        cursor: pointer;
        color: #C0C0C0;
    }

    .selected td {
        background-color: #1a3924;
        color: #e3e3e3;
    }

    .single-entry td {
        background-color: #34391a;
    }

    .form input {
        margin: 0px;
    }

    .money {
        text-align: right !important;
        min-width: 92px;
        font-family: 'Courier New', Courier, monospace;
        font-weight: bold;
    }

    .description {
        min-width: 350px;
        max-width: 33vw;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .tiny {
        font-size: 0.5em;
        color: #878787;
        margin: 3px 0 -5px 2px;
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

    .toolbar {
        float: left;
        color: #c0c0c0;
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

    .toolbar-icon-disabled {
        margin-left: 5px;
        color: #303030;
    }

    .toolbar-icon-on {
        margin-left: 5px;
        color: #43bd6e; /*#55e688*/
    }

    .toolbar-icon-on:hover{
        color: #55e688;
        cursor: pointer;
    }

    .warning:hover {
        color: #e68843;
    }

    .import-icon {
        margin-top: 1px
    }

    .filter-icon {
        display: inline-flex;
        vertical-align: top;
        margin-left: 0;
    }

    .filter-icon:hover {
        cursor: pointer;
        color: #F0F0F0;
    }
    .message {
        color: #EFEFEF;
        margin: 5px 0 20px 0;
        text-align: left;
        background-color: #303030;
        padding:10px;
        border-radius: 10px;
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