import { render } from '@testing-library/svelte'
import Transactions from '../src/Transactions.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { config } from '../src/config'
import { vi, jest } from 'vitest'
import { mockIPC } from "@tauri-apps/api/mocks"

Element.prototype.scrollTo = () => {}
accounts.set(account_data)

const ResizeObserverMock = vi.fn(() => ({
    observe: vi.fn(),
    unobserve: vi.fn(),
    disconnect: vi.fn(),
  }));

  // Stub the global ResizeObserver
  vi.stubGlobal('ResizeObserver', ResizeObserverMock);


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

async function checkResults(mockFetchTransactions) {
    const { findByText, container } = render(Transactions, { curAccount: account_data[0] })
    const _waitForRenderUpdate = await findByText('Description')

    // Note: Select should show Account 1 as selected but defaults to Account 3
    expect(container.outerHTML).toMatchSnapshot()
}

function loadTransactions() {
    page.set({ view: views.TRANSACTIONS, mode: modes.LIST })
    let transactions = [transaction_data[0], transaction_data[1]]
    mockIPC((cmd, args) => { return transactions });
}

