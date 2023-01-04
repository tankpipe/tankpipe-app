<script>
	import {settings} from './settings.js'

    let dateStr

	$: {
		if (dateStr) {
			generate()
		}
	}

	function resolved(result) {
      const msg = "Generation complete."
	  console.log(msg)
    }

    function rejected(result) {
        errors = new Errors()
        errors.addError("all", "We hit a snag: " + result)
		console.log(result)
    }

	const generate = async () => {
		if (dateStr) {
			console.log("generating to " + dateStr)
			await invoke('generate', {date: {date: dateStr}}).then(resolved, rejected)
		}
	}

	const updateSettings = async () => {
		if ($settings) {
			await invoke('update_settings', {settings: $settings}).then(
				() => console.log("settings saved"),
				() => console.log("settings not saved " + result)
			)
		}
	}


	const getEndDate = async () => {
        console.log("getEndDate")
   		let tempDate = await invoke('end_date')
		if (tempDate) dateStr = tempDate.date

	}
	getEndDate()

</script>
<div class="controls">
	<div class="form-heading">Settings</div>
    <div class="form-row2">
		<div class="widget">
			<div class="label">Enforce double entry</div><input type="checkbox" bind:checked={$settings.require_double_entry} on:change={updateSettings}/>
		</div>
	</div>
    <div class="form-row2">
		<div class="widget">
			<div class="label">Schedule until </div><div class="date-input"><input type="date" bind:value={dateStr}/></div>
		</div>
	</div>
</div>
<style>
	.controls {
		text-align: center;
	}

	.form-row2 {
        display: block;
    }

    .widget {
        padding: 5px 0px 5px 10px;
		float: left;
		clear: both;
    }

	.date-input {
        float: right;
    }

    .date-input input {
        border: none;
    }

	.label {
		font-size: .8em;
		color: #aaa !important;
		margin: 0 5px 5px 0;
        display: inline;
		vertical-align: middle;
		line-height: 36px;
	}

    .controls input {
        background-color: #aaa;
    }

</style>