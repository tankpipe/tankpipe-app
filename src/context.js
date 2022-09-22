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

class Context {
    constructor() {
        this.view = views.ACCOUNTS
        this.mode = modes.LIST
        this.settings = null
    }

    setView(view, mode) {
        this.view = view

        if (mode) {
            this.mode = mode
        }
    }
}

export {Context, views, modes}