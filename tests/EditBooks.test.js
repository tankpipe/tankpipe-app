import { render, waitFor } from '@testing-library/svelte'
import EditBooks from '../src/EditBooks.svelte'
import {page, views, modes} from '../src/page'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/i18n'

locale.set('en')

it('is displayed correctly', async () => {
    await new Promise(r => setTimeout(r));
    page.set({view: views.EditBooks, mode: modes.EDIT})
    mockIPC((cmd, args) => {
        switch (cmd) {
            case "accounts": return account_data
        }

    })

    const {container} = render(EditBooks, {})
    expect(container.outerHTML).toMatchSnapshot();
});

