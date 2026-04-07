<script>
    import {Errors} from '../utils/errors.js'
    import {onMount} from "svelte"
    import {page, modes, views} from "../stores/page.js"
    import {accounts} from '../stores/accounts.js'
    import {context} from '../stores/context.js';
    import {config} from '../stores/config.js'
    import {loadConfig, loadSettings} from '../events.js'
    import {emit} from '@tauri-apps/api/event'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import Icon from '@iconify/svelte'
    import MessagePanel from '../components/MessagePanel.svelte'

    let msg = ""
    let errors = new Errors()
    let name
    let addButtonLabel = $_('buttons.add')

    onMount(() => {
        addButtonLabel = $_('buttons.add')
    })

    const onCancel = () => {
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

    const newFile = async (name) => {
        console.log("newFile", name)
        if ($config.current_books_id || $config.current_file) {
            await invoke('new_file', {name: name}).then(loadFileSuccess, loadFileFailure)
        } else {
            await invoke('create_first_books', {name: name}).then(loadFileSuccess, loadFileFailure)
        }
    }

    const loadFileSuccess = async (result) => {
        console.log(result)
        accounts.set(result)
        await loadSettings()
        await loadConfig()
        emit('file-loaded', "")
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

    function loadFileFailure(result) {
        console.log(result)
        errors = new Errors()
        errors.addError("all", $_('editBooks.errors.hit_snag', { result }))
    }

    const onAdd = () => {
        msg = ""
        errors = new Errors()
        if (!name || name.length < 1) {
             errors.addError("name", $_('editBooks.errors.name_required'))
        }

        const regex = /[^a-z0-9_\- ]/gi
        if (regex.test(name)) {
            errors.addError("name", $_('editBooks.errors.name_alphanumeric'))
        }

        if (!errors.hasErrors()) {
            newFile(name)
        }
    }
</script>

{#if ! context.hasBooks}
    <div class="message">{$_('editBooks.messages.get_started')}<p></p></div>
{/if}
<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT ? $_('editBooks.labels.edit_books') : $_('editBooks.labels.new_books')}</div>
    <div class="toolbar toolbar-right">
        <button class="toolbar-icon" on:click={onCancel} title={$_('buttons.close')}>
            <Icon icon="mdi:close-box-outline" width="24"/>
        </button>
    </div>
    <div class="form-row">
        <div class="widget">
            <label for="name">{$_('labels.name')}</label>
            <input id="name" class="description-input" class:error={errors.isInError("name")} bind:value={name}>
        </div>
    </div>
    <div class="form-button-row">
        <div class="msg-panel">
            <MessagePanel {errors} {msg} />
        </div>
        <div class="widget buttons">
            {#if context.hasBooks}
            <button class="og-button" on:click={onCancel}>{$_('buttons.close')}</button>
            {/if}
            <button class="og-button" on:click={onAdd}>{addButtonLabel}</button>
        </div>
    </div>
</div>

<style>
    :global(.date-time-field input) {
        border: 1px solid var(--color-border-light) !important;
        border-radius: 2px !important;
        height: 33px;
    }

    :root {
        --date-input-width: 110px;
    }


    .form-button-row {
        display: block;
        text-align: left;
        margin-left: 7px;
        margin-right: 2px;
        clear: both;
    }

    input {
        margin-right: 0px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .buttons button {
        min-width: 80px;
    }

</style>
