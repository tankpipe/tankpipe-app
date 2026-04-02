<script>
    let { errors, msg } = $props()
</script>

{#if errors?.getErrorMessages().length > 0 || msg}
<div class="widget errors">
    {#each errors?.getErrorMessages() as e}
    <div class="error-msg selectable-text">{e}</div>
    {/each}
    {#if msg}
    <div class="success-msg selectable-text">{msg}</div>
    {/if}
</div>
{/if}

<style>
    .errors {
        float: left;
        margin: 7px 12px 0 0;
    }

    .errors div {
        padding: 0;
    }

    .error-msg {
        color: var(--color-warning);
        text-align: left;
        margin-bottom: 3px;
        font-size: 0.9em;
    }

    .success-msg {
        color: var(--color-success);
        text-align: left;
    }
</style>
