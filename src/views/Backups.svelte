<script>
    import { onMount } from 'svelte'
    import { invoke } from '@tauri-apps/api/core'
    import { page, views, modes } from '../stores/page.js'
    import { updateAccounts } from '../stores/accounts.js'
    import { _ } from 'svelte-i18n'
    import Icon from '@iconify/svelte'
    import MessagePanel from '../components/MessagePanel.svelte'
    import { Errors } from '../utils/errors.js'
    import { loadConfig, loadSettings } from '../events.js'

    let backups = []
    let loading = false
    let restoringPath = ""
    let msg = ""

    let runChecks = true
    let errors = new Errors()

    const loadBackups = async () => {
        loading = true
        msg = ""
        errors = new Errors()
        await invoke('list_backups').then(loadBackupsSuccess, loadBackupsRejected)
    }

    const loadBackupsSuccess = (result) => {
        backups = result
        loading = false
    }

    const loadBackupsRejected = (result) => {
        errors = new Errors()
        errors.addError("all", $_('backups.errors.load', {values: { error: result }}))
        loading = false
    }

    const restoreBackup = async (backupPath) => {
        restoringPath = backupPath
        msg = ""
        errors = new Errors()
        await invoke('restore_backup', { backupPath, runChecks }).then(restoreBackupSuccess, restoreBackupRejected)
    }

    const restoreBackupSuccess = async () => {
        await invoke('accounts').then(reloadAccountsSuccess, reloadAccountsRejected)
    }

    const restoreBackupRejected = (result) => {
        console.log(result)
        errors = new Errors()
        errors.addError("all", $_('backups.errors.restore', {values: { error: result }}))
        restoringPath = ""
    }

    const reloadAccountsSuccess = async (accounts) => {
        updateAccounts(accounts)
        await loadSettings()
        await loadConfig()
        msg = $_('backups.messages.restored')
        restoringPath = ""
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

    const reloadAccountsRejected = (result) => {
        errors = new Errors()
        errors.addError("all", $_('backups.errors.reload', {values: { error: result }}))
        restoringPath = ""
    }

    const closeView = () => {
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

    onMount(loadBackups)

    const formatBackupName = (path) => {
        const parts = path.split(/[\\/]/)
        const fileName = parts[parts.length - 1] || path
        const match = fileName.match(/^backup-(\d+)(?:-\d+)?\.json$/)
        if (!match) {
            return fileName
        }

        const timestamp = Number.parseInt(match[1], 10)
        const date = new Date(timestamp)
        if (Number.isNaN(date.getTime())) {
            return fileName
        }

        return date.toLocaleString(undefined, {
            year: 'numeric',
            month: 'short',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hour12: false,
        })
    }
</script>

<div class="form">
    <div class="form-heading">{$_('backups.title')}</div>
    <div class="toolbar toolbar-right">
        <button class="toolbar-icon" on:click={closeView} title={$_('buttons.close')}>
            <Icon icon="mdi:close-box-outline" width="24"/>
        </button>
    </div>
    <div class="message-panel">
        <MessagePanel {errors} {msg} />
    </div>
</div>
{#if loading}
    <div class="status">{$_('backups.loading')}</div>
{:else if backups.length === 0}
    <div class="status">{$_('backups.empty')}</div>
{:else}
    <div class="controls">
        <div class="form-row2">
            <div class="widget">
                <div class="label label-column">{$_('backups.runChecks')}</div><input type="checkbox" bind:checked={runChecks} />
            </div>
        </div>
    </div>
    <div class="status">{$_('backups.selectPrompt')}</div>
    <div class="scroller" id="scroller">
    <ul class="backup-list">
        {#each backups as backupPath}
            <li class="backup-item">
                <div class="backup-name">{formatBackupName(backupPath)}</div>
                <button class="single-button" disabled={restoringPath.length > 0} on:click={() => restoreBackup(backupPath)} title={$_('backups.restoreButtonTitle')}>
                    <Icon icon="mdi:backup-restore" width="24"/>
                </button>
            </li>
        {/each}
    </ul>
    </div>
{/if}


<style>
    .message-panel {
        margin-left: 10px;
        margin-right: 10px;
        clear: both;
    }

    .status {
        font-size: 0.9em;
        color: var(--color-text-subtle);
        margin: 5px 0px 0px 10px;
        float: left;
        clear: both;
        text-align: left;
    }

    .backup-list {
        list-style: none;
        margin: 8px 0 0 0;
        padding: 0;
        max-width: 700px;
    }

    .backup-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
        margin: 8px 20px 8px 10px;
        padding: 10px 12px;
        border: none;
        border-radius: 4px;
        background-color: var(--color-surface);
        color: var(--color-text);
        min-width: 300px;
        width: fit-content;
    }

    .backup-name {
        font-variant-numeric: tabular-nums;
        font-size: 0.9em;
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
        margin: 0 5px 5px 0;
        display: inline-block;
        text-align: left;
        min-width: 11em;
    }

    .label-column {
        color: var(--color-text);
    }

</style>
