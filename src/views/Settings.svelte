<script>
    import {settings} from '../stores/settings.js'
    import {config} from '../stores/config.js'
    import {hasBooks} from '../stores/context.js'
    import Select from '../components/Select.svelte';
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'

    const DATE_FORMATS = [
        {value: "Locale", name: $_('settings.dateFormats.localeDefault')},
        {value: "Regular", name: $_('settings.dateFormats.regular')},
        {value: "US", name: $_('settings.dateFormats.us')},
        {value: "ISO", name: $_('settings.dateFormats.iso')}
    ]

    const THEME_OPTIONS = [
        {value: "System", name: $_('settings.themes.system')},
        {value: "Light", name: $_('settings.themes.light')},
        {value: "Dark", name: $_('settings.themes.dark')},
    ]

    const updateSettings = async () => {
        if ($settings) {
            await invoke('update_settings', {settings: $settings}).then(
                () => console.log("settings saved"),
                (result) => console.log("settings not saved " + result)
            )
        }
    }

    const updateConfig = async () => {
        if ($config) {
            const configSettings = {
                display_date_format: $config.display_date_format,
                theme: $config.theme
            }

            await invoke('update_config', {configSettings: configSettings}).then(
                () => console.log("configuration saved"),
                (result) => console.log("configuration not saved " + result)
            )
        }
    }

    const updateTheme = async () => {
        if ($settings) {
            $settings.theme = $settings.theme || 'System'
            await updateSettings()
        }
    }

    $effect(() => {
        if ($settings && !$settings.theme) {
            $settings.theme = 'System'
        }
    })
</script>
<div class="controls">
    <div class="form-heading">{$_('settings.title')}</div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('settings.enforceDoubleEntry')}</div><input type="checkbox" bind:checked={$settings.require_double_entry} onchange={updateSettings} disabled={!hasBooks()}/>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('settings.displayDateFormat')}</div><div class="field"><Select bind:item={$config.display_date_format} items={DATE_FORMATS} flat={true} valueField="value" onChange={updateConfig}  disabled={!hasBooks()}/></div>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('settings.theme')}</div><div class="field"><Select bind:item={$config.theme} items={THEME_OPTIONS} flat={true} valueField="value" onChange={updateConfig} disabled={!hasBooks()}/></div>
        </div>
    </div>
</div>
<style>

    .form-row2 {
        display: block;
    }

    .widget {
        padding: 5px 0px 5px 10px;
        float: left;
        clear: both;
    }

    .label {
        font-size: .9em;
        margin: 0 5px 5px 0;
        display: inline-block;
        text-align: left;
        line-height: 36px;
        width: 11em;
    }

    .controls .label {
        color: var(--color-text);
    }

    .controls input {
        background-color: var(--color-input-bg);
    }

    .label-column {
        min-width: 220px;
    }

</style>
