import { render } from '@testing-library/svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page.js'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { config } from '../src/config.js'
import { vi } from 'vitest'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/i18n.js'
import TransactionList from '../src/transactions/TransactionList.svelte'

locale.set('en')
Element.prototype.scrollTo = () => {}
accounts.set(account_data)

// Mock chart to avoid dynamic identifiers.
vi.mock("svelte-apexcharts", () => {
    return {
      chart: vi.fn(() => {})
    };
});

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
    config.set({display_date_format: "ISO"})
    await checkResults(mockFetchTransactions)
});

it('is displayed correctly for Journal view', async () => {
    const mockFetchTransactions = loadTransactions()
    config.set({display_date_format: "Regular"})
    await checkResults(mockFetchTransactions)
});

it('renders Projected transaction with projected class and fields', async () => {
    const mockFetchTransactions = loadTransactions()
    config.set({ display_date_format: 'Regular' })

    // Use the projected transaction from fixture (status: 'Projected')
    const projectedTx = { ...transaction_data[2] }

    // Ensure the current account matches one of the entries so it renders a row
    const { findByText, container, findAllByText } = render(TransactionList, {
        curAccount: account_data[0], // Account 1 present in projectedTx entries
        journalMode: false,
        transactions: [projectedTx]
    })

    // Wait for render by ensuring description appears
    await findAllByText('Office supplies purchase')

    // The projected class should be applied to date/description/amount cells
    const hasProjectedClass = container.querySelectorAll('td.projected').length > 0
    expect(hasProjectedClass).toBe(true)

    // Check formatted fields render (credit and balance both show 75.00)
    const amounts = await findAllByText('75.00')
    expect(amounts.length).toBeGreaterThanOrEqual(1)

    // Snapshot the single projected row rendering
    expect(container.outerHTML).toMatchSnapshot()
});

async function checkResults(mockFetchTransactions) {
    let transactions = [transaction_data[0], transaction_data[1]]
    const { findAllByText, container } = render(TransactionList, { curAccount: account_data[0], journalMode: true, transactions: transactions })
    const _waitForRenderUpdate = await findAllByText('A test transaction')
    // Note: Select should show Account 1 as selected but defaults to Account 3
    expect(container.outerHTML).toMatchSnapshot()
}

function loadTransactions() {
    page.set({ view: views.TRANSACTIONS, mode: modes.LIST })
    let transactions = [transaction_data[0], transaction_data[1]]
    mockIPC((cmd, args) => { return transactions });
}

