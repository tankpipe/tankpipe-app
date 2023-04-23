import {listen, emit} from '@tauri-apps/api/event'
import {open} from '@tauri-apps/api/dialog'
import {page, modes, views} from './page.js'
import {accounts} from './accounts'
import {config} from './config'


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

}

listener()

const openFile = async () => {
    const selected = await open({
        directory: false,
        multiple: false,
        filters: [{name: '*', extensions: ['json']}],
        defaultPath: config.data_dir,
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
    loadConfig()
    emit('file-loaded', "")
    accounts.set(result)
}

const loadFileFailure = (result) => {
    console.log(result)
    errors = new Errors()
    errors.addError("all", "We hit a snag: " + result)
}

const loadConfig = async () => {
    let result = await invoke('config')
    config.set(result)
}