import { render } from '@testing-library/svelte'
import EditMultipleTransactions from '../src/EditMultipleTransactions.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page.js'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { mockIPC } from "@tauri-apps/api/mocks"

accounts.set(account_data)
const loadTransactions = () => {}
const onClose = () => {}

it('is displayed correctly for multiple', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.MULTI_EDIT})
    const select = render(EditMultipleTransactions, {loadTransactions: loadTransactions, curAccount: account_data[0], onClose: onClose, transactions: transaction_data})
    expect(select.container.outerHTML).toMatchSnapshot();
});

