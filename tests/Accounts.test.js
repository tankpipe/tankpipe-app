import { render, waitFor } from '@testing-library/svelte'
import Accounts from '../src/Accounts.svelte'
import {accounts} from '../src/accounts.js'
import {page, views, modes} from '../src/page'
import account_data from './account_data.json'

accounts.set(account_data)

it('is displayed correctly', async () => {
    page.set({view: views.ACCOUNTS, mode: modes.LIST})
    const mockFetchAccounts = jest.fn(() => Promise.resolve(account_data));
    global.invoke = mockFetchAccounts;

    const {container} = render(Accounts, {loadAccounts: mockFetchAccounts, curAccount: account_data[0]})
    //const _waitForRenderUpdate = await findByTitle('Assets')
    expect(container.outerHTML).toMatchSnapshot();
});

