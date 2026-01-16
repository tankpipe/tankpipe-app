<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import Icon from '@iconify/svelte'
    import {generate} from './generate.js'
    import {config} from './config.js'
    import { invoke } from '@tauri-apps/api/core'
    import { _ } from 'svelte-i18n'
    import TransactionList from './TransactionList.svelte'
    import { onMount } from 'svelte';

    let { close, edit, curSchedule } = $props()

    let hasEnd = $state(false)
    let msg = $state("")
    let successMsg = $state("")
    let errors = $state(new Errors())
    let date = $state(new Date())
    let name = $state()
    let frequency = $state(1)
    let endDate = $state()
    let lastDate = $state()
    let max = $state(new Date())
    let min = $state(new Date())
    let format = "yyyy-MM-dd"
    let period = $state({value:"Months", name:"Months"})
    const periods = [{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, {value:"Months", name:"Months"}, {value:"Years", name:"Years"}]
    let transactions = $state([])
    $inspect(curSchedule)
    $inspect(lastDate)

    onMount(() => {
        name = curSchedule.name
        period = matchPeriod(curSchedule.period)
        frequency = curSchedule.frequency
        endDate = curSchedule.end_date == "null" ? null : new Date(curSchedule.end_date)
        lastDate = curSchedule.last_date == "null" ? null : new Date(curSchedule.last_date)
        hasEnd = endDate != null
        console.log("hasEnd: ",(endDate != null))
        date = new Date(curSchedule.start_date)
        max.setFullYear(date.getFullYear() + 20)
        min.setFullYear(date.getFullYear() - 10)        
    })

    const matchPeriod = (value) =>  {
        if (!value) return null
        let match = periods.filter(p => p.value == value)
        return match.length > 0 ? match[0] : null
    }

    const formatDate = (date) => {
        switch ($config.display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return transaction.date
            default: return date.toLocaleDateString()
        }
    }

    $effect(() => {
        if (curSchedule && curSchedule.id) {
            loadTransactions()
        }
    })

    const loadTransactions = async () => {
        console.log("loadScheduleTransactions")
        transactions = await invoke("schedule_transactions", { scheduleId: curSchedule.id, status: "Projected" })
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


    const saveSchedule = async (schedule) => {
        console.log(schedule)
           await invoke('update_schedule', {schedule: schedule}).then(resolved, rejected)
    }

    const generateSchedule = async () => {        
        const isoDateString = lastDate ? lastDate.toISOString().split('T')[0] : null
        await invoke('generate_by_schedule', { 
            date: {date: isoDateString}, 
            scheduleId: curSchedule.id 
        }).then(resolvedGenerateSchedule, rejected)
    }
    
    function resolvedGenerateSchedule(result) {
        loadTransactions()
        msg = $_('schedule.generation_complete')
    }

    const deleteTransactions = async () => {
        const transactionIds = transactions.map(t => t.id)
        if (transactionIds.length > 0) {
            await invoke('delete_transactions', {ids: transactionIds}).then(resolvedDelete, rejected)
        }
    }

    function resolvedDelete(result) {
       msg = $_('transactions.transactionsDeleted')
      loadTransactions()
    }

</script>

<div class="form">
    <div class="form-heading">{$_('schedule.schedule')}</div>
    <div class="toolbar toolbar-right">
        <button class="toolbar-icon" onclick="{edit}" title={$_('schedule.edit')}><Icon icon="mdi:edit-box-outline"  width="24"/></button>
        <button class="toolbar-icon" onclick="{close}" title={$_('buttons.close')}><Icon icon="mdi:close-box-outline"  width="24"/></button>
    </div>
    <div class="form-row">
        <div class="top-widget">
            <label for="desc">{name}</label>
        </div>
    </div>
    <div class="form-row">
        <div class="small-text">
            {$_('schedule.every')}&nbsp;{frequency}&nbsp;{period.name}
            {$_('schedule.starting_from')}&nbsp;{formatDate(date)} 
        </div>
    </div>
    {#if hasEnd}
    <div class="form-row">
        <div class="small-text">
            {$_('schedule.no_end_date')}&nbsp;&nbsp;&nbsp;&nbsp;{endDate}
        </div>
    </div>
    {/if} 
    {#if lastDate}
    <div class="form-row">
        <div class="small-text">
            {$_('schedule.last_date')}&nbsp;&nbsp;&nbsp;&nbsp;{lastDate}
        </div>
    </div>
    {/if} 
    <hr/>
    <div class="form-row">
        <div class="schedule-row">
            <div class="widget left">
                <label for="lastDate">{$_('schedule.schedule_until')}&nbsp;</label>
                <div class="inline-button"><button class="og-button" onclick={generateSchedule}>{$_('schedule.generate')}</button></div>
                <div id="lastDate" class="date-input raise"><DateInput bind:value={lastDate} {format} placeholder="" {min} {max} /></div>
            </div>
            <div>
            {#each errors.getErrorMessages() as e}
                <p class="error-msg">{e}</p>
            {/each}
            {#if msg} 
                <p class="success-msg">{msg}</p>
            {/if}                
            </div>            
        </div>
    </div>
</div>
<hr/>
<div>
<div class="panel-title float-left">{$_('schedule.projected_transactions')}</div>
<div class="toolbar toolbar-right">
    <button class="toolbar-icon" onclick="{deleteTransactions}" title={$_('schedule.delete_projected_transactions')}><Icon icon="mdi:trash-can-outline"  width="24"/></button>
</div>
</div>
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

    .form-row {
        display: inline-flex;
        float: left;
        width: 100%;
        clear:both;
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

    .schedule-row {
        padding: 5px 0 0px 0;
        margin: 0 0 -6px 0;
    }

    .schedule-row label {
        display: inline-block;
        font-size: 1.0em;
    }

    .schedule-row button {
        min-height: 33px;
    }

    .top-widget {
        display: inline-block;
        padding: 5px 0px 5px 0px;
    }

    .raise {
        margin-top: -7px;
    }

    .left {
        padding-left: 0px;
    }

    .float-left {
        float: left;
    }

    .date-input {
        float: right;
    }

    hr {
        border-style: none;
        border: 1px solid #363636;
        margin-left: -20px;
        width: 100vw;
    }


    .inline-button {
        float: right;
        margin: -7px 0px 0px 3px;
    }

     .small-text {
        font-size: 0.7em;
        color: #878787;
        margin: 3px 0 -5px 2px;
        min-height: 27px;
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