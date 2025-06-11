import { render } from '@testing-library/svelte'
import EditTransaction from '../src/EditTransaction.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/i18n'

locale.set('en')
accounts.set(account_data)
const loadTransactions = () => {}

it('is displayed correctly for NEW', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.NEW})
    const select = render(EditTransaction, {loadTransactions: loadTransactions, curEntry: transaction_data[0]})
    expect(select.container.outerHTML).toMatchSnapshot();
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


