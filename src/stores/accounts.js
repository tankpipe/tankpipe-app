import { writable } from 'svelte/store'

const NORMAL_BALANCE = {
    Asset: 'Debit',
    Liability: 'Credit',
    Equity: 'Credit',
    Income: 'Credit',
    Expense: 'Debit'
}

const accounts = writable({})

const updateAccounts = (newAccounts) => {
    accounts.set(newAccounts)
}

const normalBalance = (accountType) => {
    return NORMAL_BALANCE[accountType]
}

export {accounts as accounts, updateAccounts, normalBalance}