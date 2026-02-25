import { render, fireEvent, waitFor, vi } from 'vitest'
import '@testing-library/jest-dom'
import { render as svelteRender } from '@testing-library/svelte'
import EditAccount from '../src/EditAccount.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import {setHasBooks} from '../src/context'
import account_data from './data/account_data.json'
import { locale } from 'svelte-i18n'
import '../src/i18n'

// Mock the invoke function using vitest
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn((cmd, args) => {
        if (cmd === 'transactions') {
            return Promise.resolve([])
        }
        return Promise.resolve()
    })
}))

locale.set('en')
accounts.set(account_data)

const loadAccounts = () => { return account_data }

it('renders NEW mode correctly', async () => {
    setHasBooks(true)
    page.set({view: views.ACCOUNTS, mode: modes.NEW})
    const {container, getByText} = svelteRender(EditAccount, {
        loadAccounts: loadAccounts,
        close: () => {}
    })
    
    expect(getByText('New Account')).toBeInTheDocument()
    expect(getByText('Add')).toBeInTheDocument()
    expect(container.outerHTML).toMatchSnapshot()
});

it('renders EDIT mode correctly', async () => {
    setHasBooks(true)
    page.set({view: views.ACCOUNTS, mode: modes.EDIT})
    
    const {container, getByText} = svelteRender(EditAccount, {
        loadAccounts: loadAccounts, 
        curAccount: account_data[0],
        initialize: true,
        close: () => {}
    })
    
    expect(getByText('Edit Account')).toBeInTheDocument()
    expect(getByText('Update')).toBeInTheDocument()
    expect(container.outerHTML).toMatchSnapshot()
});

it('displays reconciliation info when account has reconciliation', async () => {
    setHasBooks(true)
    page.set({view: views.ACCOUNTS, mode: modes.EDIT})
    
    const accountWithReconciliation = {
        ...account_data[0],
        reconciliation_info: {
            date: "2022-06-04",
            balance: "1000.00",
            transaction_id: "test-transaction-id"
        }
    }
    
    const {container, getByText} = svelteRender(EditAccount, {
        loadAccounts: loadAccounts, 
        curAccount: accountWithReconciliation,
        initialize: true,
        close: () => {}
    })
    
    expect(getByText('Reconciliation')).toBeInTheDocument()
    expect(getByText('Reconciled to')).toBeInTheDocument()
    expect(getByText('Reconciled balance')).toBeInTheDocument()
    // Check for either date format
    expect(() => getByText('04/06/2022') || getByText('6/4/2022')).not.toThrow()
    expect(getByText('1,000.00')).toBeInTheDocument()
});

it('displays rollback section when account has reconciliation', async () => {
    setHasBooks(true)
    page.set({view: views.ACCOUNTS, mode: modes.EDIT})
    
    const accountWithReconciliation = {
        ...account_data[0],
        reconciliation_info: {
            date: "2022-06-04",
            balance: "1000.00",
            transaction_id: "test-transaction-id"
        }
    }
    
    const {container, getByText} = svelteRender(EditAccount, {
        loadAccounts: loadAccounts, 
        curAccount: accountWithReconciliation,
        initialize: true,
        close: () => {}
    })
    
    expect(getByText('Rollback reconciliation to')).toBeInTheDocument()
    expect(getByText('Rollback')).toBeInTheDocument()
    expect(getByText('Rollback')).toBeDisabled()
});

it('does not display rollback section when account has no reconciliation', async () => {
    setHasBooks(true)
    page.set({view: views.ACCOUNTS, mode: modes.EDIT})
    
    const {container, queryByText} = svelteRender(EditAccount, {
        loadAccounts: loadAccounts, 
        curAccount: account_data[0], // Account without reconciliation_info
        initialize: true,
        close: () => {}
    })
    
    expect(queryByText('Rollback reconciliation to')).not.toBeInTheDocument()
    expect(queryByText('Rollback')).not.toBeInTheDocument()
});
