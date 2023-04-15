import { render, waitFor } from '@testing-library/svelte'
import Transactions from '../src/Transactions.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { config } from '../src/config'

accounts.set(account_data)

it('is displayed correctly for Regular date', async () => {
    const mockFetchTransactions = loadTransactions()
    config.set({display_date_format: "Regular"})
    await checkResults(mockFetchTransactions)
});

it('is displayed correctly for US date', async () => {
    const mockFetchTransactions = loadTransactions()
    config.set({display_date_format: "US"})
    await checkResults(mockFetchTransactions)
});

it('is displayed correctly for ISO date', async () => {
    const mockFetchTransactions = loadTransactions()
    config.set({display_date_format: "Locale"})
    await checkResults(mockFetchTransactions)
});

async function checkResults(mockFetchTransactions) {
    const { findByTitle, container } = render(Transactions, { curAccount: account_data[0] })
    await waitFor(() => expect(mockFetchTransactions).toHaveBeenCalledTimes(1))
    const _waitForRenderUpdate = await findByTitle('Add a transaction')

    // Note: Select should show Account 1 as selected but defaults to Account 3
    expect(container.outerHTML).toMatchSnapshot()
}

function loadTransactions() {
    page.set({ view: views.TRANSACTIONS, mode: modes.LIST })
    let transactions = [transaction_data[0], transaction_data[1]]
    const mockFetchTransactions = jest.fn(() => Promise.resolve(transactions))
    global.invoke = mockFetchTransactions
    return mockFetchTransactions
}

