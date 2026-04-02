import { render } from '@testing-library/svelte'
import EditTransaction from '../src/transactions/EditTransaction.svelte'
import {accounts} from '../src/stores/accounts.js'
import {page, views, modes} from '../src/stores/page.js'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/utils/i18n'

locale.set('en')
const loadTransactions = () => {}

beforeEach(() => {
    accounts.set(account_data)
})

it('is displayed correctly for NEW', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.NEW})
    const select = render(EditTransaction, {loadTransactions: loadTransactions})
    expect(select.container.outerHTML).toMatchSnapshot();
});

it('sets recorded to false when all entries are in future', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.NEW})
    const { container } = render(EditTransaction, {loadTransactions: loadTransactions})
    
    // Set date to future by finding DateInput component
    const futureDate = new Date()
    futureDate.setDate(futureDate.getDate() + 7)
    const dateInput = container.querySelector('.date-input input')
    dateInput.value = futureDate.toISOString().split('T')[0]
    
    // Trigger change
    dateInput.dispatchEvent(new Event('input', { bubbles: true }))
    
    // Wait for reactivity to update
    await new Promise(resolve => setTimeout(resolve, 10))
    
    // Check that recorded checkbox is unchecked and disabled
    const recordedCheckbox = container.querySelector('input#recorded')
    expect(recordedCheckbox.checked).toBe(false)
    expect(recordedCheckbox.disabled).toBe(true)
});

it('allows recorded toggle when entries are not all in future', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.NEW})
    const { container } = render(EditTransaction, {loadTransactions: loadTransactions})
    
    // Wait for component to initialize
    await new Promise(resolve => setTimeout(resolve, 10))
    
    // Check that recorded checkbox is enabled (should be enabled by default for NEW mode with today's date)
    const recordedCheckbox = container.querySelector('input#recorded')
    expect(recordedCheckbox.disabled).toBe(false)
    
    // Test toggling
    recordedCheckbox.click()
    await new Promise(resolve => setTimeout(resolve, 10))
    expect(recordedCheckbox.checked).toBe(false)
    
    recordedCheckbox.click()
    await new Promise(resolve => setTimeout(resolve, 10))
    expect(recordedCheckbox.checked).toBe(true)
});

it('disables already-selected accounts in other entry selects (simple and compound)', async () => {
    const localAccounts = [
        { id: 'acc1', name: 'Account 1', account_type: 'Asset', balance: 0, starting_balance: 0 },
        { id: 'acc2', name: 'Account 2', account_type: 'Asset', balance: 0, starting_balance: 0 },
    ]
    accounts.set(localAccounts)

    page.set({view: views.TRANSACTIONS, mode: modes.NEW})
    const { container } = render(EditTransaction, {loadTransactions: loadTransactions})
    await new Promise(resolve => setTimeout(resolve, 10))

    const getOptionByText = (selectEl, text) =>
        Array.from(selectEl.querySelectorAll('option')).find(o => o.textContent === text)

    // Simple mode: two account selects
    let selects = Array.from(container.querySelectorAll('select'))
    expect(selects.length).toBeGreaterThanOrEqual(2)
    let debitSelect = selects[0]
    let creditSelect = selects[1]

    // Choose "Account 1" for debit entry
    const debitAccount1 = getOptionByText(debitSelect, 'Account 1')
    debitSelect.selectedIndex = Array.from(debitSelect.options).indexOf(debitAccount1)
    debitSelect.dispatchEvent(new Event('change', { bubbles: true }))
    await new Promise(resolve => setTimeout(resolve, 10))

    // Re-query selects because they may be keyed/remounted
    selects = Array.from(container.querySelectorAll('select'))
    debitSelect = selects[0]
    creditSelect = selects[1]

    expect(getOptionByText(debitSelect, 'Account 1').disabled).toBe(false)
    expect(getOptionByText(creditSelect, 'Account 1').disabled).toBe(true)

    // Switch to compound mode and ensure the other row's select disables Account 1
    const compoundCheckbox = container.querySelector('input#compound')
    compoundCheckbox.click()
    await new Promise(resolve => setTimeout(resolve, 10))

    selects = Array.from(container.querySelectorAll('select'))
    expect(selects.length).toBeGreaterThanOrEqual(2)
    const row0Select = selects[0]
    const row1Select = selects[1]
    expect(getOptionByText(row0Select, 'Account 1').disabled).toBe(false)
    expect(getOptionByText(row1Select, 'Account 1').disabled).toBe(true)

    // Change row0 to Account 2; row1 should now disable Account 2 and re-enable Account 1
    const row0Account2 = getOptionByText(row0Select, 'Account 2')
    row0Select.selectedIndex = Array.from(row0Select.options).indexOf(row0Account2)
    row0Select.dispatchEvent(new Event('change', { bubbles: true }))
    await new Promise(resolve => setTimeout(resolve, 10))

    // Re-query selects because they may be keyed/remounted
    selects = Array.from(container.querySelectorAll('select'))
    const row1AfterChange = selects[1]
    expect(getOptionByText(row1AfterChange, 'Account 2').disabled).toBe(true)
    expect(getOptionByText(row1AfterChange, 'Account 1').disabled).toBe(false)

    // Restore shared accounts store for subsequent tests
    accounts.set(account_data)
})


it('is displayed correctly for simple EDIT', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.EDIT})
    mockIPC((cmd, args) => { return transaction_data[0] })

    const {findByText, container} = render(EditTransaction, {loadTransactions: loadTransactions, curEntry: transaction_data[0]})
    const _waitForRenderUpdate = await findByText('Description')
    expect(container.outerHTML).toMatchSnapshot();
});

it('is displayed correctly for compound EDIT', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.EDIT})
    mockIPC((cmd, args) => { return transaction_data[1] })

    const {findByText, container} = render(EditTransaction, {loadTransactions: loadTransactions, curEntry: transaction_data[1]})
    const _waitForRenderUpdate = await findByText('Description')
    expect(container.outerHTML).toMatchSnapshot();
});

it('is displayed correctly for MULTI_EDIT', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.EDIT})
    mockIPC((cmd, args) => { return transaction_data[0] })

    const {findByText, container} =
            render(EditTransaction, {loadTransactions: loadTransactions, curEntry: transaction_data[0], transactions: transaction_data})
    const _waitForRenderUpdate = await findByText('Description')
    expect(container.outerHTML).toMatchSnapshot();
});
