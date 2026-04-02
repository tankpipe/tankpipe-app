<script>
    import {listen, emit} from '@tauri-apps/api/event'
    import Icon from '@iconify/svelte'
    import { _ } from 'svelte-i18n'

    let errors = $state([])
    const listener = async () => {

        listen('show_errors', (event) => {
            console.log(event)
            errors = []
            errors.push(...event.payload)
        })

        listen('clear_errors', (event) => {
            console.log(event)
            errors = []
        })

    }
    listener()

    const close = () => {
        emit('clear_errors')
    }
</script>

{#if errors.length > 0}
<div class="form">
    <div class="toolbar toolbar-right list-toolbar">
        <button class="toolbar-icon" style="margin-top: -13px;" onclick={close} title={$_('buttons.close')}><Icon icon="mdi:close"  width="24"/></button>
    </div>
    {#each errors as e}
        <div class="error-msg">{e}</div>
    {/each}    
</div>
<hr class="fat-hr" />
{/if}

<style>
    .error-msg {
        color: var(--color-warning);
        font-size: .8em;
        text-align: left;
    }

    hr {
        border-style: none;
        border: 1px solid var(--color-bg-alt);
        margin-left: -20px;
        width: 100vw;
    }

    

</style>

