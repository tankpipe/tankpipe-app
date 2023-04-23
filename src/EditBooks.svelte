<script>
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import {page, modes, views} from "./page"
    import {accounts} from './accounts'
    import {context} from './context.js';
    import {emit} from '@tauri-apps/api/event'

    let msg = ""
    let errors = new Errors()
    let name
    let addButtonLabel = "Add"

    onMount(() => {
            addButtonLabel = "Add"
    })

    const onCancel = () => {
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

    const newFile = async (name) => {
        await invoke('new_file', {name: name}).then(loadFileSuccess, loadFileFailure)
    }

    function loadFileSuccess(result) {
        console.log(result)
        emit('file-loaded', "")
        accounts.set(result)
    }

    function loadFileFailure(result) {
        console.log(result)
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }

    const onAdd = () => {
        msg = ""
        errors = new Errors()
        if (!name || name.length < 1) {
             errors.addError("name", "Name is required")
        }

        const regex = /[^a-z0-9_\- ]/gi
        if (regex.test(name)) {
            errors.addError("name", "Name must be alphanumeric")
        }

        if (!errors.hasErrors()) {
            newFile(name)
        }

    }


</script>

{#if ! context.hasBooks}
    <div class="message">To get started give your first set of accounts a name. For example, Personal Finances.</div>
{/if}
<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT?"Edit":"New"} Books</div>
    <div class="form-row">
        <div class="widget">
            <label for="name">Name</label>
            <input id="name" class="description-input" class:error={errors.isInError("name")} bind:value={name}>
        </div>
    </div>
    <div class="form-button-row">
        <div class="msg-panel">
            {#each errors.getErrorMessages() as e}
            <p class="error-msg">{e}</p>
            {/each}
            {#if msg}
            <p class="success-msg">{msg}</p>
            {/if}
        </div>
        <div class="widget buttons">
            {#if context.hasBooks}
            <button on:click={onCancel}>Close</button>
            {/if}
            <button on:click={onAdd}>{addButtonLabel}</button>
        </div>
    </div>
</div>

<style>

    :global(.date-time-field input) {
        border: 1px solid #CCC !important;
        border-radius: 2px !important;
        height: 33px;
    }

    :root {
        --date-input-width: 110px;
    }

    .msg-panel {
        padding-left: 2px;
        font-size: 0.9em;
        float:left;
    }

    .msg-panel p {
        margin: 8px 0;
        max-width: 500px;
    }

    .error-msg {
        color: #FBC969;
    }

    .success-msg {
        color: green;
    }

    .error {
        border: 1px solid #FBC969 !important;
    }

    :global(.error-input input) {
        border: 1px solid #FBC969 !important;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .buttons button {
        min-width: 80px;
    }

    .form-row {
        display: inline-flex;
        float: left;
        width: 100%;
        clear:both;
    }

    .form-button-row {
        display: block;
        text-align: left;
    }

    .form-button-row {
        margin-left: 7px;
        margin-right: 2px;
    }

    input {
        margin-right: 0px;
    }

    .form {
        float: left;
        border-radius: 10px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .description-input {
        width: 400px;
    }

</style>