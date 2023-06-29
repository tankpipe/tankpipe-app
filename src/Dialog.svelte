<script>
    import Icon from '@iconify/svelte'
    export let dialog
    export let closeButton = false
    export let closeIcon = false

    const close = () => {
        dialog.close()
    }
</script>
<dialog bind:this={dialog} on:close>
    {#if closeIcon}
    <div class="toolbar">
        <div class="toolbar-icon" on:click={close} title="Close dialog"><Icon icon="mdi:close"  width="24"/></div>
    </div>
    {/if}
    <slot/>
    {#if closeButton}
    <div class="widget buttons">
        <button on:click={close} autofocus>Ok</button>
    </div>
    {/if}
</dialog>
<style>

    dialog {
        border: 1px solid #333 !important;
        background-color: #444;
        border-radius: 10px;
    }

    dialog::backdrop {
            background: rgba(0, 0, 0, 0.3);
    }

    .buttons {
        float: left;
        width: 100%;
    }


    .buttons {
        float: right;
    }

    .buttons button {
        min-width: 80px;
    }

</style>