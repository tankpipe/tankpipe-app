<script>
    export let items = []
    export let item = {}
    export let label = null
    export let none = false
    export let flat = false
    export let inError = false
    export let disabled = false
    export let valueField = null
    export let onChange = {}
    const cssClass =  label ? "widget" : "widgetNoLabel"
</script>

<div class={cssClass}>
    {#if label}<label for="itemSelect">{label}</label>{/if}
    <select bind:value={item} name="itemSelect" class:flat={flat} class:error={inError} disabled={disabled} on:change={onChange}>
        {#if none}
            <option value={null}>None</option>
        {/if}
        {#each items as a}
            {#if valueField}
                <option value={a[valueField]}>{a.name}</option>
            {:else}
                <option value={a}>{a.name}</option>
            {/if}
        {/each}
    </select>
</div>

<style>
    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .widgetNoLabel {
        display: inline-block;
    }

    label {
        text-align: left;
        font-size: .8em;
        margin-bottom: 3px;
    }

    .flat {
        background-color: #aaa;
        appearance: none;
        position: relative;
        background-image: url("data:image/svg+xml;utf8,<svg fill='black' height='24' viewBox='0 0 24 24' width='24' xmlns='http://www.w3.org/2000/svg'><path d='M7 10l5 5 5-5z'/><path d='M0 0h24v24H0z' fill='none'/></svg>");
        background-repeat: no-repeat;
        background-position-x: 100%;
        background-position-y: 5px;
        padding-right: 2rem;
    }

    .flat::after {
        content: "\25b6";
        color: #fff;
        position: absolute; top: 0; right: 0;
    }

    .error {
        border: 1px solid #FBC969 !important;
    }

    :global(.error-input input) {
        border: 1px solid #FBC969 !important;
    }
</style>