<script>
    export let items = []
    export let item
    export let label = null
    export let none = false
    const cssClass =  label ? "widget" : "widgetNoLabel"
</script>

<div class={cssClass}>
    {#if label}<label for="itemSelect">{label}</label>{/if}
    <select bind:value={item} name="itemSelect">
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
</style>