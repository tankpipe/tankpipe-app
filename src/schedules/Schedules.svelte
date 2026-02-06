<script>
    import EditSchedule from './EditSchedule.svelte'
    import Schedule from './Schedule.svelte'
    import Icon from '@iconify/svelte'
    import { page, isEditMode, isViewMode, views, modes, isListMode } from '../page'
    import {accounts} from '../accounts'
    import { getEndDate} from './generate'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'
    import { onMount, untrack } from 'svelte'
    import { selector } from '../selector';
    import { Errors  } from '../errors';
    import DateInput from 'date-picker-svelte/DateInput.svelte';
    import Spinner from '../Spinner.svelte';

    let curSchedule = $state()
    let schedules = $state([])
    let msg = $state("")
    let errors = $state(new Errors())
    let max = $state(new Date())
    let min = $state(new Date())
    let format = "yyyy-MM-dd"
    let scheduleToDate = $state()
    let loading = $state(false)

    // Reactively check for load mode when schedules are loaded
    $effect(() => {
        if (schedules.length > 0 && $page.mode === modes.LOAD) {
            if ($page.payload && $page.payload.schedule_id) {
                let match = schedules.filter(s => s.id === $page.payload.schedule_id)
                if (match.length > 0) {
                    untrack(() => {
                        curSchedule = {...match[0]}
                        if (match[0].entries) {
                            curSchedule.entries = [...match[0].entries]
                        }
                        console.log(curSchedule)
                        page.set({view: views.SCHEDULES, mode: modes.EDIT})
                    })
                }
            }
        }
        selector.set({ showMultiEdit: false, }) // hack to hide multiple select in schedules
    })

    onMount(() => {
        loadSchedules()
        getDate()
        const date = new Date()        
        max.setFullYear(date.getFullYear() + 20)
        min.setFullYear(date.getFullYear() - 10)        
    })

    const generateSchedule = async () => {
        loading = true
        console.log("generateSchedule")         
        msg = $_('schedule.generating')
        
        if (scheduleToDate) {
            const isoDateString = scheduleToDate ? scheduleToDate.toISOString().split('T')[0] : null
            console.log("generating to " + isoDateString)
            invoke('generate', {date: {date: isoDateString}}).then(resolved, rejected)
        }
    }

    const resolved = async (result) => {
        msg = $_('schedule.generation_complete')
        loadSchedules()
        loading = false
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
        loading = false
    }

    const close = () => {
        page.set({view: views.SCHEDULES, mode: modes.LIST})
        loadSchedules()
    }

    const edit = () => {
        page.set({view: views.SCHEDULES, mode: modes.EDIT})
    }

    const view = () => {
        page.set({view: views.SCHEDULES, mode: modes.VIEW})
    }


    const selectSchedule = (schedule) => {
        msg = ''
        curSchedule = {...schedule}
        if (schedule.entries) {
            curSchedule.entries = [...schedule.entries]
        }
        page.set({view: views.SCHEDULES, mode: modes.VIEW})
    }

    const editSchedule = (schedule) => {
        msg = ''
        curSchedule = {...schedule}
        if (schedule.entries) {
            curSchedule.entries = [...schedule.entries]
        }
        page.set({view: views.SCHEDULES, mode: modes.EDIT})
    }

    const loadSchedules = async () => {
        console.log("loadSchedules")
        const result = await invoke('schedules')
        const schedulesList = Array.isArray(result) ? [...result] : []
        schedules = schedulesList
    }

    const formatter = new Intl.NumberFormat('en-AU', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    })

    const matchAccount = (accountId) =>  {
        if (!accountId) return null
        let match = $accounts.filter(a => a.id == accountId)
        return match.length > 0 ? match[0] : null
    }

    const getAccountName = (accountId) => {
        const account = matchAccount(accountId)
        return account ? account.name : "None"
    }

    const handleAddClick = () => {
        page.set({view: views.SCHEDULES, mode: modes.NEW})
    }

    const getDate = async () => {
        scheduleToDate = await getEndDate()
    }

</script>
{#if isListMode($page)}
<div class="account-heading">
    <div class="toolbar toolbar-right"><button class="toolbar-icon" onclick={handleAddClick} title={$_('schedules.createNew')}><Icon icon="mdi:plus-box-outline"  width="24"/></button></div>
    <div class="form-heading">{$_('schedules.title')}</div>
    <div class="heading-spinner"><Spinner show={loading}/></div>
</div>
{/if}
{#if isEditMode($page)}
<EditSchedule {close} {curSchedule} {loadSchedules} {view}/>
{/if}
{#if isViewMode($page)}
<Schedule {close} {edit} {curSchedule}/>
{/if}
{#if isListMode($page)}
<div class="controls">
    <div class="form-row">
        <div class="widget">
            <div class="label label-column">{$_('schedules.scheduleUntil')} </div>            
            <div class="inline-button"><button class="og-button" disabled={loading} onclick={generateSchedule}>{$_('schedule.generate')}</button></div>
            <div class="date-input field"><DateInput bind:value={scheduleToDate} {format} placeholder="" {min} {max} closeOnSelection={true}/></div>            
        </div>        
    </div>
     <div class="msg-row">
            {#each errors.getErrorMessages() as e}
                <p class="error-msg">{e}</p>
            {/each}
            {#if msg} 
                <p class="success-msg">{msg}</p>
            {/if}                
            </div>  
    </div>
<div class="scroller">
    {#if schedules.length < 1}
    <div class="message">{$_('schedules.noSchedules')}</div>
    {/if}
        {#each schedules as s}
            <div class="card" onclick={() => selectSchedule(s)} onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectSchedule(s); } }} tabindex="0" role="button">
                <div class="row">
                    <div class="widget">
                        <div class="card-title">{s.name}</div>
                        <div class="toolbar toolbar-right">
                            <button class="edit-icon" onclick={(e) => { e.stopPropagation(); editSchedule(s); }} title={$_('buttons.edit')}><Icon icon="mdi:pencil" /></button>
                        </div>
                    </div>
                </div>
                <hr/>
                <div class="row">
                    <div class="widget schedule-entries">
                    <table>
                        <tbody>
                        {#each s.entries as e}
                        <tr>
                            <td>
                                <div class="description">{getAccountName(e.account_id)}</div>
                                <div class="description tiny">{e.description}</div>
                            </td>
                            <td>{#if e.entry_type == "Debit"} <div class="money">{formatter.format(e.amount)}</div>{/if}</td>
                            <td>{#if e.entry_type == "Credit"} <div class="money">{formatter.format(e.amount)}</div>{/if}</td>
                        </tr>
                        {/each}
                        </tbody>
                    </table>
                </div>
                </div>
                <div class="row">
                    <div class="bottom-widget">
                        <div class="label card-label">{$_('schedules.last')}</div>
                        <div class="last-date">{s.last_date == "null"?"-":s.last_date}</div>
                    </div>
                </div>
            </div>
        {/each}
</div>
{/if}

<style>
    .inline-button {
        float: right;
        margin: 0px 0px 0px 3px;
    }

    .inline-button button {
        height: 33px !important;
    }

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

    .scroller{
        height: 100%;
        width: 100%;
        overflow: scroll;
        margin-top: 15px;
    }

    .widget {
        display: inline-block;
        text-align: left;
        margin: 10px 10px;
        color: #F0F0F0;
        vertical-align: top;
    }

    .bottom-widget {
        display: inline-block;
        text-align: left;
        margin: 0px 10px 5px 15px;
        color: #F0F0F0;
        vertical-align: top;
    }

    .schedule-entries {
        font-size: .8em;
        color: #C0C0C0;
    }

    .row {
        display: block;
        text-align: left;
    }

    .label {
        font-size: .8em;
        color: #aaa !important;
        margin: 0 !important;
    }
    
    .card-label {
        padding: 0 !important;
        font-size: .7em !important;
        line-height: 1em !important;
    }

    .scroller {
        clear: both;
        margin-top: 0;
    }

    .card {
        float: left;
        clear: both;
        margin: 10px;
        background-color: #524e4e;
        padding: 5px;
        border-radius: 10px;
    }

    .card:hover {
        cursor: pointer;
    }

     .edit-icon {
        float: right;
        color: #524e4e;
    }

    .card:hover .edit-icon {
        color: #666;
    }

    .edit-icon:hover {
        color: #C0C0C0 !important;
        cursor: pointer;
    }

    .money {
        text-align: right !important;
        min-width: 100px;
    }

    .card-title {
        min-width: 500px;
        white-space: nowrap;
        font-weight: bold;
        float: left;
    }

    .description {
        min-width: 316px;
        max-width: 33vw;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .tiny {
        font-size: 0.5em;
        color: #878787;
    }

    .last-date {
        min-width: 200px;
        white-space: nowrap;
        display: inline-block;
        font-size: .7em;
    }

    hr {
        border: 1px solid #444;
        margin: 0 -5px;
    }

    .message {
        color: #EFEFEF;
        margin-bottom: 20px;
        text-align: left;
        background-color: #303030;
        padding:10px;
        border-radius: 10px;
    }

    .controls {
        text-align: center;
    }

    .form-row {
        display: block;
        float: left;
        clear: both;
        margin-top: -10px;
    }

    .msg-row {
        display: block;
        float: left;
        clear: both;
        margin: -10px 0px 0px 10px;        
    }

    .date-input {
        float: right;
    }

    .label {
        font-size: .9em;
        color: #aaa !important;
        margin: 0 5px 5px 0;
        display: inline-block;
        text-align: left;
        line-height: 36px;
        padding-right: .5em;
    }

    .field {
        text-align: left;
        display: inline-block;
    }

    .heading-spinner {
        margin: 3px 0 0 10px;
        float: left;
    }

</style>