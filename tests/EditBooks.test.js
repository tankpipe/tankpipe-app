import { render, waitFor } from '@testing-library/svelte'
import EditBooks from '../src/EditBooks.svelte'
import {page, views, modes} from '../src/page'


it('is displayed correctly', async () => {
    page.set({view: views.EditBooks, mode: modes.EDIT})
    const mockFetchAccounts = jest.fn(() => Promise.resolve(account_data));
    global.invoke = mockFetchAccounts;

    const {container} = render(EditBooks, {})
    //const _waitForRenderUpdate = await findByTitle('Assets')
    expect(container.outerHTML).toMatchSnapshot();
});

