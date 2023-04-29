<script>
    import {settings} from './settings.js'
    import {config} from './config.js'
    import Select from './Select.svelte';
    import {generate, getEndDate} from './generate'

    let dateStr
    const DATE_FORMATS = [{value: "Locale", name:"Locale default"}, {value: "Regular", name: "Regular (D/M/Y)", format: "%d/%m/%Y"}, {value: "US", name:"US (M/D/Y)", format: "%m/%d/%Y"}, {value: "ISO", name:"ISO (Y-M-D)", format: "%Y-%M-%D"} ]

    $: {
        if (dateStr) {
            generate()
        }
    }


    const updateSettings = async () => {
        if ($settings) {
            await invoke('update_settings', {settings: $settings}).then(
                () => console.log("settings saved"),
                () => console.log("settings not saved " + result)
            )
        }
    }

    const updateConfig = async () => {
        if ($config) {
            const configSettings = {
                display_date_format: $config.display_date_format,
                import_date_format: $config.import_date_format
            }

            await invoke('update_config', {configSettings: configSettings}).then(
                () => console.log("configuration saved"),
                () => console.log("configuration not saved " + result)
            )
        }
    }

    const getDate = async () => {
        dateStr = await getEndDate()
    }
    getDate()

</script>
<div class="controls">
    <div class="form-heading">Settings</div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">Enforce double entry</div><input type="checkbox" bind:checked={$settings.require_double_entry} on:change={updateSettings}/>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">Display date format</div><div class="field"><Select bind:item={$config.display_date_format} items={DATE_FORMATS} flat={true} valueField="value" onChange={updateConfig}/></div>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">Import date format</div><div class="field"><Select bind:item={$config.import_date_format} items={DATE_FORMATS.slice(1)} flat={true} valueField="format" onChange={updateConfig}/></div>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">Schedule until </div><div class="date-input field"><input type="date" bind:value={dateStr}/></div>
        </div>
    </div>
</div>
<style>
    .controls {
        text-align: center;
    }

    .form-row2 {
        display: block;
    }

    .widget {
        padding: 5px 0px 5px 10px;
        float: left;
        clear: both;
    }

    .date-input {
        float: right;
    }

    .date-input input {
        border: none;
    }

    .label {
        font-size: .9em;
        color: #aaa !important;
        margin: 0 5px 5px 0;
        display: inline-block;
        text-align: left;
        line-height: 36px;
        width: 11em;
    }

    .field {
        text-align: left;
        display: inline-block;
    }

    .controls input {
        background-color: #aaa;
    }

</style>