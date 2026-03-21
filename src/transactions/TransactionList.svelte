<script>
    import { config } from '../stores/config.js'
    import MessagePanel from '../components/MessagePanel.svelte'
    import { accounts } from '../stores/accounts.js'
    import { _ } from 'svelte-i18n'
    import { selector, toggleSelected, toggleAllSelected, isSelected } from './selector.js'
    import { invoke } from "@tauri-apps/api/core"
    import Icon from '@iconify/svelte'
    import { Errors } from '../utils/errors.js'
    import { ReconciliationMode as RM } from './reconciliation.js'

    let { curAccount, journalMode = false,  transactions, reconciliationResults = [], reconciliationMode = RM.NONE, onSelect, loadAccounts, rerunReconciliationIfNeeded, topScroll, setTopScroll, descriptionFilter = "" } = $props()
    let hoveredReconIndex = $state(null)
    let errors = $state(new Errors())
    let msg = $state("")
    let mergeTransaction = $state(null)
    let mergeReconTransaction = $state(null)
    
    let firstReconciledDate = $derived.by(() => {     
        return displayTransactions().find(t => t.isReconciliationResult)?.date ?? null
    })

    let lastReconciledDate = $derived.by(() => {
       return displayTransactions().findLast(t => t.isReconciliationResult)?.date ?? null
    })

    let displayTransactions = $derived(() => {
        console.log("displayTransactions", reconciliationMode)
        
        if (reconciliationMode !== RM.GUIDED || reconciliationResults.length === 0 || journalMode) {
            return transactions
        }

        let targetsToReconciliationMap = new Map()
        let previousTransaction = null

        // Filter reconciliation results that need to be displayed
        let results = reconciliationResults            
            .map(abstractResult => {
                let isReconciliation = abstractResult.Reconciliation !== undefined
                let result = isReconciliation ? abstractResult.Reconciliation : abstractResult.Original
                const transaction = result.transaction
                targetsToReconciliationMap.set(result.matched_transaction_id, result)                
                // Extract date from first entry (like existing transactions do)
                let item = {
                    ...transaction,
                    date: getEntry(transaction).date,
                    isReconciliationResult: isReconciliation,
                    reconciliationStatus: result.status,
                    balance: result.balance,
                }

                if (isReconciliation) {
                    item.targetTransactionId = result.matched_transaction_id
                    if (previousTransaction?.id === result.matched_transaction_id) {
                        item.targetReconciledStatus = getEntry(previousTransaction)?.reconciled_status
                    } 
                } else {
                    item.matchedReconciliationId = result.matched_reconciliation_id
                    previousTransaction = transaction
                }

                return item
            })

        return results.filter(filterMatchTransaction)
    })
    
    const filterMatchTransaction = (transaction) => {
        if (!descriptionFilter || descriptionFilter === "") return true
        const filterValue = descriptionFilter.toLowerCase()
        if (journalMode) {
            return transaction.entries.some(e => e.description?.toLowerCase().includes(filterValue))
        }
        const entry = getEntry(transaction)
        return entry?.description?.toLowerCase().includes(filterValue)        
    }

    $effect(() => {
        if (topScroll === null || topScroll === undefined) {
            let targetTransaction
            
            if (reconciliationMode === RM.GUIDED) {
                targetTransaction = findLastReconciledTransaction()
            } else {
                targetTransaction = findClosestTransaction()
            }
            
            if (targetTransaction) {
                setTopScroll(getScrollPosition(targetTransaction.id))
            }
        }
        scrollToPosition()
    });

    const setCurrentScroll = () => {
        setTopScroll(document.getElementById("scroller").scrollTop)
    }

    const scrollToPosition = () => {
        const scroller = document.getElementById("scroller")
        if (scroller) {
            const target = topScroll ?? 0
            if (typeof scroller.scrollTo === 'function') {
                scroller.scrollTo(0, target)
            } else {
                scroller.scrollTop = target
            }
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

    const findLastReconciledTransaction = () => {
        const transactionsToCheck = displayTransactions() ?? []

        for (let i = transactionsToCheck.length - 1; i >= 0; i--) {
            const t = transactionsToCheck[i]
            const entry = getEntry(t)
            if (!t.isReconciliationResult && entry && entry.reconciled_status) {
                return t
            }
        }

        console.log("No reconciled transactions found")
        return null
    }

    const findClosestTransaction = () => {
        const today = new Date().setHours(0, 0, 0, 0)
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

    const reconcilationTargetAlreadyReconciled = (transaction) => {
        return transaction.isReconciliationResult && transaction.targetReconciledStatus == 'Reconciled'
    }

    const reconcileTransactions = async (toEntry) => {
        setCurrentScroll()
        if (!curAccount || !curAccount.id || !toEntry) return

        const displayTxns = displayTransactions()
        const transactionIds = displayTxns
            .slice(0, displayTxns.findIndex(t => t.isReconciliationResult && t.targetTransactionId === toEntry.transaction_id) + 1)
            .filter(t => t.isReconciliationResult && t.targetReconciledStatus != 'Reconciled' && (t.reconciliationStatus == 'Matched' || t.reconciliationStatus == 'PartialMatch'))
            .map(t => t.targetTransactionId)

        console.log("reconciling", transactionIds)
        await invoke('reconcile_account_transactions', {accountId: curAccount.id, transactionIds: transactionIds}).then(resolvedReconcile, rejectedReconcile)            
        await loadAccounts()
        if (rerunReconciliationIfNeeded) {
            await rerunReconciliationIfNeeded()
        }
    }

    const resolvedReconcile = () => {}

    const rejectedReconcile = (result) => {
        console.log(result)
        errors = new Errors()
        errors.addError("all", $_('transactions.error', { values: {error: result} }))
        showErrors(errors)
    }

    const mergeTransactions = (t) => {
        setCurrentScroll()
        if (t.isReconciliationResult) {
            if (mergeReconTransaction && mergeReconTransaction.id == t.id) {
                mergeReconTransaction = null
            } else {
                mergeReconTransaction = t
            }
        } else {
            if (mergeTransaction && mergeTransaction.id == t.id) {
                mergeTransaction = null
            } else {
                mergeTransaction = t
            }
        }
        if (mergeTransaction && mergeReconTransaction) {
            let editIntent = {
                ...mergeTransaction,
                prefillEdit: true,
                editPatch: mergeReconTransaction,
                targetTransactionId: mergeTransaction.id,
                mergeAccountId: curAccount?.id
            }
            selectTransaction(editIntent)
            mergeTransaction = null
            mergeReconTransaction = null
        }
    }

    const manualReconcile = (e) => async () => {
        setCurrentScroll()
        console.log("manualReconcile", e)
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
       
    const isOrphan = (t, e) => {
        return reconciliationMode === RM.GUIDED && !t.isReconciliationResult && 
               !isReconciled(e) && t.reconciliationStatus === 'Unmatched' && 
               e.date >= firstReconciledDate && e.date <= lastReconciledDate 
    }

    const isHovered = (i) => hoveredReconIndex !== null && i <= hoveredReconIndex    

    const isSelectedForMerge = (t_id) => {
        return mergeTransaction && mergeTransaction.id == t_id || mergeReconTransaction && mergeReconTransaction.id== t_id
    }

    const MERGE_WINDOW_MARGIN = 14 * 24 * 60 * 60 * 1000
    const inMergeWindow = (e) => {
        return new Date(firstReconciledDate).getTime() - MERGE_WINDOW_MARGIN <= new Date(e.date).getTime() && 
               new Date(lastReconciledDate).getTime() + MERGE_WINDOW_MARGIN >= new Date(e.date).getTime() 
    }

    const transactionCanBeReconciled = (t) => {
        return !t.isReconciliationResult && 
            (t.reconciliationStatus == 'Matched' || 
            (t.reconciliationStatus == 'PartialMatch' && !isSelectedForMerge(t.matchedReconciliationId)))
    }

    const availableForMerge = (t, e) => {
        return t.reconciliationStatus != 'Matched' && inMergeWindow(e) && !reconcilationTargetAlreadyReconciled(t)
    }

    /**
     * Determines what type of widget to display in the reconciliation column.
     */
    const getReconciledCellType = (t, e) => {
        if (e.reconciled_status == 'Reconciled') {
            return 'reconciled'
        }

        if (reconciliationMode === RM.GUIDED) {

            if (transactionCanBeReconciled(t)) {
                return 'reconcilable'
            }

            if (availableForMerge(t, e)) {
                return 'merge'                    
            }
        
        } else if (reconciliationMode === RM.MANUAL) {
            return 'manual-reconcile'
        }
        
        if (e.reconciled_status == 'Outstanding') {
            return 'outstanding'
        }
        
        return 'empty'
    }
   
</script>

<MessagePanel {errors} {msg} />
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
            {@const isReconciliationRow = reconciliationMode === RM.GUIDED && t.isReconciliationResult}
          {#if !journalMode}
            {@const e =  getEntry(t)}
            {#if e}
                {@const reconciledContent = getReconciledCellType(t, e)}
                <tr class="{selected ? 'selected' : ''} {t.entries.length == 1 ? 'single-entry' : ''} {isReconciliationRow ? 'reconciliation-row reconciliation-row-' + (t.reconciliationStatus?.toLowerCase() || '') : ''} {isReconciliationRow && reconcilationTargetAlreadyReconciled(t) ? ' reconciled-recon-row' : ''} {isOrphan(t, e)? 'orphan-row' : ''}" 
                    onclick={true ? (event) => stopPropagationHandler(event, () => e && selectTransaction(t)) : undefined} 
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
                <td class="reconciled-cell" onclick={(event) => stopPropagationHandler(event, () => {})}>
                    {#if reconciledContent === 'reconciled'}
                        <div class="recon-status"><Icon icon="mdi:check" width="16"/></div>
                    {:else if reconciledContent === 'reconcilable'}
                        <button
                            class={"recon-marker " + (isHovered(i) ? " hover-highlight" : "")}
                            onclick={(event) => stopPropagationHandler(event, () => {if (t.reconciliationStatus == 'Matched') reconcileTransactions(e)})}
                            onmouseenter={() => {if (t.reconciliationStatus == 'Matched') hoveredReconIndex = i}}
                            onmouseleave={() => hoveredReconIndex = null}
                            title={$_('transaction.reconcileTransactions')}
                        ><Icon icon="mdi:check" width="16"/></button>    
                    {:else if reconciledContent === 'merge'}
                        <button class={"merge-marker " + (isSelectedForMerge(t.id) ? "merge-marker-selected" : "")} onclick={(event) => stopPropagationHandler(event, () => mergeTransactions(t))}>
                            {#if isSelectedForMerge(t.id)}<Icon icon="mdi:merge" width="16"/>{:else}<Icon icon="mdi:square-outline" width="16"/>{/if}
                        </button>                    
                    {:else if reconciledContent === 'manual-reconcile'}
                        <button
                            class="recon-marker "
                            onclick={(event) => { event.stopPropagation(); manualReconcile(e)()}}
                            title={$_('transaction.reconcileTransactions')}
                        ><Icon icon="mdi:check" width="16"/></button>
                    {:else if reconciledContent === 'outstanding'}
                        <div class="recon-status"><Icon icon="mdi:circle-small" width="16"/></div>
                    {/if}
                </td>
            </tr>
            {#if reconciliationMode === RM.GUIDED && (t.isReconciliationResult || t.matchedReconciliationId === null)}
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
                <td class="reconciled-cell recon-status">{#if isReconciled(e)}<Icon icon="mdi:check" width="16"/>{:else if e.reconciled_status == 'Outstanding'}<Icon icon="mdi:circle-small" width="16"/>{/if}</td>
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
        color: var(--color-table-cell-text);
        background-color: var(--color-table-cell-bg);
        padding: 8px;
        white-space: nowrap;
        font-size: 0.9em;
    }

    .recon-status-reconciled {
        color: var(--color-recon-accent);
    }

    .recon-status-outstanding {
        color: var(--color-recon-accent);
    }

    .projected {
        color: var(--color-text-dim);
        background-color: var(--color-bg-dim);
    }

    th {
        color:var(--color-border);
        background-color: var(--color-bg);
        font-weight: 400;
        font-size: .8em;
    }
    

    .scroller tr:hover td {
        cursor: pointer;
        color: var(--color-text-strong);
    }

    tr:hover td .tiny{
        cursor: pointer;
        color: var(--color-text-muted);
    }

    .journal tr:last-child  {
        display: none;
    }

    .selected td {
        background-color: var(--color-table-selected-bg);
        color: var(--color-table-selected-text);
    }

    .money {
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
        color: var(--color-text-dim);
        margin: 3px 0 -5px 2px;
    }

    
    
    .message {
        margin: 5px 0 20px 0;
    }

    .errors {
        padding-bottom: 10px;
    }

    .error-msg {
        color: var(--color-error);
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
    }

    .messages .success-msg {
        color: var(--color-success);
        text-align: left;
        font-size: 0.85em;
    }

    .reconciled-cell {
        width: 30px;
        min-width: 30px;
        height: 100%;
        cursor: default !important;
        background-color: transparent;
        padding: 0;
    }

    .reconciled-cell div {
        margin-left: 2px;
    }

    .merge-marker {
        width: 12px !important;
        height: 12px !important;
        background: transparent;
        padding: 0 0 0 0px;
        border: none;
        cursor: pointer;
        color: var(--color-border);
        margin-left: 1px;
    }

    .merge-marker:hover, .merge-marker-selected {
        color: var(--color-success-strong);
    }

    .merge-marker-selected {
        margin-left: 2px;
    }

    .recon-marker {
        border: none;
        background: transparent;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        padding: 0 0 0 0px;
        cursor: pointer;
        border-radius: 50%;
        border: 1px solid var(--color-border);
        color: transparent;
        width: 12px !important;
        height: 12px !important;
        margin: 2px 0 0 4px !important;
    }

    .reconciled-cell button {
        width: 19px;
        margin-left: 2px;
    }

    .recon-marker:hover {
        cursor: default
    }

    .recon-marker.hover-highlight {
        border-color: transparent;
        background: transparent;
        color: var(--color-success-strong);
        box-shadow: none;
        font-weight: bold;
        width: 30px !important;
        /* height: 24px !important; */
        margin-left: -6px !important;
    }

    .recon-check {
        margin-left: 0px;
        color: var(--color-text-muted);
        font-weight: bold;
    }

    .reconciliation-status-cell {
        text-align: center !important;
        min-width: 80px;
        font-weight: bold;
        font-size: 1.2em;
    }   
    
    .divider-row {
        background-color: var(--color-bg);     
    }

    .reconciliation-row-matched td {
        color: var(--color-success-strong);
    }   

    .reconciliation-row-partialmatch td, .reconciliation-row-mismatch td {
        color: var(--color-accent);
    }

    .reconciliation-row-unmatched td {
        color: var(--color-error-muted);
    }

    .reconciliation-row td {
        background-color: var(--color-table-recon-row-bg);
        font-size: .7em;
        line-height: .8em;
    }

    .reconciliation-row .reconciled-cell {
        background-color: transparent;
    }

    .reconciled-recon-row td{
        color: var(--color-muted-strong);
    }

    .reconciliation-row-matched:hover td {
        cursor: default !important;
        color: var(--color-success-strong) !important;
    }

    .reconciliation-row-partialmatch:hover td, .reconciliation-row-mismatch:hover td  {
        cursor: default !important;
        color: var(--color-accent) !important;
    }

    .reconciliation-row-unmatched:hover td {
        cursor: default !important;
        color: var(--color-error-muted) !important;
    }

    .reconciled-recon-row:hover td {
        cursor: default !important;
        color: var(--color-muted-strong) !important;
    }

    .orphan-row td {
        border-bottom: 1px solid var(--color-error-border);        
    }

    .orphan-row td:last-child {
        border: none;
    }

    .error-msg {
        color: var(--color-error);
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
    }

    .success-msg {
        color: var(--color-success);
        text-align: left;
    }

    @media (min-width: 1010px) {
        .description {
            max-width: calc(70vw - 350px);
        }
    }

</style>
