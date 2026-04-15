import { render } from '@testing-library/svelte'
import {accounts} from '../src/stores/accounts.js'
import {page, views, modes} from '../src/stores/page.js'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { config } from '../src/stores/config.js'
import { vi } from 'vitest'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/utils/i18n.js'
import TransactionList from '../src/transactions/TransactionList.svelte'
import { ReconciliationMode } from '../src/transactions/reconciliation.js'

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
        transactions: [projectedTx],
        setTopScroll: vi.fn()
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

it('renders correctly in ReconciliationMode.GUIDED with all reconciliation content variations', async () => {
    // Create test transactions with different reconciliation states using existing data
    const reconciledTransaction = {
        ...transaction_data[0],
        entries: transaction_data[0].entries.map(entry => ({
            ...entry,
            reconciled_status: 'Reconciled'
        }))
    };

    const outstandingTransaction = {
        ...transaction_data[1],
        entries: transaction_data[1].entries.map(entry => ({
            ...entry,
            reconciled_status: 'Outstanding'
        }))
    };

    const unreconciledTransaction = {
        ...transaction_data[2],
        entries: transaction_data[2].entries.map(entry => ({
            ...entry,
            reconciled_status: null
        }))
    };

    // Create reconciliation results - the matched_transaction_id should match an entry's transaction_id
    const reconciliationResults = [
        {
            Original:{
                transaction: reconciledTransaction,
                matched_reconciliation_id: null,
                status: 'Unmatched',
                balance: 100
            }
        },
        {
            Original:{
                transaction: outstandingTransaction,
                matched_reconciliation_id: null,
                status: 'Unmatched',
                balance: 100
            }
        },
        {
            Original:{
                transaction: unreconciledTransaction,
                matched_reconciliation_id: unreconciledTransaction.id,
                status: 'Matched',
                balance: 100
            }
        },
        {
            Reconciliation:{
                transaction: {
                    ...unreconciledTransaction,
                    // Ensure reconciliation result transaction has an entry that matches curAccount
                    entries: [
                        {
                            ...unreconciledTransaction.entries[0],
                            account_id: account_data[0].id, // Match current account ID
                            account: account_data[0]
                        },
                        ...unreconciledTransaction.entries.slice(1)
                    ]
                },
                matched_transaction_id: transaction_data[2].id, // Use the actual transaction ID
                status: 'Matched',
                balance: 100
            }
        }
    ];

    const { container, findAllByText } = render(TransactionList, {
        curAccount: account_data[0],
        journalMode: false,
        transactions: [reconciledTransaction, outstandingTransaction, unreconciledTransaction],
        reconciliationResults: reconciliationResults,
        reconciliationMode: ReconciliationMode.GUIDED,
        topScroll: null,
        setTopScroll: vi.fn(),
        descriptionFilter: ""
    });

    // Wait for render
    await findAllByText('A test transaction');

    // Debug: Check what's actually rendered
    console.log('Container HTML:', container.innerHTML);

    // Check for reconciled content (check icon)
    const reconciledIcons = container.querySelectorAll('.reconciled-cell .mdi-check');
    console.log('Reconciled icons found:', reconciledIcons.length);

    // Check for outstanding content (circle icon)
    const outstandingIcons = container.querySelectorAll('.reconciled-cell .mdi-circle-small');
    console.log('Outstanding icons found:', outstandingIcons.length);

    // Check for reconcilable content (check button)
    const reconcilableButtons = container.querySelectorAll('.reconcilable-marker');
    console.log('Reconcilable buttons found:', reconcilableButtons.length);

    // Check for merge markers
    const mergeMarkers = container.querySelectorAll('.merge-marker');
    console.log('Merge markers found:', mergeMarkers.length);

    // Basic assertions - should at least have some reconciliation cells
    const reconciliationCells = container.querySelectorAll('.reconciled-cell');
    expect(reconciliationCells.length).toBeGreaterThan(0);

    // Verify component structure with snapshot
    expect(container.outerHTML).toMatchSnapshot();
});

async function checkResults(mockFetchTransactions) {
    let transactions = [transaction_data[0], transaction_data[1]]
    const { findAllByText, container } = render(TransactionList, { curAccount: account_data[0], journalMode: true, transactions: transactions, setTopScroll: vi.fn() })
    const _waitForRenderUpdate = await findAllByText('A test transaction')
    // Note: Select should show Account 1 as selected but defaults to Account 3
    expect(container.outerHTML).toMatchSnapshot()
}

function loadTransactions() {
    page.set({ view: views.TRANSACTIONS, mode: modes.LIST })
    let transactions = [transaction_data[0], transaction_data[1]]
    mockIPC((cmd, args) => { return transactions });
}

