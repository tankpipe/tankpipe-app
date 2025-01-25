<script>
    import EditTransaction from './EditTransaction.svelte'
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import { open } from '@tauri-apps/plugin-dialog'
    import { documentDir } from '@tauri-apps/api/path'
    import { Errors } from './errors'
    import { page, modes, views, isEditMode } from './page'
    import { settings } from './settings'
    import { config } from './config.js'
    import { accounts } from './accounts'
    import { afterUpdate } from 'svelte'
    import { invoke } from "@tauri-apps/api/core"
    import { chart } from "svelte-apexcharts"
    import { onMount } from 'svelte'

    export let curAccount

    let curTransaction
    let errors = new Errors()
    let msg = ""
    let previousAccount
    let topScroll

    $: {
        if (!curAccount && $accounts.length > 0) {
            curAccount = $accounts[0]
        }
        if (curAccount && curAccount !== previousAccount) {
            topScroll = null
            transactions = []
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
        topScroll = document.getElementById("scroller").scrollTop
    }

    const scrollToPosition = () => {
        if (!isEditMode($page)) {
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

    const selectTransaction = (transaction) => {
        setCurrentScroll()
        curTransaction = transaction
        page.set({view: views.TRANSACTIONS, mode: modes.EDIT})
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
            curve: "straight",
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

    let transactions = []
    let chartValues = []

    export const loadTransactions = async () => {
        console.log("loadTransactions: " + curAccount.id)
        transactions = await invoke("transactions", {
            accountId: curAccount.id,
        })
        chartValues = []
        for (const t of transactions) {
            let entry = getEntry(t)
            chartValues.push([new Date(entry.date).valueOf(), entry.balance])
        }
        chartOptions["series"] = [{data: chartValues}]
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
        page.set({view: views.TRANSACTIONS, mode: modes.NEW})
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

</script>
<div class="account-heading">
    {#if !isEditMode($page)}
    <div class="account">
        <Select bind:item={curAccount} items={$accounts} none={settings.require_double_entry} flat={true}/>
    </div>
    <div class="toolbar">
        {#if curAccount}
        <div class="toolbar-icon import-icon" on:click={openFile} title="Import transactions"><Icon icon="mdi:application-import" width="22"/></div>
        {/if}
        <div class="toolbar-icon" on:click="{handleAddClick(curAccount)}" title="Add a transaction"><Icon icon="mdi:plus-box-outline"  width="24"/></div>
    </div>
    {#if transactions.length > 0}
    <div class="chart"><div use:chart={chartOptions}></div></div>
    {/if}
    {/if}
</div>
{#if isEditMode($page)}
<EditTransaction {loadTransactions} {curTransaction}/>
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
<div class="scroller" id="scroller">
    {#if transactions.length > 0}
    <table>
        <tbody>
        <tr><th class="justify-left">Date</th><th class="justify-left">Description</th><th>Debit</th><th>Credit</th><th>Balance</th></tr>
        {#each transactions as t}
            {@const e =  getEntry(t)}
            {#if e}
            <tr on:click={() => selectTransaction(e)} id={t.id}><!--{t.id}-->
                <td class={projected(t) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(t)} title="{e.description}"><div class="description">{e.description}</div>
                    {#each t.entries as en}
                        {#if en.account_id != curAccount.id}
                        <div class="description tiny">{$accounts.find(a => a.id == en.account_id).name}</div>
                        {/if}
                    {/each}
                </td>
                <td class="{projected(t)} money">{getDebitAmount(e, curAccount)}</td>
                <td class="{projected(t)} money">{getCreditAmount(e, curAccount)}</td>
                <td class="{projected(t)} money">{getBalance(e)}</td>
            </tr>
            {/if}
        {/each}
        </tbody>
    </table>
    {/if}
    {#if transactions.length < 1}
    <div class="message">No transactions</div>
    {/if}
</div>
{/if}

<style>
    .scroller{
        height: 100%;
        width: 100%;
        overflow: scroll;
    }

    table {
        padding-right: 10px;
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

    tr:hover td {
        cursor: pointer;
        color: #FFF;
    }

    tr:hover td .tiny{
        cursor: pointer;
        color: #C0C0C0;
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

    .import-icon {
        margin-top: 1px
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
        max-width: 350px;
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