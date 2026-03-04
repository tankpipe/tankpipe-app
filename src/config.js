import { writable, get } from 'svelte/store'

const config = writable({})

const dateFormat = (config) => {
    switch (config.display_date_format) {
        case "Regular": return "dd/MM/yyyy"
        case "US": return "MM/dd/yyyy"
        default: return "yyyy-MM-dd"
    }
}

const amountFormatter = new Intl.NumberFormat('en-AU', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
})

const formatAmount = (amount) => {
    return amountFormatter.format(amount)
}

const formatDate = (inDate) => {
        const date = new Date(inDate)

        switch (get(config).display_date_format) {
            case "Regular": return date.toLocaleDateString("en-GB")
            case "US": return date.toLocaleDateString("en-US")
            case "ISO": return transaction.date
            default: return date.toLocaleDateString()
        }
    }


const updateConfig = (update) => {
    config.update(value => (Object.assign(value, update)))
}

export {config, dateFormat, formatDate, formatAmount, updateConfig}