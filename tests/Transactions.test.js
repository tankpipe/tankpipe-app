import { render } from '@testing-library/svelte'
import Transactions from '../src/transactions/Transactions.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { config } from '../src/config'
import { vi } from 'vitest'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/i18n'

locale.set('en')
Element.prototype.scrollTo = () => {}
accounts.set(account_data)

// Mock chart to avoid dynamic identifiers.
vi.mock("svelte-apexcharts", () => {
    return {
      chart: vi.fn(() => {})
    };
});


it('is displayed correctly for Transactions mode', async () => {
    const mockFetchTransactions = loadTransactions()
    config.set({display_date_format: "Regular"})
    await checkResults(mockFetchTransactions, false)
});


it('is displayed correctly for Journal mode', async () => {
    const mockFetchTransactions = loadTransactions()
    config.set({display_date_format: "Regular"})
    await checkResults(mockFetchTransactions, true)
});

async function checkResults(mockFetchTransactions, jornalMode) {
    const { findAllByText, container } = render(Transactions, { curAccount: account_data[0], journalMode: jornalMode })
    const _waitForRenderUpdate = await findAllByText('A test transaction')
    // Note: Select should show Account 1 as selected but defaults to Account 3
    expect(container.outerHTML).toMatchSnapshot()
}

function loadTransactions() {
    page.set({ view: views.TRANSACTIONS, mode: modes.LIST })
    let transactions = [transaction_data[0], transaction_data[1]]
    mockIPC((cmd, args) => { return transactions });
}

