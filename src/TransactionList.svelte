<script>
    import { config } from './config.js'
    import { accounts } from './accounts'
    import { _ } from 'svelte-i18n'
    import { selector, toggleSelected, toggleAllSelected, isSelected } from './selector'

    let { curAccount, journalMode = false,  transactions, onSelect } = $props()
    let topScroll

    $effect(() => {

            if (!topScroll) {
                const closest = findClosestTransaction()
                if (closest) {
                    topScroll = getScrollPosition(closest.id)
                }
            }
            scrollToPosition()
    });

    const setCurrentScroll = () => {
        topScroll = document.getElementById("scroller").scrollTop
    }

    const scrollToPosition = () => {
        const scroller = document.getElementById("scroller")
        if (scroller) {
            scroller.scrollTo(0, topScroll)
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
        onSelect(entry)
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

    const getDebitAmount = (transaction) => {
        return transaction.entry_type === "Debit" ? formatter.format(transaction.amount) : ''
    }

    const getCreditAmount = (transaction) => {
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

    const projected = (t) => t.status == 'Projected' ? 'projected' : ''
    const date_class = date_style()

    const sortEntries = (entries) => {
        return entries.toSorted((a, b) => {
            if (a.entry_type === "Debit" && b.entry_type === "Credit") return -1
            if (a.entry_type === "Credit" && b.entry_type === "Debit") return 1
            return 0
        })
    }

    const handleToggleSelected = (t) => {
        toggleSelected(t)
        setCurrentScroll()
    }

    const stopPropagationHandler = (event, handler) => {
        event.stopPropagation()
        handler()
    }

</script>

<div class="scroller" id="scroller">
    <table class="{journalMode ? 'journal' : ''}">
        <tbody>
        <tr>
            {#if $selector.showMultipleSelect}
            <th onclick={(event) => stopPropagationHandler(event, () => toggleAllSelected(transactions))}><input id="selectAll" type=checkbox checked={$selector.isSelectAll}></th>
            {/if}
            <th class="justify-left">{$_('labels.date')}</th><th class="justify-left">{$_('labels.description')}</th><th>Debit</th><th>Credit</th>{#if !journalMode}<th>Balance</th>{/if}
        </tr>
        {#each transactions as t}
            {@const selected = isSelected(t)}
          {#if !journalMode}
            {@const e =  getEntry(t)}
            {#if e}
            <tr class="{selected ? 'selected' : ''} {t.entries.length == 1 ? 'single-entry' : ''}"  onclick={(event) => stopPropagationHandler(event, () => selectTransaction(e))} id={t.id}><!--{t.id}-->
                {#if $selector.showMultipleSelect}<td onclick={(event) => stopPropagationHandler(event, () => handleToggleSelected(t))}><input id={"selected_" + t.id} type=checkbox checked={selected}></td>{/if}
                <td class={projected(t) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(t)} title="{e.description}"><div class="description">{e.description}</div>
                    {#each t.entries as en}
                        {#if en.account_id != curAccount.id}
                        <div class="description tiny">{$accounts.find(a => a.id == en.account_id).name}</div>
                        {/if}
                    {/each}
                </td>
                <td class="{projected(t)} money">{getDebitAmount(e)}</td>
                <td class="{projected(t)} money">{getCreditAmount(e)}</td>
                <td class="{projected(t)} money">{getBalance(e)}</td>
            </tr>
            {/if}
          {/if}
          {#if journalMode}
            {@const sortedEntries = sortEntries(t.entries)}
            {#each sortedEntries as e}
            <tr class="{selected ? 'selected' : ''} {t.entries.length == 1 ? 'single-entry' : ''}" onclick={() => selectTransaction(e)} id={t.id}><!--{t.id}-->
                {#if $selector.showMultipleSelect}
                <td onclick={(event) => stopPropagationHandler(event, () => handleToggleSelected(t))}><input id={"selected_" + t.id} type=checkbox checked={selected}></td>
                {/if}
                <td class={projected(t) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(t)} style="{e.entry_type == 'Credit' ? 'padding-left: 30px' : ''}" title="{e.description}">
                    <div class="description">{$accounts.find(a => a.id == e.account_id).name}</div>
                    <div class="description tiny">{e.description}</div>
                </td>
                <td class="{projected(t)} money">{getDebitAmount(e)}</td>
                <td class="{projected(t)} money">{getCreditAmount(e)}</td>
            </tr>
            {/each}
            <tr style="height: 8px;"></tr>
          {/if}
        {/each}
        </tbody>
    </table>
    {#if transactions.length < 1}
    <div class="message">{$_('transactions.noTransactions')}</div>
    {/if}
</div>


<style>
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

    .journal tr:last-child  {
        display: none;
    }

    .selected td {
        background-color: #1a3924;
        color: #e3e3e3;
    }

    .single-entry td {
        background-color: #34391a;
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