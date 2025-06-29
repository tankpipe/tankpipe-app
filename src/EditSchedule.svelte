<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import {page, modes} from './page.js'
    import {accounts} from './accounts'
    import {generate} from './generate'
    import {settings} from './settings.js'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'

    export let close
    export let curSchedule
    export let loadSchedules

    const zeros = '00000000-0000-0000-0000-000000000000'
    const minEntries = $settings.require_double_entry ? 2 : 1
    let hasEnd = false
    let msg = ""
    let errors = new Errors()
    let date = new Date(), name, description, amount, frequency = 1
    let endDate
    let max = new Date(), min = new Date()
    max.setFullYear(date.getFullYear() + 20)
    min.setFullYear(date.getFullYear() - 10)
    let format="yyyy-MM-dd"
    let addButtonLabel = "Add"
    let period = {value:"Months", name:"Months"}
    const periods = [{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, period, {value:"Years", name:"Years"}]
    let drAccount
    let crAccount
    let drTotal = 0
    let crTotal = 0
    let entries = []

    onMount(() => {
        if ($page.mode === modes.EDIT) {
            name = curSchedule.name
            description = curSchedule.description
            amount = curSchedule.amount
            entries = curSchedule.entries
            entries.forEach(e => {
                e.entry_type === "Credit" ? e.crAmount = e.amount : e.drAmount = e.amount
                e.realDate = new Date(e.date)
                e.account = matchAccount(e.account_id)
            })
            addButtonLabel = "Update"
            drAccount = matchAccount(curSchedule.dr_account_id)
            crAccount = matchAccount(curSchedule.cr_account_id)
            period = matchPeriod(curSchedule.period)
            frequency = curSchedule.frequency
            endDate = curSchedule.end_date == "null" ? null : new Date(curSchedule.end_date)
            hasEnd = endDate != null
            date = new Date(curSchedule.start_date)

        } else {
            drAccount = null
            crAccount = null
            addButtonLabel = "Add"

            if ($page.payload && $page.payload.entries) {
                console.log($page.payload)
                entries = $page.payload.entries
                entries.forEach((e, i) => e.schedule_id = zeros)
                name = $page.payload.entries[0].description
            } else {
                for (var i = 0; i < minEntries; i++) {
                    addEntry()
                }
            }
        }
    })

    const matchAccount = (accountId) =>  {
        if (!accountId) return null
        let match = $accounts.filter(a => a.id == accountId)
        return match.length > 0 ? match[0] : null
    }

    const matchPeriod = (value) =>  {
        if (!value) return null
        let match = periods.filter(p => p.value == value)
        return match.length > 0 ? match[0] : null
    }

    const onCancel = () => {
        console.log("onCancel")
        close()
    }

    const onAdd = () => {
        msg = ""
        errors = new Errors()
        if (!name || name.length < 1) {
             errors.addError("name", "Name is required")
        }

        if (!date || date.length < 1) {
            errors.addError("date", "Date is required")
        }

        if (!period) {
            errors.addError("period", "A period needs to be selected")
        }

        entries.forEach((e, i) => validateEntry(e, i, errors))

        if (!errors.hasErrors()) {
            let dateStr = date.getFullYear()+ "-" + (date.getMonth() + 1) + "-" + date.getDate()
            let endDateStr = hasEnd ? endDate.getFullYear()+ "-" + (endDate.getMonth() + 1) + "-" + endDate.getDate() : "null"
            entries.forEach (
                e => {
                    e["account_id"] = e["account"]["id"]
                    e["amount"] = (e["entry_type"] === "Credit") ? e["crAmount"] : e["drAmount"]
                }
            )

            let schedule = {
                    name: name,
                    period: period.value,
                    frequency: parseInt(frequency),
                    start_date: dateStr,
                    last_date: "null",
                    end_date: endDateStr,
                    entries: entries
                }

            if ($page.mode === modes.NEW) {
                addSchedule(schedule)
            } else if ($page.mode === modes.EDIT) {
                schedule["id"] = curSchedule.id
                schedule["last_date"] = curSchedule.last_date
                saveSchedule(schedule)
            }
        }

    }

    const resolved = async (result) => {
      msg = "Schedule added."
      await generate()
      loadSchedules()
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }

    const addSchedule = async (schedule) => {
        console.log(schedule)
        schedule.id = zeros
           await invoke('add_schedule', {schedule: schedule}).then(resolved, rejected)
    }

    const saveSchedule = async (schedule) => {
        console.log(schedule)
           await invoke('update_schedule', {schedule: schedule}).then(resolved, rejected)
    }

    const addEntry = () => {
        entries = [...entries, {
            id: zeros,
            schedule_id: zeros,
            description: (entries.length > 0) ? entries[0].description : "",
            amount: (entries.length == 1) ? entries[0].amount : 0,
            drAmount: (entries.length == 1 && entries[0].entry_type == "Credit") ? entries[0].crAmount : '',
            crAmount: (entries.length == 1 && entries[0].entry_type == "Debit") ? entries[0].drAmount : '',
            account: {},
            entry_type: (entries.length == 1 && entries[0].entry_type == "Debit") ? "Credit" : "Debit"}]
    }

    const handleRemoveClick = () => {
        if (entries.length > minEntries) {
            entries = [...entries.slice(0, entries.length - 1)]
        }
    }


    const validateEntry = (entry, index, errors) => {
        console.log("validate " + entry)
        const prefix =  index + "_"
        if (!entry.description || entry.description.length < 1) {
             errors.addError(prefix + "description", "Description is required")
        }

        if (entry.entry_type === "Credit") {
            if (!entry.crAmount || entry.crAmount.length < 1 || isNaN(entry.crAmount)) {
                errors.addError(prefix + "crAmount", "A valid amount is required")
            }
        } else {
            if (!entry.drAmount || entry.drAmount.length < 1 || isNaN(entry.drAmount)) {
                errors.addError(prefix + "drAmount", "A valid amount is required")
                console.log(entry.drAmount)
            }
        }

        if (!entry.account || !entry.account.id || entry.account.id < 1) {
            errors.addError(index + "_account", "Account is required")
        }

        if (drTotal != crTotal) {
            errors.addError("totals", "Totals should balance")
        }

        return errors
    }

    const showAmount = (entry, type) => {
        if (entry["drAmount"] > 0) {
            entry["entry_type"] = "Debit"
            calculateTotals(entries)
            return type === "Debit"
        }

        if (entry["crAmount"] > 0) {
            entry["entry_type"] = "Credit"
            calculateTotals(entries)
            return type === "Credit"
        }

        return true
    }

    const total = (type) => {
        let total = 0
        entries.filter(e => e.entry_type === type).forEach(e => total += Number(e[type === "Credit" ? "crAmount" : "drAmount"]))
        return total
    }

    const calculateTotals = () => {
        drTotal = total("Debit")
        crTotal = total("Credit")
    }

</script>

<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT ? $_('schedule.edit_schedule') : $_('schedule.new_schedule')}</div>
    <div class="form-row">
        <div class="top-widget">
            <label for="desc">{$_('labels.name')}</label>
            <input id="desc" class="description-input" class:error={errors.isInError("name")} bind:value={name}>
        </div>
    </div>
    <hr/>
    <div class="panel-title">{$_('labels.transaction')}</div>
    <div class="entries">
        <table>
            <tbody>
            <tr><td><div class="heading">{$_('labels.description')}</div></td><td><div class="heading">{$_('labels.account')}</div></td><td><div class="heading">{$_('labels.debit')}</div></td><td><div class="heading">{$_('labels.credit')}</div></td></tr>
            {#each entries as e, i}
            <tr>
                <td class="description"><input id="desc" class="description-input-2" class:error={errors.isInError(i + "_description")} bind:value={e.description}></td>
                <td><div class="select-adjust"><Select bind:item={e["account"]} items={$accounts} label="" none={false} flat={true} inError={errors.isInError(i + "_account")}/></div></td>
                <td class="money">
                    {#if showAmount(e, "Debit")}<input id="amount" class="money-input" class:error={errors.isInError(i + "_drAmount")} bind:value={e.drAmount}>{/if}
                    {#if !showAmount(e, "Debit")}<input id="amount" class="money-input disabled" disabled="disabled">{/if}
                </td>
                <td class="money">
                    {#if showAmount(e, "Credit")}<input id="amount" class="money-input" class:error={errors.isInError(i + "_crAmount")} bind:value={e.crAmount}>{/if}
                    {#if !showAmount(e, "Credit")}<input id="amount" class="money-input disabled" disabled="disabled">{/if}
                </td>
            </tr>
            {/each}
            <tr>
                <td>
                    <div class="toolbar entry-buttons">
                        <i class="gg-add-r" on:click={addEntry}></i>
                        <i class="gg-remove-r" on:click={handleRemoveClick} class:greyed={entries.length <= minEntries}></i>
                    </div>
                </td>
                <td><div class="total">{$_('labels.totals')}</div></td>
                <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={drTotal} disabled="disabled"></td>
                <td class="money"><input id="amount" class="money-input" class:error={errors.isInError("totals")} bind:value={crTotal} disabled="disabled"></td></tr>
            </tbody>
        </table>
    </div>
    <hr/>
    <div class="panel-title">{$_('schedule.schedule')}</div>
    <div class="form-row2">
        <div class="widget">
            {$_('schedule.every')}&nbsp;<input id="amount" class="frequency-input" class:error={errors.isInError("frequency")} bind:value={frequency}>
            &nbsp;<Select bind:item={period} items={periods} flat={true} inError={errors.isInError("period")}/>
            {$_('schedule.starting_from')}&nbsp;<div class="date-input"><DateInput bind:value={date} {format} placeholder="" {min} {max} /></div>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget2">
            <input id="end" type="radio" bind:group={hasEnd} value={true} class="" name="endType"/>
            <div class="widget left"><label for="end">{$_('schedule.end_after')}&nbsp;</label><div class="date-input raise"><DateInput bind:value={endDate} {format} placeholder="" {min} {max} /></div></div>
            <br/>
            <input id="noEnd" type="radio" bind:group={hasEnd} value={false} class="" name="endType"/>
            <label for="noEnd">{$_('schedule.no_end_date')}</label>
        </div>
    </div>
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
            <button on:click={onCancel}>{$_('buttons.close')}</button>
            <button on:click={onAdd}>{addButtonLabel}</button>
        </div>
    </div>
</div>

<style>

    :root {
        --date-input-width: 110px;
    }

    .error-msg {
        color: #FBC969;
    }

    .success-msg {
        color: green;
    }

    .error {
        border: 1px solid #FBC969 !important;
    }


    .buttons {
        float: left;
        margin: 10px 12px 0 0;
    }

    .buttons button {
        min-width: 80px;
    }

    .form-row {
        display: inline-flex;
        float: left;
        width: 100%;
        clear:both;
    }

    .form-row2, .form-button-row {
        display: block;
        text-align: left;
    }

    .form-button-row {
        margin-left: 7px;
        margin-right: 2px;
    }

    input {
        margin-right: 0px;
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

    .widget2 {
        padding: 5px 0px 5px 10px;
        margin: 13px 12px 0px 0px;
    }

    .widget2 label {
        display: inline-block;
        font-size: 1.0em;
    }

    .widget2 input {
        margin: 0px;
    }

    .top-widget {
        display: inline-block;
        padding: 5px 0px 5px 0px;
    }

    td .heading {
        margin-bottom: -1px;
    }

    .money-input {
        width: 100px;
    }
    .frequency-input {
        width: 40px;
        text-align: right;
        background-color: #F0F0F0;
    }

    .raise {
        margin-top: -7px;
    }

    .left {
        padding-left: 0px;
    }

    .money-input {
        text-align: right;
    }

    .description-input {
        width: 400px;
    }

    .date-input {
        float: right;
    }

    .total {
        text-align: right;
        margin: 0px 5px 6px 0px;
    }

    hr {
        border-style: none;
        border: 3px solid #363636;
        margin-left: -20px;
        width: 100vw;

    }

    .entry-buttons {
        float: left;
    }

    .toolbar {
        color: #C0C0C0;
        display: flex;
        -webkit-user-select: none; /* Chrome/Safari */
        -moz-user-select: none; /* Firefox */
        -ms-user-select: none; /* IE10+ */

        /* The rule below is not implemented in browsers yet */
        -o-user-select: none;

        /* The rule below is implemented in most browsers by now */
        user-select: none;
    }

    .toolbar i:hover{
        color: #F0F0F0;
        border-color: #F0F0F0;
        cursor: pointer;
    }

    .toolbar i {
        margin-right: 5px;
    }

    .greyed {
        color: #666;
        border-color: #666;
    }

    .greyed:hover {
        color: #666 !important;
        border-color: #666 !important;
        cursor: default !important;
    }

    .gg-add-r {
        box-sizing: border-box;
        position: relative;
        display: block;
        width: 22px;
        height: 22px;
        border: 2px solid currentColor;
        transform: scale(var(--ggs,1));
        border-radius: 4px
    }

    .gg-add-r::after,
    .gg-add-r::before {
        content: "";
        display: block;
        box-sizing: border-box;
        position: absolute;
        width: 10px;
        height: 2px;
        background: currentColor;
        border-radius: 5px;
        top: 8px;
        left: 4px
    }

    .gg-add-r::after {
        width: 2px;
        height: 10px;
        top: 4px;
        left: 8px
    }

    .gg-remove-r {
        box-sizing: border-box;
        position: relative;
        display: block;
        transform: scale(var(--ggs,1));
        width: 22px;
        height: 22px;
        border: 2px solid ;
        border-radius: 4px
    }

    .gg-remove-r::before {
        content: "";
        display: block;
        box-sizing: border-box;
        position: absolute;
        width: 10px;
        height: 2px;
        background: currentColor;
        border-radius: 5px;
        top: 8px;
        left: 4px
    }

</style>