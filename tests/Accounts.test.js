import { render, waitFor } from '@testing-library/svelte'
import Accounts from '../src/Accounts.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './data/account_data.json'
import { locale } from 'svelte-i18n'
import '../src/i18n'

locale.set('en')
accounts.set(account_data)

it('is displayed correctly', async () => {
    page.set({view: views.ACCOUNTS, mode: modes.LIST})
    const {container} = render(Accounts, {loadAccounts: () => {return account_data}, curAccount: account_data[0]})
    expect(container.outerHTML).toMatchSnapshot();
});

