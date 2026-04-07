<script>
    import {settings, updateSettings} from '../stores/settings.js'
    import {config, formatDate} from '../stores/config.js'
    import {hasBooks} from '../stores/context.js'
    import Select from '../components/Select.svelte';
    import { invoke } from '@tauri-apps/api/core'
    import MessagePanel from '../components/MessagePanel.svelte'
    import {Errors} from '../utils/errors.js'
    import { _ } from 'svelte-i18n'
    const MAX_PROJECTION_MONTHS = 1200

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

    let msg = $state("")
    let errors = $state(new Errors())
    let showDoubleEntry = $state(false)

    const saveSettings = async () => {
        if ($settings) {
            await invoke('update_settings', {settings: $settings}).then(settingsResolved, settingsRejected)
        }
    }

    const settingsResolved = (result) => {
        msg = $_('settings.saved')
        setTimeout(() => {msg = ""}, 3000)
        updateSettings(result)
    }

    const settingsRejected = (result) => {
        errors = new Errors()
        errors.addError("all", result)
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

    $effect(() => {
        if ($settings && !$settings.theme) {
            $settings.theme = 'System'
        }

        if ($settings && ($settings.projection_months == null || Number.isNaN(Number($settings.projection_months)))) {
            $settings.projection_months = 12
        }

        if ($settings && $settings.projection_months > MAX_PROJECTION_MONTHS) {
            $settings.projection_months = MAX_PROJECTION_MONTHS
        }

        if ($settings?.require_double_entry) {
            showDoubleEntry = true
        }

    })
</script>
<div class="controls">
    <div class="form-heading">{$_('settings.title')}</div>
    <div class="info-title">{$_('settings.appSettings')}</div>
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
<hr/>
<div class="controls">
    <div class="info-title" title={$config.current_file?.path || ''}>{$_('settings.booksSettings', {values: {name: $config.current_file?.name || ''}})}</div>
    {#if showDoubleEntry}
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('settings.enforceDoubleEntry')}</div><input type="checkbox" bind:checked={$settings.require_double_entry} disabled={!hasBooks()}/>
        </div>
    </div>
    {/if}
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('settings.projectionMonths')}</div><input type="number" min="0" max={MAX_PROJECTION_MONTHS} step="1" bind:value={$settings.projection_months} disabled={!hasBooks()}/>
        </div>
    </div>
    {#if $settings.projected_to}
    <div class="form-row">
        <div class="small-text" style="padding-left: 7px;">
            {$_('settings.projectedTo')}&nbsp;{formatDate($settings.projected_to)}
        </div>
    </div>
    {/if}
    <div class="form-button-row">
        <div class="msg-panel sele">
            <MessagePanel errors={errors} msg={msg} />
        </div>
        <div class="widget buttons">
            <button class="og-button" onclick={saveSettings}>{$_('buttons.update')}</button>
        </div>
    </div>

</div>
<hr/>
<style>

    .form-row2 {
        display: block;
    }

    .widget {
        padding: 5px 0px 5px 10px;
        float: left;
        clear: none;
    }

    .label {
        font-size: .9em;
        margin: 0 5px 5px 0;
        display: inline-block;
        text-align: left;
        line-height: 36px;
        width: 11em;
    }

    .controls {
        width: 500px;
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

    .form-button-row {
        display: block;
        text-align: left;
        margin-left: 7px;
        margin-right: 2px;
        clear: both;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

</style>
