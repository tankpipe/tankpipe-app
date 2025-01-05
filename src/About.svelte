<script>
    import { invoke } from '@tauri-apps/api/core'
    let about

    $: {
        loadAbout()
    }
    export const loadAbout = async () => {
        about = await invoke('about')
    }

</script>
<div class="header">
    <div class="logo">
        <img src="/logo.png" alt="Tankpipe logo" width="64px"/>
    </div>
    <div class="heading">
        <h1>Tankpipe</h1>
        <h5>Cashflow Manager</h5>
    </div>
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
    <div class="panel-title">Got questions?</div>
    <div class="about">
            <div class="description">
                <a target="_blank" href="{about.contact}">Join our Discord</a>
            </div>
    </div>
    <hr/>
    {/if}

    <div class="panel-title">Privacy policy</div>
    <div class="about">
            <div class="description">
                <p>Our Privacy Policy is available at <a target="_blank" href="https://tankpipe.com/app/privacy">tankpipe.com/app/privacy</a>.</p>
            </div>
    </div>
    <hr/>
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
        <hr/>
        <div class="panel-title">Advice disclaimer</div>
        <div class="about">
                <div class="description">
                    <p>Material, content, and functionality available on the Tankpipe app <span class="highlight">does not represent financial, tax, or legal advice</span>.
                        As per the license, the Tankpipe App is provided "AS IS" and no warranty is made on its suitablity for any purpose including financial, tax, or legal matters.
                        You should consult with a qualified professional for advice on anything relating to these matters.
                        See also the Tankpipe <a target="_blank" href="https://tankpipe.com/app/terms">terms.</a></p>
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

</style>