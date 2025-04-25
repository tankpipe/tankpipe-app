import { writable } from 'svelte/store'

const views = {
    BOOKS: "BOOKS",
    ACCOUNTS: "ACCOUNTS",
    TRANSACTIONS: "TRANSACTIONS",
    SCHEDULES: "SCHEDULES",
    SETTINGS: "SETTINGS",
    NET_ASSETS: "NET_ASSETS",
}

const modes = {
    NEW: "NEW",
    EDIT: "EDIT",
    LIST: "LIST",
    LOAD: "LOAD"
}

const page = writable({view: views.NET_ASSETS, mode:modes.LIST, payload:{}})

const isEditMode = (page) => {
    return page.mode === modes.EDIT || page.mode === modes.NEW
}


export {page, views, modes, isEditMode}