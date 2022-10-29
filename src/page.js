import { writable } from 'svelte/store'

const views = {
    ACCOUNTS: "ACCOUNTS",
    TRANSACTIONS: "TRANSACTIONS",
    SCHEDULES: "SCHEDULES",
    SETTINGS: "SETTINGS"
}

const modes = {
    NEW: "NEW",
    EDIT: "EDIT",
    LIST: "LIST"
}

const page = writable({view: views.ACCOUNTS, mode:modes.LIST, payload:{}})

const isEditMode = (page) => {
    return page.mode === modes.EDIT || page.mode === modes.NEW
}


export {page, views, modes, isEditMode}