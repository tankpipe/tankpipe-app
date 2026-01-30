<script>
    import {DateInput} from 'date-picker-svelte'
    import Select from '../Select.svelte'
    import { _ } from 'svelte-i18n'
    import { onMount } from 'svelte';
    import { periods } from '../dates.js'

    let { 
        frequency, 
        period, 
        date, 
        hasEnd, 
        endDate, 
        errors 
    } = $props()

    let max = $state(new Date())
    let min = $state(new Date())
    let format = "yyyy-MM-dd"

    onMount(() => {
        max.setFullYear(date.getFullYear() + 20)
        min.setFullYear(date.getFullYear() - 10)
    })

</script>
<form>
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
</form>

<style>
    .form-row2 {
        display: block;
        text-align: left;
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

    .date-input {
        float: right;
    }

    input {
        margin-right: 0px;
    }
</style>
