import {listen, emit} from '@tauri-apps/api/event'
import {open} from '@tauri-apps/plugin-dialog'
import {page, modes, views} from './page.js'
import {accounts, updateAccounts} from './accounts.js'
import {updateSettings} from './settings.js'
import {updateContext, setInitialising, isInitialising} from './context.js'
import {config, updateConfig} from './config.js'
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
    showFilePicker(loadFile)
}

const showFilePicker = async (success) => {
    const selected = await open({
        directory: false,
        multiple: false,
        filters: [{name: '*', extensions: ['json']}],
        defaultPath: get(config).data_dir,
    });

    if(selected) {
        console.log(selected)
        success(selected)
    }
}


const loadFile = async (path) => {
    emit('clear_errors')
    if (isInitialising()) {
        await invoke('load_with_path', {path: path}).then(initialiseBooks, loadFileFailure)
    } else {
        await invoke('load_file', {path: path}).then(loadFileSuccess, loadFileFailure)
    }
};

const loadFileSuccess = (result) => {
    console.log(result)
    emit('file-loaded', "")
    updateAccounts(result)
}

const loadFileFailure = (result) => {
    console.log(result)
    emit('show_errors', ["Error loading file", result])
}

const initialiseBooks = async () => {
    console.log("initialiseBooks")
    await loadAccounts()        
    await loadSettings()
    await loadConfig()
    resetMenu()
    setInitialising(false)
    emit('initialised', "")
}

const initialiseFailed = (result) => {
    console.log(result)
    setInitialising(true)
    let errors = []
    errors.push(result)
    emit('show_errors', errors)
}

const loadAccounts = async () => {
    let result = await invoke('accounts')
    updateAccounts(result)
};

const loadSettings = async () => {
    let result = await invoke('settings')
    updateSettings(result)
};

const loadConfig = async () => {
    let result = await invoke('config')
    updateConfig(result)
    console.log(result)
};

const resetMenu = () => {
    updateContext({hasBooks: get(config).recent_files.length > 0})
    if (get(accounts).length < 1) {
        page.set({view: views.ACCOUNTS, mode: modes.NEW})
    }
}

export {showFilePicker, initialiseBooks, initialiseFailed}

