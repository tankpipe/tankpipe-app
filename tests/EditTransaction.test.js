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
accounts.set(account_data)
const loadTransactions = () => {}

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


