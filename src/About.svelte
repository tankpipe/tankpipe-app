<script>
    import Icon from '@iconify/svelte'
    import {page, modes, views} from './page.js'

    let about

    $: {
        loadAbout()
    }
    export const loadAbout = async () => {
        console.log("loadAbout ")
        about = await invoke('about')
        console.log(about)
    }

    close = () => {
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
    }

</script>
<div class="toolbar">
    <div class="toolbar-icon" on:click="{close}" title="Close dialog"><Icon icon="mdi:close"  width="24"/></div>
</div>
<div>

<div class="card">
{#if about}
<div class="row">
    <div class="widget">
        <div class="description">Version {about.version}</div>
    </div>
</div>
<hr/>
{/if}
<div class="panel-title">Tankpipe is available under the MIT License</div>
<div class="license">
    <h2>The MIT License (MIT)</h2>

    <p>Copyright © 2023 Tankpipe.com</p>

    <p>Permission is hereby granted, free of charge, to any person obtaining a copy
      of this software and associated documentation files (the “Software”), to deal
      in the Software without restriction, including without limitation the rights
      to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
      copies of the Software, and to permit persons to whom the Software is
      furnished to do so, subject to the following conditions:</p>

    <p>The above copyright notice and this permission notice shall be included in
      all copies or substantial portions of the Software.</p>

    <p>THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
      IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
      FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
      AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
      LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
      OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
      THE SOFTWARE.</p>

</div>
</div>
</div>
<style>
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

</style>