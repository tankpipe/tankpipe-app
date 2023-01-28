import { render, waitFor } from '@testing-library/svelte'
import Transactions from '../src/Transactions.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './account_data.json'
import transaction_data from './transaction_data.json'

accounts.set(account_data)

it('is displayed correctly', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.LIST})
    let entries = [transaction_data[0].entries[0], transaction_data[1].entries[0]]
    const mockFetchTransactions = jest.fn(() => Promise.resolve(entries));
    global.invoke = mockFetchTransactions;

    const {findByTitle, container} = render(Transactions, {curAccount: account_data[0]})
    await waitFor(() => expect(mockFetchTransactions).toHaveBeenCalledTimes(1))
    const _waitForRenderUpdate = await findByTitle('Add a transaction')

    // Note: Select should show Account 1 as selected but defaults to Account 3
    expect(container.outerHTML).toMatchSnapshot();
});

