<script>
    export let items = []
    export let item = {}
    export let label = null
    export let none = false
    export let flat = false
    export let inError = false
    const cssClass =  label ? "widget" : "widgetNoLabel"  
</script>

<div class={cssClass}>
    {#if label}<label for="itemSelect">{label}</label>{/if}    
    <select bind:value={item} name="itemSelect" class:flat={flat} class:error={inError}>
        {#if none}
        <option value={null}>None</option>
        {/if}
        {#each items as a}
        <option value={a}>{a.name}</option>
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
        color: #333;
        margin-bottom: 3px;
    }

    .flat {
        border: none; 
        background-color: #F0F0F0; 
        appearance: button;
    }

    .error {
        border: 1px solid red !important;
    }

    :global(.error-input input) {
        border: 1px solid red !important;
    }
</style>