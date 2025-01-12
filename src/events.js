import {listen, emit} from '@tauri-apps/api/event'
import {open} from '@tauri-apps/plugin-dialog'
import {page, modes, views} from './page.js'
import {accounts} from './accounts'
import {config} from './config'
import { get } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core';



const listener = async () => {
    await listen('file-open', (event) => {
        console.log(event)
        page.set({view: views.ACCOUNTS, mode: modes.LIST})
        openFile()
    })

    listen('file-new', (event) => {
        console.log(event)
        page.set({view: views.BOOKS, mode: modes.NEW})
    })

    listen('preferences', (event) => {
        console.log(event)
        page.set({view: views.SETTINGS, mode: modes.EDIT})
    })
}

listener()

const openFile = async () => {
    const selected = await open({
        directory: false,
        multiple: false,
        filters: [{name: '*', extensions: ['json']}],
        defaultPath: get(config).data_dir,
    });

    if(selected) {
        console.log(selected)
        loadFile(selected)
    }
}

const loadFile = async (path) => {
    await invoke('load_file', {path: path}).then(loadFileSuccess, loadFileFailure)
};

const loadFileSuccess = (result) => {
    console.log(result)
    emit('file-loaded', "")
    accounts.set(result)
}

const loadFileFailure = (result) => {
    console.log(result)
    let errors = new Errors()
    errors.addError("all", "We hit a snag: " + result)
}

