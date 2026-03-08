import { render } from '@testing-library/svelte'
import EditMultipleTransactions from '../src/transactions/EditMultipleTransactions.svelte'
import {accounts} from '../src/stores/accounts.js'
import {page, views, modes} from '../src/stores/page.js'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { locale } from 'svelte-i18n'
import '../src/utils/i18n'
import { config } from '../src/stores/config.js'

locale.set('en')
accounts.set(account_data)
const loadTransactions = () => {}
const onClose = () => {}
config.set({display_date_format: "US"})

it('is displayed correctly for multiple', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.MULTI_EDIT})
    const select = render(EditMultipleTransactions, {loadTransactions: loadTransactions, curAccount: account_data[0], onClose: onClose, transactions: transaction_data})
    expect(select.container.outerHTML).toMatchSnapshot();
});

