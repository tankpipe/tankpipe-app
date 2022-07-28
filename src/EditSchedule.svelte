<script>
    import {DateInput} from 'date-picker-svelte'
    import {Errors} from './errors.js'
    import {onMount} from "svelte"
    import Select from './Select.svelte'
    
    export let close
    export let curSchedule
    export let accounts = []
    export let editMode = "ADD"
    
    let drAccount
    let crAccount
    let msg = ""
    let errors = new Errors();
    let date = new Date(), name, amount, frequency = 1 
    let format="yyyy-MM-dd"
    let addButtonLabel = "Add"
    let period = {value:"Monthly", name:"Months"}
    let periods = [{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, {value:"Months", name:"Months"}, {value:"Years", name:"Years"}]
    
    onMount(() => { 
        console.log(editMode, curSchedule)        
        if (editMode == "EDIT") {
            name = curSchedule.name
            amount = curSchedule.amount
            addButtonLabel = "Update"
            drAccount = matchAccount(curSchedule.dr_account_id)
            crAccount = matchAccount(curSchedule.cr_account_id)
            period = matchPeriod(curSchedule.period)
            frequency = curSchedule.frequency

        } else {
            drAccount = null
            crAccount = null
            addButtonLabel = "Add"
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

        if (!errors.hasErrors()) {
            let drAccountId = drAccount? drAccount.id : null
            let crAccountId = crAccount? crAccount.id : null
            let dateStr = date.getFullYear()+ "-" + (date.getMonth()+1) + "-" + date.getDate()

            if (editMode == "ADD") {
                const schedule = {
                    name: name, 
                    period: period.value,
                    frequency: parseInt(frequency),
                    start_date: dateStr, 
                    last_date: dateStr,
                    amount: amount,  
                    description: name,                    
                    dr_account_id: drAccountId,
                    cr_account_id: crAccountId,
                }

                addSchedule(schedule)
            } else if (editMode == "EDIT") {
                const schedule = {
                    id: curSchedule.id,                    
                    name: name, 
                    period: period.value,
                    frequency: parseInt(frequency),
                    start_date: dateStr, 
                    last_date: dateStr,
                    amount: amount,  
                    description: name,                    
                    dr_account_id: drAccountId,
                    cr_account_id: crAccountId,
                }
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
      msg = "Transaction added."
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
    <div class="panel">
        <div class="form-row">
            <div class="widget">
                <label for="desc">Description</label>
                <input id="desc" class="description-input" class:error={errors.isInError("description")} bind:value={name}>                
            </div>
            <div class="widget">
                <label for="amount">Amount</label>
                <input id="amount" class="money-input" class:error={errors.isInError("amount")} bind:value={amount}>
            </div>
        </div>
        <div class="form-row2">
            <Select bind:item={drAccount} items={accounts} label="Debit" none={true}/>
            <Select bind:item={crAccount} items={accounts} label="Credit" none={true} />
        </div>
        <div class="form-row2">
            <div class="widget">
                Every&nbsp;<input id="amount" class="frequency-input" class:error={errors.isInError("frequency")} bind:value={frequency}>
                &nbsp;<Select bind:item={period} items={periods} flat={true}/>  
                starting from&nbsp;<div class="date-input"><DateInput bind:value={date} {format} placeholder="" /></div>        
            </div>
        </div>
    </div>
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

    :global(.date-time-field input) {
        border: none !important;
        border-radius: 2px !important;
        height: 33px;
    }

    :root {
		--date-input-width: 110px;	
	}

    .error-msg {
        color: red;
    }

    .success-msg {
        color: green;
    }

    .error {
        border: 1px solid red !important;
    }

    :global(.error-input input) {
        border: 1px solid red !important;
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
        background-color: #F0f0f0;        
        margin-top: 20px;
        border-radius: 10px;
    }

    .form label {
        text-align: left;
        font-size: .8em;
        color: #333;
        margin-bottom: 3px;
    }

    .panel {
        background-color: #E0E0E0;
        margin: 15px 15px 0 15px;
        border-radius: 10px;
        padding-right: 10px;
    }

    .widget {
        display: inline-block;
        padding: 5px 0px 5px 10px;        
    }

    .widget p {
        max-width: 500px;
        font-size: 0.9em;
    }

    .money-input {
		width: 100px;
	}
    .frequency-input {
		width: 40px;
        text-align: right;
        border: none;
        background-color: #F0F0F0;
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

</style>