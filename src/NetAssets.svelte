<script>
    import {page, modes, views} from "./page"
    import {config} from './config'
    import {accounts} from './accounts'
    import { invoke } from '@tauri-apps/api/core'

    export let curAccount

    let bsTransactions = {}
    let assetAccounts = []
    let liabilityAccounts = []
    let bsBalances = {}
    let balance = 0

    let assetsTotal = 0
    let liabilitiesTotal = 0
    const today = new Date().setUTCHours(0,0,0,0)

    const loadEmUp = async () => {
        for (const account of $accounts) {
            if (account.account_type == "Asset") {
                assetAccounts.push(account)
                await loadTransactions(account)
            } else if (account.account_type == "Liability") {
                liabilityAccounts.push(account)
                await loadTransactions(account)
            }
        }
    }

    let chartValues = []


    const loadTransactions = async (account) => {
        const accountId = account.id
        console.log("loadTransactions: " + accountId)
        let transactions = await invoke("transactions", {
            accountId: accountId,
        })
        bsBalances[accountId] = curBalance(transactions, account)
        if (account.account_type == "Liability") {
            balance -= Number(bsBalances[accountId])
            liabilitiesTotal += Number(bsBalances[accountId])
        } else if (account.account_type == "Asset") {
            balance += Number(bsBalances[accountId])
            assetsTotal += Number(bsBalances[accountId])
        }
        console.log(balance)
        bsTransactions[accountId] = transactions


        // chartValues = []
        // for (const t of transactions) {
        //     let entry = getEntry(t)
        //     chartValues.push([new Date(entry.date).valueOf(), chartBalance(entry.balance)])
        // }
        // chartOptions["series"] = [{data: chartValues}]
    }

    const curBalance = (transactions, account) => {
        let balance = transactions.length > 0 ? 0 : account.starting_balance
        for (const t of transactions) {
            let entry = getEntry(t, account.id)
            if (new Date(entry.date) < today) {
                balance = entry.balance
            } else {
                break
            }
        }
        return balance
    }

    const getEntry = (transaction, accountId) => {
        return transaction.entries.find(e => e.account_id == accountId)
    }

    const selectAccount = (account) => {
        console.log(account)
        curAccount = account
        page.set({view: views.TRANSACTIONS, mode: modes.LIST})
    }

    const formatter = new Intl.NumberFormat('en-AU', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    })

    const formatAmount = (amount) => {
        return formatter.format(amount)
    }

    const formatDate = (inDate) => {
        const date = new Date(inDate)

        switch ($config.display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return transaction.date
            default: return date.toLocaleDateString()
        }
    }

</script>

<div class="bs-form-heading">Net Assets</div>
<div class="bs-sub-heading">As at {formatDate(today)}</div>
{#await loadEmUp()}
Loading...
{:then _}

<div class="scroller" id="scroller">
    <table>
        <tbody>
            <tr ><th colspan="3" class="justify-left"></th></tr>
            <tr ><th colspan="3" class="justify-left">Assets</th></tr>

        {#each assetAccounts as a}
            <tr id={a.id}><!--{a.id}-->
                <td title="{a.name}"><div class="description" onclick={() => selectAccount(a)}>{a.name}</div></td>
                <td class="money">{formatAmount(bsBalances[a.id])}</td>
                <td class="money"></td>
            </tr>
        {/each}
        <tr ><th colspan="3" class="justify-left"></th></tr>
        <tr class="sub-total">
            <td title="balance"><div class="description indent">Total assets</div></td>
            <td class="money sub-total" >{formatAmount(assetsTotal)}</td>
            <td class="money"></td>
        </tr>


        <tr ><th colspan="3" class="justify-left"></th></tr>
        <tr ><th colspan="3" class="justify-left">Liabilities</th></tr>

        {#each liabilityAccounts as a}
            <tr id={a.id}><!--{a.id}-->
                <td title="{a.name}"><div class="description" onclick={() => selectAccount(a)}>{a.name}</div></td>
                <td class="money"></td>
                <td class="money">{formatAmount(bsBalances[a.id] == 0?bsBalances[a.id]:bsBalances[a.id])}</td>
            </tr>
        {/each}
        <tr ><th colspan="3" class="justify-left"></th></tr>
        <tr class="sub-total">
            <td title="balance"><div class="description indent">Total liabilities</div></td>
            <td class="money"></td>
            <td class="money sub-total" >{formatAmount(liabilitiesTotal)}</td>
        </tr>

        <tr ><th colspan="3" class="justify-left"></th></tr>
        <tr ><th colspan="3" class="justify-left">Balance</th></tr>

        <tr>
            <td title="balance"><div class="description indent">Net assets</div></td>
            <td class="money total">{formatAmount(balance)}</td>
            <td class="money"></td>
        </tr>

        </tbody>
    </table>
</div>
{/await}

<style>
    .scroller{
        height: 100%;
        width: 100%;
        overflow: scroll;
        margin-top: 15px;
    }

    .bs-form-heading {
        font-size: 1.2em;
        font-weight: 500;
        margin: 0px 0 20px 0;
        text-align: left;
        overflow: visible;
        color: #757575;
    }

    .bs-sub-heading {
        font-size: 0.8em;
        font-weight: 500;
        margin: -10px 0 0px 1px;
        text-align: left;
        overflow: visible;
        color: #757575;
    }

    .toolbar {
        float: right;
        color: #C0C0C0;
        margin-left: 10px;
        display: flex;
        padding-right: 9px;
    }



    .scroller{
        height: 100%;
        width: 100%;
        overflow: scroll;
    }

    .sub-total {
        border-top: 1px solid #C0C0C0;
    }

    .total {
        border-top: 1px solid #C0C0C0;
        border-bottom: 1px solid #C0C0C0
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

    .indent {
        padding-left: 2em;
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

</style>