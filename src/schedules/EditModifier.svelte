<script>
    import {Errors} from '../errors.js'
    import {onMount} from "svelte"
    import {page, modes} from '../page.js'
    import Icon from '@iconify/svelte'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import { periods } from '../dates.js'
    import SchedulePanel from './SchedulePanel.svelte'

    const zeros = '00000000-0000-0000-0000-000000000000'

    let { close, curModifier, loadModifiers, view } = $props()

    let msg = $state("")
    let successMsg = $state("")
    let errors = $state(new Errors())
    let date = $state(new Date())
    let name = $state()
    let amount = $state(0)
    let percentage = $state(0)
    let frequency = $state(1)
    let endDate = $state()
    let addButtonLabel = $state("Add")
    let period = $state({value:"Months", name:"Months"})
    let hasEnd = $state(false)

    onMount(() => {
        if ($page.mode === modes.EDIT) {
            name = curModifier.name
            amount = curModifier.amount
            percentage = curModifier.percentage * 100
            frequency = curModifier.frequency
            period = matchPeriod(curModifier.period)
            endDate = curModifier.end_date == "null" ? null : new Date(curModifier.end_date)
            hasEnd = endDate != null
            date = new Date(curModifier.start_date)            
            addButtonLabel = $_('buttons.update')
            successMsg = $_('modifier.updated')
        } else if ($page.mode === modes.NEW) {
            addButtonLabel = $_('buttons.add')
            successMsg = $_('modifier.added')
        }
    })

    const matchPeriod = (periodValue) => {
        let match = periods.filter(p => p.value === periodValue)
        return match.length > 0 ? match[0] : periods[0]
    }

    const resolved = () => {
        msg = successMsg
        loadModifiers()
        setTimeout(() => {
            close()
        }, 1000)
    }

    const rejected = (result) => {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }

    const addModifier = async (modifier) => {
        console.log(modifier)
        modifier.id = zeros
        await invoke('add_modifier', {modifier: modifier}).then(resolved, rejected)
    }

    const saveModifier = async (modifier) => {
        console.log(modifier)
        await invoke('update_modifier', {modifier: modifier}).then(resolved, rejected)
    }

    const onAdd = () => {
        errors = new Errors()
        msg = ""

        if (!name || name.trim() === '') {
            errors.addError("name", $_('modifier.nameRequired'))
            return
        }

        if (frequency < 1) {
            errors.addError("frequency", $_('modifier.frequencyRequired'))
            return
        }

        if (!errors.hasErrors()) {
            let dateStr = date.getFullYear()+ "-" + (date.getMonth() + 1) + "-" + date.getDate()
            let endDateStr = hasEnd ? endDate.getFullYear()+ "-" + (endDate.getMonth() + 1) + "-" + endDate.getDate() : "null"          

            const modifier = {
                id: $page.mode === modes.EDIT ? curModifier.id : null,
                name: name.trim(),
                amount: Number(amount) || 0,
                percentage: Number(percentage) / 100 || 0,
                frequency: Number(frequency),
                period: period.value,
                start_date: dateStr,
                end_date: endDateStr
            }

            if ($page.mode === modes.EDIT) {
                saveModifier(modifier)
            } else {
                addModifier(modifier)
            }
        }
    }

    const onCancel = () => {
        close()
    }

    const handleEndDateChange = (selectedDate) => {
        endDate = selectedDate
    }

</script>

<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT ? $_('modifier.edit_modifier') : $_('modifier.new_modifier')}</div>
    <div class="toolbar toolbar-right">
        <button class="toolbar-icon" onclick={onCancel} title={$_('buttons.cancel')}><Icon icon="mdi:close-box-outline" width="24"/></button>
    </div>

    <div class="form-row">
        <div class="top-widget">
            <label for="desc">{$_('labels.name')}</label>
            <input id="desc" class="description-input" class:error={errors.isInError("name")} bind:value={name}>
        </div>
    </div>

    <div class="form-row">
        <div class="top-widget">
            <label for="percentage">{$_('modifier.percentage')}</label>
            <input id="percentage" class="money-input" class:error={errors.isInError("percentage")} type="number" bind:value={percentage} placeholder="0.0" step="0.1" />
        </div>
    </div>

    <hr/>
    <SchedulePanel {frequency} {period} {date} {hasEnd} {endDate} {errors} />
    <hr/>
    <div class="form-button-row">
        <div class="widget">
            {#each errors.getErrorMessages() as e}
            <p class="error-msg">{e}</p>
            {/each}
            {#if msg} 
            <p class="success-msg">{msg}</p>
            {/if}
        </div>
        <div class="widget buttons">
            <button class="og-button" onclick={onCancel}>{$_('buttons.close')}</button>
            <button class="og-button" onclick={onAdd}>{addButtonLabel}</button>
        </div>
    </div>
</div>

<style>    
    .error-msg {
        color: #FBC969;
        font-size: .8em;
    }

    .success-msg {
        color: green;
        font-size: .8em;
    }

    .error {
        border: 1px solid #FBC969 !important;
    }

    .form {
        float: left;
        border-radius: 10px;
        color: #DDDDDD;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;
    }

    .widget p {
        max-width: 500px;
        font-size: 0.9em;
    }

    .top-widget {
        display: inline-block;
        padding: 5px 0px 5px 0px;
    }

    .money-input {
        width: 120px;
        text-align: right;
    }

    .buttons {
        float: right;
        margin: 10px 12px 0 0;
    }

    .buttons button {
        min-width: 80px;
    }

    .form-row {
        display: block;
        float: left;
        clear: both;
        margin-top: 10px;
    }

    .form-button-row {
        margin-left: 7px;
        margin-right: 2px;
    }

    hr {
        border-style: none;
        border: 1px solid #363636;
        margin-left: -20px;
        width: 100vw;
        clear: both;
    }
 

</style>
