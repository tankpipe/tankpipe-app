<script>
    import EditModifier from './EditModifier.svelte'
    import Modifier from './Modifier.svelte'
    import Icon from '@iconify/svelte'
    import { page, isEditMode, isViewMode, views, modes, isListMode } from '../page'
    import { invoke } from "@tauri-apps/api/core"
    import { _ } from 'svelte-i18n'
    import { onMount, untrack } from 'svelte'
    import { Errors  } from '../errors'
    import Spinner from '../Spinner.svelte'
    import {config} from '../config.js'


    let period = $state({value:"Months", name:"Months"})
    const periods = [{value:"Days", name:"Days"}, {value:"Weeks", name:"Weeks"}, {value:"Months", name:"Months"}, {value:"Years", name:"Years"}]

    let curModifier = $state()
    let modifiers = $state([])
    let msg = $state("")
    let errors = $state(new Errors())
    let loading = $state(false)

    // Reactively check for load mode when modifiers are loaded
    $effect(() => {
        if (modifiers.length > 0 && $page.mode === modes.LOAD) {
            if ($page.payload && $page.payload.modifier_id) {
                let match = modifiers.filter(m => m.id === $page.payload.modifier_id)
                if (match.length > 0) {
                    untrack(() => {
                        curModifier = {...match[0]}
                        console.log(curModifier)
                        page.set({view: views.MODIFIERS, mode: modes.EDIT})
                    })
                }
            }
        }
    })

    onMount(() => {
        
        loadModifiers()
    })

    const matchPeriod = (value) =>  {
        if (!value) return null
        let match = periods.filter(p => p.value == value)
        return match.length > 0 ? match[0] : null
    }

    const close = () => {
        page.set({view: views.MODIFIERS, mode: modes.LIST})
        loadModifiers()
    }

    const edit = () => {
        page.set({view: views.MODIFIERS, mode: modes.EDIT})
    }

    const view = () => {
        page.set({view: views.MODIFIERS, mode: modes.VIEW})
    }

    const selectModifier = (modifier) => {
        msg = ''
        curModifier = {...modifier}
        page.set({view: views.MODIFIERS, mode: modes.VIEW})
    }

    const editModifier = (modifier) => {
        msg = ''
        curModifier = {...modifier}
        page.set({view: views.MODIFIERS, mode: modes.EDIT})
    }

    const loadModifiers = async () => {
        console.log("loadModifiers")
        const result = await invoke('modifiers')
        const modifiersList = Array.isArray(result) ? [...result] : []
        modifiers = modifiersList
    }

    const formatter = new Intl.NumberFormat('en-AU', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    })

    const handleAddClick = () => {
        page.set({view: views.MODIFIERS, mode: modes.NEW})
    }

    const formatDate = (date) => {
        console.log(date)
        switch ($config.display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return transaction.date
            default: return date.toLocaleDateString()
        }
    }

</script>
{#if isListMode($page)}
<div class="account-heading">
    <div class="toolbar toolbar-right"><button class="toolbar-icon" onclick={handleAddClick} title={$_('modifiers.createNew')}><Icon icon="mdi:plus-box-outline"  width="24"/></button></div>
    <div class="form-heading">{$_('modifiers.title')}</div>
    <div class="heading-spinner"><Spinner show={loading}/></div>
</div>
{/if}
{#if isEditMode($page)}
<EditModifier {close} {curModifier} {loadModifiers} {view}/>
{/if}
{#if isViewMode($page)}
<Modifier {close} {edit} {curModifier}/>
{/if}
{#if isListMode($page)}
<div class="controls">
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
    {#if modifiers.length < 1}
    <div class="message">{$_('modifiers.noModifiers')}</div>
    {/if}
        {#each modifiers as m}
            <div class="card" onclick={() => selectModifier(m)} onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); selectModifier(m); } }} tabindex="0" role="button">
                <div class="row">
                    <div class="widget">
                        <div class="card-title">{m.name}</div>
                        <div class="toolbar toolbar-right">
                            <button class="edit-icon" onclick={(e) => { e.stopPropagation(); editModifier(m); }} title={$_('buttons.edit')}><Icon icon="mdi:pencil" /></button>
                        </div>
                    </div>
                </div>
                <hr/>
                <div class="form-row">
                    
                </div>
                <div class="form-row">
                    <div class="small-text"> 
                        {#if m.percentage >= 0}
                        {m.percentage * 100}{$_('modifier.increase')}                        
                        {/if}
                        {#if m.percentage < 0}
                        {m.percentage * -100}{$_('modifier.decrease')}                        
                        {/if}
                    </div>
                    <div class="small-text">
                        
                        {$_('schedule.every')}&nbsp;{m.frequency}&nbsp;{matchPeriod(m.period)?.name}
                        {$_('schedule.starting_from')}&nbsp;{formatDate(new Date(m.start_date))} 
                    </div>
                </div>

            </div>
        {/each}
</div>
{/if}

<style>
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

    .row {
        display: block;
        text-align: left;
    }

    .scroller {
        clear: both;
        margin-top: 0;
    }

    .form-row {
        display: inline-flex;
        float: left;
        width: 100%;
        clear:both;
        margin: 10px 0 0 10px;
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
        margin-left: 5px;
    }

    .card:hover .edit-icon {
        color: #666;
    }

    .edit-icon:hover {
        color: #C0C0C0 !important;
        cursor: pointer;
    }

   .card-title {
        min-width: 500px;
        white-space: nowrap;
        font-weight: bold;
        float: left;
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

    .msg-row {
        display: block;
        float: left;
        clear: both;
        margin: -10px 0px 0px 10px;        
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
