<script>
    import EditSchedule from './EditSchedule.svelte'
    import Icon from '@iconify/svelte'
    import { page, isEditMode, views, modes } from './page'
    import {accounts} from './accounts'
    import {generate, getEndDate} from './generate'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'

    let curSchedule
    let dateStr

    $: {
        loadSchedules()
    }

    const updateSchedule = async () => {
        console.log("updateSchedule")
        await generate(dateStr)
        loadSchedules()
    }

    const close = () => {
        console.log("close")
        page.set({view: views.SCHEDULES, mode: modes.LIST})
        loadSchedules()
    }

    const selectSchedule = (schedule) => {
        curSchedule = schedule
        page.set({view: views.SCHEDULES, mode: modes.EDIT})
    }

    let schedules = []
    const loadSchedules = async () => {
        console.log("loadSchedules")
        schedules = await invoke('schedules')
        checkForLoadMode()
    }

    const checkForLoadMode = () => {
        if ($page.mode === modes.LOAD) {
            if ($page.payload && $page.payload.schedule_id) {
                let match = schedules.filter(s => s.id === $page.payload.schedule_id)
                if (match.length > 0) {
                    curSchedule = match[0]
                    console.log(curSchedule)
                    page.set({view: views.SCHEDULES, mode: modes.EDIT})
                }
            } else { // Should never happen
                console.log("Couldn't find schedule " + $page.payload)
                page.set({view: views.SCHEDULES, mode: modes.LIST})
            }
        }
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
        dateStr = await getEndDate()
    }
    getDate()

</script>

<div class="account-heading">
    {#if !isEditMode($page)}
    <div class="toolbar"><div class="toolbar-icon" on:click={handleAddClick} title={$_('schedules.createNew')}><Icon icon="mdi:plus-box-outline"  width="24"/></div></div>
    <div class="form-heading">{$_('schedules.title')}</div>
    {/if}
</div>
{#if isEditMode($page)}
<EditSchedule {close} {curSchedule} {loadSchedules}/>
{/if}
{#if !isEditMode($page)}
<div class="controls">
    <div class="form-row2">
        <div class="widget">
            <div class="label label-column">{$_('schedules.scheduleUntil')} </div><div class="date-input field"><input type="date" bind:value={dateStr} on:change={updateSchedule}/></div>
        </div>
    </div>
</div>
<div class="scroller">
    {#if schedules.length < 1}
    <div class="message">{$_('schedules.noSchedules')}</div>
    {/if}
        {#each schedules as s}
            <div class="card" on:click={() => selectSchedule(s)}>
                <div class="row">
                    <div class="widget">
                        <div class="description">{s.name}</div>
                    </div>
                </div>
                <hr/>
                <div class="row">
                    <div class="widget schedule-entries">
                    <table>
                        <tbody>
                        {#each s.entries as e}
                        <tr>
                            <td>{getAccountName(e.account_id)}</td>
                            <td>{#if e.entry_type == "Debit"} <div class="money">{formatter.format(e.amount)}</div>{/if}</td>
                            <td>{#if e.entry_type == "Credit"} <div class="money">{formatter.format(e.amount)}</div>{/if}</td>
                        </tr>
                        {/each}
                        </tbody>
                    </table>
                </div>
                </div>
                <div class="row">
                    <div class="widget">
                        <div class="label">{$_('schedules.last')}</div>
                        <div class="account">{s.last_date == "null"?"-":s.last_date}</div>
                    </div>
                </div>
            </div>
        {/each}
</div>
{/if}

<style>
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

    .schedule-entries {
        font-size: .9em;
        color: #C0C0C0;
    }

    .row {
        display: block;
        text-align: left;
    }

    .label {
        font-size: .8em;
        color: #aaa !important;
        margin-bottom: 5px;
    }

    .scroller {
        clear: both;
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

    .money {
        text-align: right !important;
        min-width: 100px;
    }

    .description {
        min-width: 500px;
        white-space: nowrap;
        font-weight: bold;
    }

    .account {
        min-width: 200px;
        white-space: nowrap;
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

    .toolbar {
        float: right;
        color: #C0C0C0;
        margin-bottom: 10px;
    }

    .toolbar-icon:hover{
        color: #F0F0F0;
        cursor: pointer;
    }

    .controls {
        text-align: center;
    }

    .form-row2 {
        display: block;
        float: left;
        clear: both;
        margin-top: -10px;
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
        padding-right: .5em;
    }

    .field {
        text-align: left;
        display: inline-block;
    }

    .controls input {
        background-color: #aaa;
    }

</style>