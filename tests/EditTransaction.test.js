import { render, waitFor } from '@testing-library/svelte'
import EditTransaction from '../src/EditTransaction.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'

accounts.set(account_data)
const loadTransactions = () => {}

it('is displayed correctly for NEW', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.NEW})
    const select = render(EditTransaction, {close: loadTransactions, curTransaction: transaction_data[0]})
    expect(select.container.outerHTML).toMatchSnapshot();
});


it('is displayed correctly for simple EDIT', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.EDIT})
    const mockFetchTransaction = jest.fn(() => Promise.resolve(transaction_data[0]));
    global.invoke = mockFetchTransaction;

    const {findByText, container} = render(EditTransaction, {close: loadTransactions, curTransaction: transaction_data[0]})
    await waitFor(() => expect(mockFetchTransaction).toHaveBeenCalledTimes(1))
    const _waitForRenderUpdate = await findByText('Description')
    expect(container.outerHTML).toMatchSnapshot();
});

it('is displayed correctly for compound EDIT', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.EDIT})
    const mockFetchTransaction = jest.fn(() => Promise.resolve(transaction_data[1]));
    global.invoke = mockFetchTransaction;

    const {findByText, container} = render(EditTransaction, {close: loadTransactions, curTransaction: transaction_data[1]})
    await waitFor(() => expect(mockFetchTransaction).toHaveBeenCalledTimes(1))
    const _waitForRenderUpdate = await findByText('Description')
    expect(container.outerHTML).toMatchSnapshot();
});

