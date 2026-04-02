import { writable } from 'svelte/store'

const settings = writable({})

const updateSettings = (newSettings) => {
    settings.set(newSettings)
}

export {settings, updateSettings}