import {listen, emit} from '@tauri-apps/api/event'
import {open} from '@tauri-apps/plugin-dialog'
import {page, modes, views} from './stores/page.js'
import {accounts, updateAccounts} from './stores/accounts.js'
import {updateSettings} from './stores/settings.js'
import {updateContext, setInitialising, isInitialising, setHasBooks, hasBooks} from './stores/context.js'
import {config, updateConfig} from './stores/config.js'
import { get } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'


const listener = async () => {
    listen('file-open', (event) => {
        openFile()
    })

    listen('file-new', (event) => {
        page.set({view: views.BOOKS, mode: modes.NEW})
    })

    listen('preferences', (event) => {
        page.set({view: views.SETTINGS, mode: modes.EDIT})
    })

    listen('file-backups', (event) => {
        page.set({view: views.BACKUPS, mode: modes.LIST})
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
    console.log("loadFile", path, (isInitialising() || !hasBooks()))
    emit('clear_errors')
    if (isInitialising() || !hasBooks()) {
        await invoke('load_with_path', {path: path}).then(initialiseBooks, loadFileFailure)
    } else {
        await invoke('load_file', {path: path}).then(loadFileSuccess, loadFileFailure)
    }
};

const loadFileSuccess = async (result) => {
    emit('file-loaded', "")
    updateAccounts(result)
    await loadSettings()
    page.set({view: views.ACCOUNTS, mode: modes.LIST})
    await loadConfig()
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
    setHasBooks(true)
    setInitialising(false)
    page.set({view: views.ACCOUNTS, mode: modes.LIST})
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

const resetMenu = async () => {
    const hasBooksData = get(config).current_books_id || get(config).current_file;
    updateContext({hasBooks: hasBooksData})
    if (get(accounts).length < 1) {
        page.set({view: views.ACCOUNTS, mode: modes.NEW})
    }
}

export {showFilePicker, initialiseBooks, initialiseFailed, loadConfig, loadSettings}
