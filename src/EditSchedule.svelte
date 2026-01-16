<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import {page, modes} from './page.js'
    import Icon from '@iconify/svelte'
    import {accounts} from './accounts'
    import {generate} from './generate'
    import {settings} from './settings.js'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import TransactionList from './TransactionList.svelte'

    let { close, curSchedule, loadSchedules } = $props()

    const zeros = '00000000-0000-0000-0000-000000000000'
    let minEntries = $derived($settings.require_double_entry ? 2 : 1)
    let hasEnd = $state(false)
    let msg = $state("")
    let successMsg = $state("")
    let errors = $state(new Errors())
    let date = $state(new Date())
    let name = $state()
    let description = $state()
    let amount = $state()
    let frequency = $state(1)
    let endDate = $state()
    let lastDate = $state()
    let max = $state(new Date())
    let min = $state(new Date())
    let format = "yyyy-MM-dd"
    let addButtonLabel = $state("Add")
    let period = $state({value:"Months", name:"Months"})
    const periods = [{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, {value:"Months", name:"Months"}, {value:"Years", name:"Years"}]
    let drAccount = $state()
    let crAccount = $state()
    let entries = $state([])
    let transactions = $state([])
    $inspect(curSchedule)
    $inspect(lastDate)

    const getEntryType = (entry) => {
        if (entry.drAmount > 0) {
            return "Debit"
        }
        if (entry.crAmount > 0) {
            return "Credit"
        }
        return entry.entry_type || "Debit"
    }

    // Derived totals that update automatically when entries change
    let drTotal = $derived.by(() => {
        let total = 0
        entries.forEach(e => {
            const eType = getEntryType(e)
            if (eType === "Debit") {
                total += Number(e.drAmount || 0)
            }
        })
        return total
    })

    let crTotal = $derived.by(() => {
        let total = 0
        entries.forEach(e => {
            const eType = getEntryType(e)
            if (eType === "Credit") {
                total += Number(e.crAmount || 0)
            }
        })
        return total
    })

    onMount(() => {
        if ($page.mode === modes.EDIT) {
            name = curSchedule.name
            description = curSchedule.description
            amount = curSchedule.amount
            // Create deep copies of entries to avoid mutation issues
            entries = curSchedule.entries.map(e => {
                const entryCopy = {...e}
                entryCopy.entry_type === "Credit" ? entryCopy.crAmount = e.amount : entryCopy.drAmount = e.amount
                entryCopy.realDate = new Date(e.date)
                entryCopy.account = matchAccount(e.account_id)
                return entryCopy
            })
            addButtonLabel = $_('buttons.update')
            successMsg = $_('schedule.updated')
            drAccount = matchAccount(curSchedule.dr_account_id)
            crAccount = matchAccount(curSchedule.cr_account_id)
            period = matchPeriod(curSchedule.period)
            frequency = curSchedule.frequency
            endDate = curSchedule.end_date == "null" ? null : new Date(curSchedule.end_date)
            lastDate = curSchedule.last_date == "null" ? null : new Date(curSchedule.last_date)
            hasEnd = endDate != null
            date = new Date(curSchedule.start_date)
            max.setFullYear(date.getFullYear() + 20)
            min.setFullYear(date.getFullYear() - 10)
        } else if ($page.mode === modes.NEW) {
            drAccount = null
            crAccount = null
            addButtonLabel = $_('buttons.add')
            successMsg = $_('schedule.created')

            if ($page.payload && $page.payload.entries) {
                console.log($page.payload)
                // Create deep copies of entries
                entries = $page.payload.entries.map(e => ({...e, schedule_id: zeros}))
                name = $page.payload.entries[0].description
            } else {
                for (var i = 0; i < minEntries; i++) {
                    addEntry()
                }
            }
        }

    })

    $effect(() => {
        if (curSchedule && curSchedule.id) {
            loadTransactions()
        }
    })

    const loadTransactions = async () => {
        console.log("loadScheduleTransactions")
        transactions = await invoke("schedule_transactions", { scheduleId: curSchedule.id, status: "Projected" })
    }

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
            // Update entry types before processing for saving
            updateEntryTypes()

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

    function resolvedSchedule(result) {
      msg = "Schedule deleted."
      close()
    }

    const deleteSchedule = async () => {
        if (curSchedule && curSchedule.id) {
            await invoke('delete_schedule', {id: curSchedule.id}).then(resolvedSchedule, rejected)
        } else {
            close()
        }
    }

    const resolved = async (result) => {
      msg = successMsg
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
        // Compute entry type on the fly without mutating
        const entryType = getEntryType(entry)
        return type === entryType
    }

    // Update entry_type before saving (called only when needed, not during render)
    const updateEntryTypes = () => {
        entries.forEach(entry => {
            entry.entry_type = getEntryType(entry)
        })
    }

</script>

<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT ? $_('schedule.edit_schedule') : $_('schedule.new_schedule')}</div>
    <div class="toolbar toolbar-right">
        <button class="toolbar-icon" onclick="{deleteSchedule}" title={$_('schedule.delete')}><Icon icon="mdi:trash-can-outline"  width="24"/></button>
    </div>
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
                        <i class="gg-add-r" onclick={addEntry}></i>
                        <i class="gg-remove-r" onclick={handleRemoveClick} class:greyed={entries.length <= minEntries}></i>
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
            <input id="noEnd" type="radio" bind:group={hasEnd} value={false} class="" name="endType"/>
            <label for="noEnd">{$_('schedule.no_end_date')}&nbsp;&nbsp;&nbsp;&nbsp;</label>
            <input id="end" type="radio" bind:group={hasEnd} value={true} class="" name="endType"/>
            <div class="widget left"><label for="end">{$_('schedule.end_after')}&nbsp;</label><div class="date-input raise"><DateInput bind:value={endDate} {format} placeholder="" {min} {max} /></div></div>
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
            <button class="og-button" onclick={onCancel}>{$_('buttons.close')}</button>
            <button class="og-button" onclick={onAdd}>{addButtonLabel}</button>
        </div>
    </div>
</div>
<hr class="fat-hr"/>
<div class="panel-title">{$_('schedule.projected_transactions')}</div>
<TransactionList curAccount={{}} journalMode={true} transactions={transactions} onSelect={()=>{}} />
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
        float: right;
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
        border: 1px solid #363636;
        margin-left: -20px;
        width: 100vw;
    }

    .fat-hr {
        border: 3px solid #363636;
    }

    .entry-buttons {
        float: left;
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