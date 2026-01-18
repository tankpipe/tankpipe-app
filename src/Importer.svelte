<script>
    import Select from './Select.svelte'
    import Icon from '@iconify/svelte'
    import { open } from '@tauri-apps/plugin-dialog'
    import { documentDir } from '@tauri-apps/api/path'
    import { Errors } from './errors'
    import { page } from './page'
    import { config } from './config.js'
    import { accounts } from './accounts'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'

    export let curAccount
    export let onClose

    let errors = new Errors()
    let msg = ""
    let rows = []
    let columnTypes = []
    let selectedColumns = []
    let columns = []
    let requiredColumnsMatched = false;
    let mappingExists = false;
    let path = ""
    let fileDialogShown = false
    let rememberForNextTime = false
    let hasHeaderRow = false;


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

    $: {
        if ((!curAccount || !curAccount.id) && $accounts.length > 0) {
            curAccount = $accounts[0]
        }

        console.log($page.mode, fileDialogShown)
        if (!fileDialogShown) {
            fileDialogShown = true
            evaluateFile()
        }

        requiredColumnsMatched =
                selectedColumns.includes(COLUMN_TYPES_MAP["Date"]) && selectedColumns.includes(COLUMN_TYPES_MAP["Description"]) &&
                (selectedColumns.includes(COLUMN_TYPES_MAP["Amount"]) ||
                 selectedColumns.includes(COLUMN_TYPES_MAP["Debit"]) && selectedColumns.includes(COLUMN_TYPES_MAP["Credit"]))
    }

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
        columns = result.column_types.columns.map(c => columns.push({name: c}))
        selectedColumns = []
        columnTypes.forEach(c => selectedColumns.push(COLUMN_TYPES_MAP[c]))
        mappingExists = result.mapping_exists
        rememberForNextTime = mappingExists
        requiredColumnsMatched =
                columnTypes.includes("Date") && columnTypes.includes("Description") &&
                (columnTypes.includes("Amount") ||
                 columnTypes.includes("Debit") && columnTypes.includes("Credit"))
        hasHeaderRow = !(columnTypes.length > 0 && columnTypes.every(e => e == "Unknown"))
    }

    function importCompleted(result) {
        console.log(result)
        fileDialogShown = false
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
        console.log(path, columnTypes)
        errors = new Errors()
        let updatedColumns = []
        selectedColumns.forEach(c => updatedColumns.push(c.id))
        await invoke('import_csv', {path: path, account: curAccount, columnTypes: updatedColumns, saveMapping: rememberForNextTime, hasHeaders: hasHeaderRow}).then(importCompleted, rejected)
    }

    const close = () => {
        fileDialogShown = false
        onClose()
    }

</script>


<div class="account-heading">
    <div class="account">
        <Select bind:item={curAccount} items={$accounts} none={true} flat={true} disabled={true}/>
    </div>
    <div class="toolbar">
        {#if curAccount}
        <button class="toolbar-icon import-icon" on:click={evaluateFile} title={$_('transactions.openCsv')}><Icon icon="mdi:folder-upload" width="22"/></button>
        <button class="{requiredColumnsMatched ? 'toolbar-icon-on' : 'toolbar-icon'} import-icon" on:click={importCsv()} title={$_('transactions.importTransactions')}><Icon icon="mdi:application-import" width="22"/></button>
        <button class="toolbar-icon import-icon" on:click={close} title={$_('actions.close')}><Icon icon="mdi:window-close" width="22"/></button>
        {/if}
    </div>
</div>

<div class="widget errors">
    {#each errors.getErrorMessages() as e}
    <div class="error-msg">{e}</div>
    {/each}
    {#if msg}
    <div class="success-msg">{msg}</div>
    {/if}
</div>

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
            <tr><th colspan="{columnTypes.length}">{$_('importer.columns')}</th></tr>
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
    .controls {
        text-align: center;
    }

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
        color: #aaa !important;
        margin: 0 5px 5px 0;
        display: inline-block;
        text-align: left;
        min-width: 11em;
    }

    .field {
        text-align: left;
        display: inline-block;
    }

    .controls input {
        background-color: #aaa;
    }

    .scroller{
        height: 100%;
        width: 100%;
        overflow: scroll;
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
        text-align: left;
    }

    .scroller tr:hover td {
        cursor: pointer;
        color: #FFF;
    }

    .account {
        float: left;
    }

    .shrink-font {
        font-size: .9em;
    }

    .matched {
        background-color: rgb(0, 71, 0);
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
        color: rgb(252, 0, 0);
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
    }

    .success-msg {
        color: rgb(0, 187, 0);
        text-align: left;
    }

</style>