<script>
    import { config } from '../config.js'
    import { accounts } from '../accounts'
    import { _ } from 'svelte-i18n'
    import { selector, toggleSelected, toggleAllSelected, isSelected } from '../selector.js'
    import { invoke } from "@tauri-apps/api/core"
    import Icon from '@iconify/svelte'
    import { Errors } from '../errors'

    let { curAccount, journalMode = false,  transactions, reconciliationResults = [], isReconciliationMode = false, manualReconciliationMode = false, onSelect, loadAccounts } = $props()
    let topScroll
    let hoveredReconIndex = $state(null)
    let errors = $state(new Errors())
    let msg = $state("")

    let displayTransactions = $derived(() => {
        // If not in reconciliation mode or no reconciliation results, return normal transactions
        if (!isReconciliationMode || reconciliationResults.length === 0 || journalMode) {
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

        // Start with existing transactions in their original order
        let combined = transactions.map(tx => ({
            ...tx,
            isReconciliationResult: false,
            reconciliationStatus: null
        }))

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

    const selectTransaction = (transaction) => {
        setCurrentScroll()
        onSelect(transaction)
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

    const getDebitAmount = (entry) => {
        return entry.entry_type === "Debit" ? formatter.format(entry.amount) : ''
    }

    const getCreditAmount = (entry) => {
        return entry.entry_type === "Credit" ? formatter.format(entry.amount) : ''
    }

    const getBalance = (entry) => {
        return formatter.format(entry.balance)
    }

    const getDate = (entry) => {
        const date = new Date(entry.date)

        switch ($config.display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return entry.date
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
        return entry.reconciled_status == 'Reconciled'
    }

    const noReconciledStatus = (transaction) => {                
        return !transaction.entries.some(e => e.reconciled_status)
    }

    const hasReconciliationMatch = (entry) => {
        if (!isReconciliationMode || reconciliationResults.length === 0) {
            return false
        }

        return reconciliationResults.some(result =>
            result.matched_transaction_id === entry.transaction_id &&
            result.status !== 'Unmatched'
        )
    }

    const canBeReconciled = (entry) => {
        if (!isReconciliationMode || reconciliationResults.length === 0) {
            return false
        }

        return reconciliationResults.some(result =>
            result.matched_transaction_id === entry.transaction_id &&
           (result.status == 'Matched' || result.status == 'PartialMatch')
        )
    }

    const reconcilationTargetAlreadyReconciled = (transaction) => {
        if (!transaction || !transaction.targetTransactionId) return false
        
        const targetTransaction = transactions.find(t => t.id === transaction.targetTransactionId)
        if (!targetTransaction) return false
        
        return isReconciled(getEntry(targetTransaction))
    }

    const reconcileTransactions = async (toEntry) => {
        if (!curAccount || !curAccount.id || !toEntry) return

        try {
            const transactionIds = reconciliationResults
                .slice(0, reconciliationResults.findIndex(result => result.matched_transaction_id === toEntry.transaction_id) + 1)
                .filter(result => result.status == 'Matched' || result.status == 'PartialMatch')
                .map(result => result.matched_transaction_id)

            await invoke('reconcile_account_transactions', {accountId: curAccount.id, transactionIds: transactionIds})            
        } catch (err) {
            console.log(err)
            errors = new Errors()
            errors.addError("all", $_('transactions.error', { values: {error: err} }))
            showErrors(errors)
        }
        loadAccounts()        
    }

    const sortEntries = (entries) => {
        return entries.toSorted((a, b) => {
            if (a.entry_type === "Debit" && b.entry_type === "Credit") return -1
            if (a.entry_type === "Credit" && b.entry_type === "Debit") return 1
            return 0
        })
    }

    const handleToggleSelected = (t) => {
        if (noReconciledStatus(t)) {
            toggleSelected(t)
            setCurrentScroll()
        }        
    }

    const stopPropagationHandler = (event, handler) => {
        event.stopPropagation()
        if (typeof handler === 'function') {
            handler()
        } else {
            console.error('Handler is not a function:', handler)
        }
    }

</script>
{#if errors.getErrorMessages().length > 0 || msg && msg != ""}
<div class="widget errors">
    {#each errors.getErrorMessages() as e}
    <div class="error-msg selectable-text">{e}</div>
    {/each}
    {#if msg}
    <div class="success-msg selectable-text">{msg}</div>
    {/if}
</div>
{/if}
<div class="scroller" id="scroller">
    <table class="{journalMode ? 'journal' : ''}">
        <tbody>
        <tr>
            {#if $selector.showMultipleSelect}
            <th onclick={(event) => stopPropagationHandler(event, () => toggleAllSelected(transactions.filter(t => noReconciledStatus(t))))}><input id="selectAll" type=checkbox checked={$selector.isSelectAll}></th>
            {/if}
            <th class="justify-left">{$_('labels.date')}</th>
            <th class="justify-left">{$_('labels.description')}</th>
            <th>Debit</th>
            <th>Credit</th>
            {#if !journalMode}<th>Balance</th>{/if}
            <th></th>
        </tr>
        {#each displayTransactions() as t, i}
            {@const selected = isSelected(t)}
            {@const isReconciliationRow = isReconciliationMode && t.isReconciliationResult}
          {#if !journalMode}
            {@const e =  getEntry(t)}
            {#if e}
                <tr class="{selected ? 'selected' : ''} {t.entries.length == 1 ? 'single-entry' : ''} {isReconciliationRow ? 'reconciliation-row reconciliation-row-' + (t.reconciliationStatus?.toLowerCase() || '') : ''} {isReconciliationRow && reconcilationTargetAlreadyReconciled(t) ? ' reconciled-recon-row' : ''}" 
                    onclick={!isReconciliationRow ? (event) => stopPropagationHandler(event, () => e && selectTransaction(t)) : undefined} 
                    id={t.id}><!--{t.id}-->
                {#if $selector.showMultipleSelect}<td onclick={(event) => stopPropagationHandler(event, () => handleToggleSelected(t))}>{#if noReconciledStatus(t)}<input id={"selected_" + t.id} type=checkbox checked={selected}>{/if}</td>{/if}
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
                {#if isReconciliationRow}
                    <td class="reconciled-cell"></td>
                {:else if isReconciled(e) }
                    <td class="reconciled-cell"><Icon icon="mdi:check" width="16"/></td>
                {:else if isReconciliationMode && canBeReconciled(e)}
                        <td class="reconciled-cell">
                            <button
                                class={"recon-marker " + (hoveredReconIndex !== null && i <= hoveredReconIndex ? " hover-highlight" : "")}
                                onclick={(event) => stopPropagationHandler(event, () => reconcileTransactions(e))}
                                onmouseenter={() => hoveredReconIndex = i}
                                onmouseleave={() => hoveredReconIndex = null}
                                title={$_('transaction.reconcileTransactions')}
                            >✓</button>
                        </td>
                {:else if manualReconciliationMode && !isReconciled(e)}
                    <td class="reconciled-cell">
                        <button
                            class="recon-marker"
                            onclick={(event) => stopPropagationHandler(event, async () => {
                                if (!curAccount || !curAccount.id) return
                                try {
                                    await invoke('reconcile_account_transactions', {
                                        accountId: curAccount.id,
                                        transactionIds: [e.transaction_id]
                                    })
                                } catch (err) {
                                    console.log(err)
                                    errors = new Errors()
                                    errors.addError("all", $_('transactions.error', { values: {error: err} }))
                                }
                                loadAccounts()
                            })}
                            title={$_('transaction.reconcileTransactions')}
                        >✓</button>
                    </td>
                {:else if e.reconciled_status == 'Outstanding' }
                    <td class="reconciled-cell"><Icon icon="mdi:circle-small" width="16"/></td>
                {:else}
                    <td class="reconciled-cell"></td>
                {/if}
            </tr>
            {#if isReconciliationMode && ! hasReconciliationMatch(e)}
            <tr>
                <td colspan="7" class="divider-row"></td>
            </tr>
            {/if}
            {/if}
          {/if}
          {#if journalMode}
            {@const sortedEntries = sortEntries(t.entries)}
            {#each sortedEntries as e}
            <tr class="{selected ? 'selected' : ''} {t.entries.length == 1 ? 'single-entry' : ''}" onclick={() => selectTransaction(t)} id={t.id}><!--{t.id}-->
                {#if $selector.showMultipleSelect}
                <td onclick={(event) => stopPropagationHandler(event, () => handleToggleSelected(t))}>{#if noReconciledStatus(t)}<input id={"selected_" + t.id} type=checkbox checked={selected}>{/if}</td>
                {/if}
                <td class={projected(t) + ' ' + date_class}>{getDate(e)}</td>
                <td class={projected(t)} style="{e.entry_type == 'Credit' ? 'padding-left: 30px' : ''}" title="{e.description}">
                    <div class="description">{$accounts.find(a => a.id == e.account_id).name}</div>
                    <div class="description tiny">{e.description}</div>
                </td>
                <td class="{projected(t)} money">{getDebitAmount(e)}</td>
                <td class="{projected(t)} money">{getCreditAmount(e)}</td>
                <td class="reconciled-cell">{#if isReconciled(e)}<Icon icon="mdi:check" width="16"/>{:else if e.reconciled_status == 'Outstanding'}<Icon icon="mdi:circle-small" width="16"/>{/if}</td>
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
        /* background-color: #34391a; */
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
        text-align: center;
    }

    .recon-marker {
        width: 12px;
        height: 12px;
        border-radius: 50%;
        border: 1px solid #666;
        background: transparent;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 0;
        cursor: pointer;
        color: transparent;
        font-size: 12px;
        line-height: 12px;
    }

    .recon-marker.past {
        background: #4CAF50;
        border-color: #4CAF50;
        opacity: 0.5;
    }

    .recon-marker.current {
        background: #4CAF50;
        border-color: #4CAF50;
        box-shadow: 0 0 0 2px #222, 0 0 0 3px #4CAF50;
    }

    .recon-marker.future {
        background: transparent;
        border-color: #666;
    }

    .recon-marker:hover {
        border-color: #9be29f;
    }

    .recon-marker.hover-highlight {
        border-color: transparent;
        background: transparent;
        color: #4CAF50;
        box-shadow: none;
        font-weight: bold;
        font-size: 14px;
    }

    .recon-check {
        margin-left: 6px;
        color: #c0c0c0;
        font-weight: bold;
    }

    .reconciliation-status-cell {
        text-align: center !important;
        min-width: 80px;
        font-weight: bold;
        font-size: 1.2em;
    }   
    
    .divider-row {
        background-color: #444;     
    }

    .reconciliation-row-matched td {
        color: #74d965;
    }   

    .reconciliation-row-partialmatch td, .reconciliation-row-mismatch td {
        color: #daae3e;
    }

    .reconciliation-row-unmatched td {
        color: #e2634f;
    }

    .reconciliation-row td {
        background-color: #2a2a2a;
        font-size: .7em;
        line-height: .8em;
    }

    .reconciled-recon-row td{
        color: #888;
    }

    .reconciliation-row-matched:hover td {
        cursor: default !important;
        color: #74d965 !important;
    }

    .reconciliation-row-partialmatch:hover td, .reconciliation-row-mismatch:hover td  {
        cursor: default !important;
        color: #daae3e !important;
    }

    .reconciliation-row-unmatched:hover td {
        cursor: default !important;
        color: #e2634f !important;
    }

    .reconciled-recon-row:hover td {
        cursor: default !important;
        color: #888 !important;
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
