<script>
    import Select from '../components/Select.svelte'
    import Icon from '@iconify/svelte'
    import MessagePanel from '../components/MessagePanel.svelte'
    import { open } from '@tauri-apps/plugin-dialog'
    import { documentDir } from '@tauri-apps/api/path'
    import { Errors } from '../utils/errors'
    import { page } from '../stores/page'
    import { config } from '../stores/config.js'
    import { accounts } from '../stores/accounts'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'

    let { curAccount, onClose, onReconciliationResults = null } = $props()

    let errors = $state(new Errors())
    let msg = $state("")
    let rows = $state([])
    let columnTypes = $state([])
    let selectedColumns = $state([])
    let columns = $state([])
    let requiredColumnsMatched = $state(false);
    let mappingExists = $state(false);
    let path = $state("")
    let fileDialogShown = $state(false)
    let hasHeaderRow = $state(true)
    let rememberForNextTime = $state(true)
    let showReverseDrCrMsg = $state(false)
    let originalDrCrColumns = $state([])

    const DATE_FORMATS = [{value: "Locale", name:"Locale default"}, {value: "Regular", name: "Regular (D/M/Y)", format: "%d/%m/%Y"}, {value: "US", name:"US (M/D/Y)", format: "%m/%d/%Y"}, {value: "ISO", name:"ISO (Y-M-D)", format: "%Y-%M-%D"} ]
    const COLUMN_TYPES_MAP = {
        "Date": {name: $_('labels.date'), id: "Date"},
        "Description": {name: $_('labels.description'), id: "Description"},
        "Credit": {name: $_('labels.credit'), id: "Credit"},
        "Debit": {name: $_('labels.debit'), id: "Debit"},
        "Amount": {name: $_('labels.amount'), id: "Amount"},
        "Balance": {name: $_('labels.balance'), id: "Balance"},
        "Unknown": {name: "", id: "Unknown"}
    }

    const COLUMN_TYPES = [
        COLUMN_TYPES_MAP["Date"],
        COLUMN_TYPES_MAP["Description"],
        COLUMN_TYPES_MAP["Credit"],
        COLUMN_TYPES_MAP["Debit"],
        COLUMN_TYPES_MAP["Amount"],
        COLUMN_TYPES_MAP["Balance"],
        COLUMN_TYPES_MAP["Unknown"]
    ]

    const hasSelectedColumn = (id) => selectedColumns.some(col => col?.id === id)

    $effect(() => {
        if ((!curAccount || !curAccount.id) && $accounts.length > 0) {
            curAccount = $accounts[0]
        }

        console.log($page.mode, fileDialogShown)
        if (!fileDialogShown) {
            fileDialogShown = true
            evaluateFile()
        }

        requiredColumnsMatched =
                hasSelectedColumn("Date") && hasSelectedColumn("Description") &&
                (hasSelectedColumn("Amount") ||
                 hasSelectedColumn("Debit") && hasSelectedColumn("Credit"))

        if (showReverseDrCrMsg) {
            const curDrCrColumns = selectedColumns.filter(col => col && (col.id === 'Debit' || col.id === 'Credit'))
            if (originalDrCrColumns.length === 0 && curDrCrColumns.length > 0) {
                originalDrCrColumns = [...curDrCrColumns]
            } else {
                let changed = curDrCrColumns.length > 0 &&  (originalDrCrColumns.length !== curDrCrColumns.length ||
                     originalDrCrColumns.some((col, index) => col.id !== curDrCrColumns[index].id))
                showReverseDrCrMsg = !changed
            }
        }
    });

    const evaluateFile = async () => {
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
            path = selected
            evaluateCsv(selected, curAccount)
        }
    }

    function loaded(result) {
        console.log(result)
        rows = result.sample_rows.slice(0, 20)
        columnTypes = result.column_types.columns
        showReverseDrCrMsg = result.dr_cr_reversed
        columns = result.column_types.columns.map(c => ({name: c}))
        selectedColumns = []
        columnTypes.forEach(c => selectedColumns.push(COLUMN_TYPES_MAP[c]))
        mappingExists = result.mapping_exists
        rememberForNextTime = mappingExists
        requiredColumnsMatched =
                hasSelectedColumn("Date") && hasSelectedColumn("Description") &&
                (hasSelectedColumn("Amount") ||
                 hasSelectedColumn("Debit") && hasSelectedColumn("Credit"))
        hasHeaderRow = !(columnTypes.length > 0 && columnTypes.every(e => e == "Unknown"))
    }

    function importCompleted(result) {
        console.log(result)
        onClose()
    }

    function rejected(result) {
        console.log(result)
        errors = new Errors()
        errors.addError("all", $_('transactions.error', { values: { error: result } }))
    }

    const evaluateCsv = async (path, account) => {
        console.log(path)
        errors = new Errors()
        rows = []
        columnTypes = []
        selectedColumns = []
        columns = []
        requiredColumnsMatched = false
        await invoke('evaluate_csv', {path: path, account: account}).then(loaded, rejected)
    }

    const importCsv = async () => {

        if (!path) {
            errors.addError("all", "No file selected")
            return
        }

        if (!curAccount || !curAccount.id) {
            errors.addError("all", "No account selected")
            return
        }

        let updatedColumns = []
        selectedColumns.forEach(c => updatedColumns.push(c.id))

        console.log('Calling import_csv with:', {path, accountId: curAccount.id, columnTypes: updatedColumns, saveMapping: rememberForNextTime, hasHeaders: hasHeaderRow})
        await invoke('import_csv', {path: path, accountId: curAccount.id, columnTypes: updatedColumns, saveMapping: rememberForNextTime, hasHeaders: hasHeaderRow}).then(importCompleted, rejected)
    }

    let lastReconcileRequest = null

    const reconcileCsv = async () => {

        if (!path) {
            errors.addError("all", "No file selected")
            return
        }

        if (!curAccount || !curAccount.id) {
            errors.addError("all", "No account selected")
            return
        }

        let updatedColumns = []
        selectedColumns.forEach(c => updatedColumns.push(c.id))

        lastReconcileRequest = {
            path: path,
            accountId: curAccount.id,
            columnTypes: updatedColumns,
            hasHeaders: hasHeaderRow
        }
        console.log('Calling reconcile_csv with:', {path, accountId: curAccount.id  , columnTypes: updatedColumns, hasHeaders: hasHeaderRow, reverseDrCr: showReverseDrCrMsg})
        await invoke('reconcile_csv', {path: path, accountId: curAccount.id, columnTypes: updatedColumns, hasHeaders: hasHeaderRow}).then(reconciliationCompleted, rejected)
    }

    const reconciliationCompleted = (results) => {
        msg = `Reconciliation complete: ${results.length} transactions processed`
        console.log('Reconciliation results:', results)
        if (onReconciliationResults) {
            onReconciliationResults(results, lastReconcileRequest)
        }
    }

    const close = () => {
        onClose()
    }

</script>


<div class="account-heading">
    <div class="account">
        <Select bind:item={curAccount} items={$accounts} none={true} flat={true} disabled={true}/>
    </div>
    <div class="toolbar">
        {#if curAccount}
        <button class="toolbar-icon import-icon" onclick={evaluateFile} title={$_('transactions.openCsv')}><Icon icon="mdi:folder-upload" width="22"/></button>
        <button class="{requiredColumnsMatched ? 'toolbar-icon-on' : 'toolbar-icon'} import-icon" onclick={importCsv} title={$_('transactions.importTransactions')}>
            <Icon icon="mdi:application-import" width="22"/>
        </button>
        <button class="{requiredColumnsMatched ? 'toolbar-icon-on' : 'toolbar-icon'} import-icon" onclick={reconcileCsv} title={$_('importer.reconcileTransactions')}>
            <Icon icon="mdi:compare-horizontal" width="22"/>
        </button>
        <button class="toolbar-icon import-icon" onclick={close} title={$_('actions.close')}><Icon icon="mdi:window-close" width="22"/></button>
        {/if}
    </div>
</div>

<MessagePanel {errors} {msg} />

<div class="controls">
    <div class="form-heading"></div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('importer.has_header')}</div><input type="checkbox" bind:checked={hasHeaderRow} />
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('importer.import_date_format')}</div><div class="field"><Select bind:item={$config.import_date_format} items={DATE_FORMATS.slice(1)} flat={true} valueField="format" /></div>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('importer.save_mappings')}</div><input type="checkbox" bind:checked={rememberForNextTime} />
        </div>
    </div>
</div>
<div class="form-row2">
    <div class="widget">
        {#if requiredColumnsMatched}
        <div class="label label-column"><div class="success-msg">{$_('importer.required_columns_mapped')}&nbsp;&check;</div></div>
        {/if}
    </div>
</div>
<div class="scroller" id="scroller">
    <table class="csv-table">
        <tbody>
            <tr><th colspan="2">{$_('importer.columns')}</th><th colspan="{columnTypes.length - 2}"><div class="label label-column note">{#if showReverseDrCrMsg}{$_('importer.reverse_dr_cr')}{/if}</div></th></tr>
            <tr class="shrink-font">
            {#each columnTypes as c, i}
                <td class="{selectedColumns[i] && selectedColumns[i].id != "Unknown"?'matched ':' '}"><Select bind:item={selectedColumns[i]} items={COLUMN_TYPES} none={true} flat={true}/></td>
            {/each}
            </tr>
            <tr class="spacer"></tr>
            <tr><th colspan="{columnTypes.length}">{$_('importer.sample_data')}</th></tr>
        {#each rows as r}
            <tr class="csv-row">
            {#each r as c}
                <td>{c}</td>
            {/each}
            </tr>
        {/each}
        </tbody>
    </table>
    {#if rows.length < 1}
    <div class="message">{$_('importer.no_data')}</div>
    {/if}
</div>

<style>


    .form-row2 {
        display: block;
    }

    .widget {
        padding: 5px 0px 5px 10px;
        float: left;
        clear: both;
    }

    .label {
        font-size: .9em;
        margin: 0 5px 5px 0;
        display: inline-block;
        text-align: left;
        min-width: 11em;
    }



    .controls input {
        background-color: var(--color-input-bg);
    }

    .csv-table td {
        max-width: 18vw;
        overflow: hidden;
        text-overflow: ellipsis;
        font-size: 0.9em;
    }

    .csv-row td {
        font-size: 0.8em;
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

    th {
        color:var(--color-border);
        background-color: var(--color-bg);
        font-weight: 400;
        font-size: .8em;
        text-align: left;
    }

    .scroller tr:hover td {
        cursor: pointer;
        color: var(--color-text-strong);
    }



    .shrink-font {
        font-size: .9em;
    }

    .matched {
        background-color: var(--color-success-bg);
    }

    .message {
        margin: 5px 0 20px 0;
    }

    .success-msg {
        color: var(--color-success-strong);
        text-align: left;
    }
    .note {
        font-size: 0.7em;
        color: var(--color-accent) !important;
        height: 9px;
    }

</style>
