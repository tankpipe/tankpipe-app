<script>
    import Icon from '@iconify/svelte';
    import { invoke } from '@tauri-apps/api/core'
    import { onMount } from 'svelte';
    import { _ } from 'svelte-i18n'
    let about

    onMount(() => {
        loadAbout()
    })

    export const loadAbout = async () => {
        about = await invoke('about')
    }

</script>
<div class="header">
    <div class="logo">
        <img src="/logo.png" alt={$_('about.logo_alt')} width="64px"/>
    </div>
    <div class="heading">
        <h1>{$_('about.title')}</h1>
        <h5>{$_('about.subtitle')}</h5>
    </div>
</div>
<div>

<div class="card">
    {#if about}
    <div class="row">
        <div class="widget">
            <div class="description">{$_('about.version', { values:{ version: about.version } })}</div>
        </div>
    </div>
    <hr/>
    <div class="panel-title">{$_('about.questions_title')}</div>
    <div class="about">
            <div class="description">
                <div class="link"><a target="_blank" href="https://github.com/tankpipe/tankpipe-app/issues"><Icon icon="mdi:github-circle"  width="24"/><div class="link-text"> {$_('about.github_link')}</div></a><br/></div>
                <div class="link"><a target="_blank" href="{about.contact}"><Icon icon="mdi:discord"  width="24"/><div class="link-text"> {$_('about.discord_link')}</div></a></div>
            </div>
    </div>
    <hr/>
    {/if}

    <div class="panel-title">{$_('about.privacy_title')}</div>
    <div class="about">
            <div class="description">
                <p>{@html $_('about.privacy_text')}</p>
            </div>
    </div>
    <hr/>
    <div class="panel-title">{$_('about.license_title')}</div>
        <div class="license">
            <h2>{$_('about.mit_license')}</h2>

            <p>{$_('about.copyright')}</p>

            <p>{$_('about.license_permission')}</p>

            <p>{$_('about.license_notice')}</p>

            <p>{$_('about.license_warranty')}</p>

        </div>
        <hr/>
        <div class="panel-title">{$_('about.disclaimer_title')}</div>
        <div class="about">
                <div class="description">
                    <p>{@html $_('about.disclaimer_text')}</p>
                </div>
        </div>
    </div>
</div>
<style>
    .logo {
        float: left;
        margin-left: -7px;
    }

    .heading {
        margin-left: 57px;
        font-size: 1em;
    }

    .header {
        float: left;
        margin-left: 10px;
        font-size: 1em !important;
    }

    .header h1 {
        margin-bottom: 0;
        margin-top: 0px;
        padding-top: 4px;
    }

    .header h5 {
        margin-left: 5px;
        margin-top: -2px;
        margin-bottom: 0;
    }
    hr {
        border: 1px solid #444;
        margin: 0 -5px;
    }
    .row {
        display: block;
        text-align: left;
    }
    .card {
        float: left;
        clear: both;
        margin: 10px;
        background-color: #524e4e;
        padding: 5px;
    }

    .widget {
        display: inline-block;
        text-align: left !important;
        margin: 10px 10px;
        color: #F0F0F0;
        vertical-align: top;
    }

    .panel-title {
        margin: 10px 10px;
        color: #A0A0A0;
    }
    .license {
        min-width: 360px;
        max-width: 700px;
        color: #C0C0C0;
        font-family: Inconsolata,"DejaVu Sans Mono","Bitstream Vera Sans Mono",monospace;
        font-size: .9em;
        line-height: 1.2em;
        text-align: left;
        margin: 10px 10px;
    }
    .license h2 {
        color: #F0F0F0;
    }

    a {
        color: #C0C0C0;
        font-family: Inconsolata,"DejaVu Sans Mono","Bitstream Vera Sans Mono",monospace;
        font-size: .9em;
        line-height: 1.2em;
    }

    .about {
        min-width: 360px;
        max-width: 700px;
        color: #C0C0C0;
        font-size: .9em;
        text-align: left;
        margin: 10px 10px;
    }

    .description .highlight {
        color: #f0f0f0;
    }

    .link-text {
        vertical-align: super;
        display: initial;
        padding-left: 5px;
    }

    .link {
        margin-bottom: 5px;
        margin-left: 3px;
    }

</style>