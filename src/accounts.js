import { writable } from 'svelte/store'

const accounts = writable({})

const updateAccounts = (newAccounts) => {
    accounts.set(newAccounts)
}

export {accounts as accounts, updateAccounts}