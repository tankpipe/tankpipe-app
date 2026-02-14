<script>
    import { config } from './config.js'
    import { accounts } from './accounts'
    import { _ } from 'svelte-i18n'
    import { selector, toggleSelected, toggleAllSelected, isSelected } from './selector'

    let { curAccount, journalMode = false,  transactions, reconciliationResults = [], isReconciliationMode = false, onSelect } = $props()
    let topScroll

    let displayTransactions = $derived(() => {
        if (!isReconciliationMode || reconciliationResults.length === 0) {
            return transactions
        }

        // Filter reconciliation results that need to be displayed
        const unmatchedResults = reconciliationResults
            .map(result => {
                const transaction = result.transaction
                
                // Extract date from first entry (like existing transactions do)
                return {
                    ...transaction,
                    date: getEntry(transaction).date,
                    isReconciliationResult: true,
                    reconciliationStatus: result.status,
                    balance: result.balance,
                    // Store target transaction ID for matched/partial matches
                    targetTransactionId: result.matched_transaction_id
                }
            })

            console.log(unmatchedResults)

        // Start with existing transactions in their original order
        let combined = transactions.map(tx => ({
            ...tx,
            isReconciliationResult: false,
            reconciliationStatus: null
        }))

        // Sort unmatched results by date to ensure proper insertion order
        unmatchedResults.sort((a, b) => new Date(a.date) - new Date(b.date))

        // Insert each reconciliation result at the correct position
        // Work backwards through the array to avoid index shifting issues
        for (let i = unmatchedResults.length - 1; i >= 0; i--) {
            const reconTx = unmatchedResults[i]
            
            // For matched/partial matched transactions, insert immediately after target
            if (reconTx.targetTransactionId) {
                const targetIndex = combined.findIndex(tx => tx.id === reconTx.targetTransactionId)
                if (targetIndex !== -1) {
                    combined.splice(targetIndex + 1, 0, reconTx)
                    continue
                }
            }
            
            // For unmatched transactions, insert by date
            const insertIndex = combined.findIndex((tx) => {
                return new Date(getEntry(tx).date) >= new Date(reconTx.date)
            })
            
            if (insertIndex === -1) {
                // No later transaction, append to end
                combined.push(reconTx)
            } else {
                // Insert at the correct position
                combined.splice(insertIndex, 0, reconTx)
            }
        }

        return combined
    })

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

    const isReconciled = (entry) => {        
        if (curAccount.reconciliation_info) {
            const reconDate = new Date(curAccount.reconciliation_info.date).getTime()
            const transDate = new Date(entry.date).getTime()
            if (transDate < reconDate) return true
            if (transDate == reconDate) {
                // Find the index of the reconciliation transaction in the transactions array
                const reconIndex = transactions.findIndex(trans => trans.id === curAccount.reconciliation_info.transaction_id)
                const transIndex = transactions.findIndex(trans => trans.id === entry.transaction_id)
                return transIndex <= reconIndex                    
            }
        }
        return false
    }

    const getReconciliationStatus = (transaction) => {
        if (!isReconciliationMode) return ''
        
        if (transaction.isReconciliationResult) {
            switch (transaction.reconciliationStatus) {
                case 'Matched': return '🟢'
                case 'PartialMatch': return '🟡'
                case 'Unmatched': return '🔴'
                default: return ''
            }
        }
        
        return ''
    }

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
            <th class="justify-left">{$_('labels.date')}</th><th class="justify-left">{$_('labels.description')}</th><th>Debit</th><th>Credit</th>{#if !journalMode}<th>Balance</th>{/if}<th>Reconciled</th>{#if isReconciliationMode}<th>Match Status</th>{/if}
        </tr>
        {#each displayTransactions() as t}
            {@const selected = isSelected(t)}
          {#if !journalMode}
            {@const e =  getEntry(t)}
            {#if e}
            <tr class="{selected ? 'selected' : ''} {t.entries.length == 1 ? 'single-entry' : ''}"  onclick={(event) => stopPropagationHandler(event, () => selectTransaction(e))} id={t.id}><!--{t.id}-->
                {#if $selector.showMultipleSelect}<td onclick={(event) => stopPropagationHandler(event, () => handleToggleSelected(t))}><input id={"selected_" + t.id} type=checkbox checked={selected}></td>{/if}
                <td class={projected(e) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(e)} title="{e.description}"><div class="description">{e.description}</div>
                    {#each t.entries as en}
                        {#if en.account_id != curAccount.id}
                        <div class="description tiny">{$accounts.find(a => a.id == en.account_id).name}</div>
                        {/if}
                    {/each}
                </td>
                <td class="{projected(e)} money">{getDebitAmount(e)}</td>
                <td class="{projected(e)} money">{getCreditAmount(e)}</td>
                <td class="{projected(e)} money">{getBalance(e)}</td>
                <td class="reconciled-cell">{isReconciled(e) ? '✓' : ''}</td>
                {#if isReconciliationMode}
                <td class="reconciliation-status-cell">{getReconciliationStatus(t)}</td>
                {/if}
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
                <td class={projected(e) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(e)} style="{e.entry_type == 'Credit' ? 'padding-left: 30px' : ''}" title="{e.description}">
                    <div class="description">{$accounts.find(a => a.id == e.account_id).name}</div>
                    <div class="description tiny">{e.description}</div>
                </td>
                <td class="{projected(e)} money">{getDebitAmount(e)}</td>
                <td class="{projected(e)} money">{getCreditAmount(e)}</td>
                <td class="reconciled-cell">{isReconciled(e) ? '✓' : ''}</td>
                {#if isReconciliationMode}
                <td class="reconciliation-status-cell">{getReconciliationStatus(t)}</td>
                {/if}
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

    .reconciled-cell {
        background-color: #444 !important;
        font-size: .8em;
        font-weight: bold;
        padding: 0 0 4px 3px;
    }

    .reconciliation-status-cell {
        text-align: center !important;
        min-width: 80px;
        font-weight: bold;
        font-size: 1.2em;
    }    

    @media (min-width: 1010px) {
        .description {
            max-width: calc(70vw - 350px);
        }
    }

</style>