<script>
    import Icon from '@iconify/svelte'
    export let dialog
    export let heading

    const close = () => {
        dialog.close()
    }
</script>
<dialog bind:this={dialog} on:close>
    <div class="toolbar">
        <div class="toolbar-icon" on:click={close} title="Close dialog"><Icon icon="mdi:close"  width="24"/></div>
    </div>
    <slot/>
</dialog>
<style>

    dialog {
        border: 1px solid #333 !important;
        background-color: #444;
        border-radius: 10px;
    }

    .dialog-body {
        padding-top: 50px;
    }

    dialog::backdrop {
        background: rgba(0, 0, 0, 0.3);
    }

</style>