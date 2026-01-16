import { writable } from 'svelte/store'

const views = {
    BOOKS: "BOOKS",
    ACCOUNTS: "ACCOUNTS",
    TRANSACTIONS: "TRANSACTIONS",
    SCHEDULES: "SCHEDULES",
    SETTINGS: "SETTINGS",
    NET_ASSETS: "NET_ASSETS",
    JOURNAL: "JOURNAL",
}

const modes = {
    NEW: "NEW",
    EDIT: "EDIT",
    MULTI_EDIT: "MULTI_EDIT",
    LIST: "LIST",
    LOAD: "LOAD",
    VIEW: "VIEW"
}

const page = writable({view: views.ACCOUNTS, mode:modes.LIST, payload:{}})

const isEditMode = (page) => {
    return page.mode === modes.EDIT || page.mode === modes.NEW || page.mode === modes.MULTI_EDIT
}

const isSingleEditMode = (page) => {
    return page.mode === modes.EDIT || page.mode === modes.NEW
}

const isMultiEditMode = (page) => {
    return page.mode === modes.MULTI_EDIT
}

const isListMode = (page) => {
    return page.mode === modes.LIST
}

const isViewMode = (page) => {
    return page.mode === modes.VIEW
}

export {page, views, modes, isEditMode, isMultiEditMode, isSingleEditMode, isListMode, isViewMode}