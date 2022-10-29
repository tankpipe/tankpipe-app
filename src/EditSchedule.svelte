<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    import { page, modes } from './page.js'

    export let close
    export let curSchedule
    export let accounts = []

    let hasEnd = false
    let drAccount
    let crAccount
    let msg = ""
    let errors = new Errors();
    let date = new Date(), name, description, amount, frequency = 1
    let endDate
    let max = new Date(), min = new Date()
    max.setFullYear(date.getFullYear() + 20);
    min.setFullYear(date.getFullYear() - 10);
    console.log(max)
    let format="yyyy-MM-dd"
    let addButtonLabel = "Add"
    let period = {value:"Months", name:"Months"}
    const periods = [{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, period, {value:"Years", name:"Years"}]

    onMount(() => {
        if ($page.mode === modes.EDIT) {
            name = curSchedule.name
            description = curSchedule.description
            amount = curSchedule.amount
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

            if ($page.payload && $page.payload.values) {
                console.log($page.payload)
                name = $page.payload.values.description
                description = $page.payload.values.description
                amount = $page.payload.values.amount
                drAccount = $page.payload.values.debitAccount
                crAccount = $page.payload.values.creditAccount
            }
        }
    });

    const matchAccount = (accountId) =>  {
        if (!accountId) return null
        let match = accounts.filter(a => a.id == accountId)
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
        msg = "";
        errors = new Errors();
        if (!name || name.length < 1) {
             errors.addError("description", "Description is required")
        }

        if (!date || date.length < 1) {
            errors.addError("date", "Date is required")
        }

        if (!amount || amount.length < 1 || isNaN(amount)) {
            errors.addError("amount", "A valid amount is required")
        }

        if (!drAccount && !crAccount) {
            errors.addError("accounts", "At least one account needs to be selected")
        }

        if (!period) {
            errors.addError("period", "A period needs to be selected")
        }

        if (!errors.hasErrors()) {
            let drAccountId = drAccount? drAccount.id : null
            let crAccountId = crAccount? crAccount.id : null
            let dateStr = date.getFullYear()+ "-" + (date.getMonth() + 1) + "-" + date.getDate()
            let endDateStr = hasEnd ? endDate.getFullYear()+ "-" + (endDate.getMonth() + 1) + "-" + endDate.getDate() : "null"

            let schedule = {
                    name: name,
                    period: period.value,
                    frequency: parseInt(frequency),
                    start_date: dateStr,
                    last_date: "null",
                    end_date: endDateStr,
                    amount: amount,
                    description: description ? description : name,
                    dr_account_id: drAccountId,
                    cr_account_id: crAccountId,
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

    /*
            id: Uuid::new_v4(),
			name: self.name,
			period: self.period,
			frequency: self.frequency,
            start_date: self.start_date,
			last_date: self.last_date,
			amount: self.amount,
            description: self.description,
			dr_account_id: self.dr_account_id,
            cr_account_id: self.cr_account_id,
    */

    function resolved(result) {
      msg = "Schedule added."
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
    }
    const addSchedule = async (schedule) => {
        console.log(schedule)
   		await invoke('add_schedule', {schedule: schedule}).then(resolved, rejected)
	};

    const saveSchedule = async (schedule) => {
        console.log(schedule)
   		await invoke('update_schedule', {schedule: schedule}).then(resolved, rejected)
	};

</script>

<div class="form">
    <div class="form-heading">{$page.mode === modes.EDIT?"Edit":"New"} Schedule</div>
    <div class="form-row">
        <div class="widget">
            <label for="desc">Name</label>
            <input id="desc" class="description-input" class:error={errors.isInError("name")} bind:value={name}>
        </div>
    </div>
    <hr/>
    <div class="panel-title">Transaction</div>
    <div class="form-row">
        <div class="widget">
            <label for="desc">Description</label>
            <input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={description} placeholder={name}>
        </div>
        <div class="widget">
            <label for="amount">Amount</label>
            <input id="amount" class="money-input" class:error={errors.isInError("amount")} bind:value={amount}>
        </div>
    </div>
    <div class="form-row2">
        <Select bind:item={drAccount} items={accounts} label="Debit" none={true} inError={errors.isInError("accounts")} flat={true}/>
        <Select bind:item={crAccount} items={accounts} label="Credit" none={true} inError={errors.isInError("accounts")} flat={true}/>
    </div>
    <hr/>
    <div class="panel-title">Schedule</div>
    <div class="form-row2">
        <div class="widget">
            Every&nbsp;<input id="amount" class="frequency-input" class:error={errors.isInError("frequency")} bind:value={frequency}>
            &nbsp;<Select bind:item={period} items={periods} flat={true} inError={errors.isInError("period")}/>
            starting from&nbsp;<div class="date-input"><DateInput bind:value={date} {format} placeholder="" {min} {max} /></div>
        </div>
    </div>
    <div class="form-row2">
        <div class="widget2">
            <input id="end" type="radio" bind:group={hasEnd} value={true} class="" name="endType"/>
            <div class="widget left"><label for="end">End after&nbsp;</label><div class="date-input raise"><DateInput bind:value={endDate} {format} placeholder="" {min} {max} /></div></div>
            <br/>
            <input id="noEnd" type="radio" bind:group={hasEnd} value={false} class="" name="endType"/>
            <label for="noEnd">No end date</label>
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
            <button on:click={onCancel}>Close</button>
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

    hr {
        border-style: none;
        border: 3px solid #363636;
        margin-left: -20px;
        width: 100vw;

    }

</style>