import { writable } from 'svelte/store'

const config = writable({})

const dateFormat = (config) => {
    console.log("*** " + config.display_date_format)
    switch (config.display_date_format) {
        case "Regular": return "dd/MM/yyyy"
        case "US": return "MM/dd/yyyy"
        default: return "yyyy-MM-dd"
    }
}

export {config, dateFormat}