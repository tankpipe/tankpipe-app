import { render, waitFor } from '@testing-library/svelte'
import Accounts from '../src/accounts/Accounts.svelte'
import {accounts} from '../src/stores/accounts.js'
import {page, views, modes} from '../src/stores/page.js'
import {setHasBooks} from '../src/stores/context.js'
import account_data from './data/account_data.json'
import { init } from 'svelte-i18n'
import '../src/utils/i18n.js'

init({
    fallbackLocale: 'en',
    initialLocale: 'en',
});
accounts.set(account_data)

it('is displayed correctly', async () => {
    setHasBooks(true)
    page.set({view: views.ACCOUNTS, mode: modes.LIST})
    const {container} = await render(Accounts, {loadAccounts: () => {return account_data}, curAccount: account_data[0]})
    expect(container.outerHTML).toMatchSnapshot();
});

