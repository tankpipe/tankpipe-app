<script>
    import Icon from '@iconify/svelte'
    import { _ } from 'svelte-i18n'
    import { onMount } from 'svelte'
    import {config} from '../config.js'

    let { close, edit, curModifier } = $props()

    let hasEnd = $state(false)
    let date = $state(new Date())
    let name = $state()
    let amount = $state(0)
    let percentage = $state(0)
    let frequency = $state(1)
    let endDate = $state()
    let period = $state({value:"Months", name:"Months"})
    const periods = [{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, {value:"Months", name:"Months"}, {value:"Years", name:"Years"}]

    onMount(() => {
        name = curModifier.name
        amount = curModifier.amount
        percentage = curModifier.percentage
        frequency = curModifier.frequency
        period = matchPeriod(curModifier.period)
        endDate = curModifier.end_date == "null" ? null : new Date(curModifier.end_date)
        date = new Date(curModifier.start_date)
        hasEnd = endDate != null
    })

    const formatDate = (date) => {
        switch ($config.display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return transaction.date
            default: return date.toLocaleDateString()
        }
    }

    const matchPeriod = (value) =>  {
        if (!value) return null
        let match = periods.filter(p => p.value == value)
        return match.length > 0 ? match[0] : null
    }

</script>

<div class="form">
    <div class="form-heading">{$_('modifier.title')}</div>    
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
            {#if percentage >= 0}
            {percentage * 100}{$_('modifier.increase')}                        
            {/if}
            {#if percentage < 0}
            {percentage * -100}{$_('modifier.decrease')}                        
            {/if}
        </div>
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
    <hr/>
    
</div>

<style>

    :root {
        --date-input-width: 110px;
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

    .top-widget {
        display: inline-block;
        padding: 5px 0px 5px 0px;
    }

    hr {
        border-style: none;
        border: 1px solid #363636;
        margin-left: -20px;
        width: 100vw;
    }

   
    .small-text {
        font-size: 0.7em;
        color: #878787;
        margin: 3px 0 -5px 2px;
        min-height: 27px;
    }

    .heading-spinner {
        margin: 3px 0 0 10px;
        float: left;
    }

</style>