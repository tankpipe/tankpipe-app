<script>
    import { onDestroy } from 'svelte'

    export let items = []
    export let item = {}
    export let disabledItems = []
    export let label = null
    export let title = null
    export let none = false
    export let flat = false
    export let limitWidth = false
    export let inError = false
    export let disabled = false
    export let valueField = null
    export let onChange = () => {}
    const cssClass =  label ? "widget" : "widgetNoLabel"

    const defer = (fn) => {
        if (typeof queueMicrotask === 'function') return queueMicrotask(fn)
        return Promise.resolve().then(fn)
    }

    let destroyed = false
    onDestroy(() => {
        destroyed = true
    })

    const isItemDisabled = (candidate) => {
        if (!disabledItems || disabledItems.length < 1) return false
        if (valueField) {
            const candidateValue = candidate?.[valueField]
            return disabledItems.some(d => {
                if (d && typeof d === 'object') return d?.[valueField] === candidateValue
                return d === candidateValue
            })
        }
        if (disabledItems.includes(candidate)) return true
        const candidateId = candidate?.id
        if (!candidateId) return false
        return disabledItems.some(d => d && typeof d === 'object' && d.id === candidateId)
    }

    const handleChange = (event) => {
        // Defer so `bind:value={item}` updates propagate before callers run.
        defer(() => {
            if (destroyed) return
            onChange(event)
        })
    }
</script>

<div class={cssClass}>
    {#if label}<label for="itemSelect">{label}</label>{/if}
    <select bind:value={item} name="itemSelect" class:flat={flat} class:limit-width={limitWidth} class:error={inError} disabled={disabled} on:change={handleChange} {title}>
        {#if none}
            <option value={null}>None</option>
        {/if}
        {#each items as a}
            {#if valueField}
                <option value={a[valueField]} disabled={isItemDisabled(a)}>{a.name}</option>
            {:else}
                <option value={a} disabled={isItemDisabled(a)}>{a.name}</option>
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
        background-color: var(--color-input-bg);
        appearance: none;
        position: relative;
        background-image: url("data:image/svg+xml;utf8,<svg fill='black' height='24' viewBox='0 0 24 24' width='24' xmlns='http://www.w3.org/2000/svg'><path d='M7 10l5 5 5-5z'/><path d='M0 0h24v24H0z' fill='none'/></svg>");
        background-repeat: no-repeat;
        background-position-x: 100%;
        background-position-y: 5px;
        padding-right: 2rem;
        max-width: 250px;
    }

    .flat::after {
        content: "\25b6";
        color: var(--color-white);
        position: absolute; top: 0; right: 0;
    }

    .limit-width {
        max-width: 200px;
    }

    .error {
        border: 1px solid var(--color-warning) !important;
    }

    :global(.error-input input) {
        border: 1px solid var(--color-warning) !important;
    }
</style>
