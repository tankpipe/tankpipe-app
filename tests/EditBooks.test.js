import { render } from '@testing-library/svelte'
import EditBooks from '../src/EditBooks.svelte'
import {page, views, modes} from '../src/page'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale, init } from 'svelte-i18n'
import { vi } from 'vitest'
import account_data from './data/account_data.json'
import '../src/i18n'

// Mock Tauri event APIs
vi.mock('@tauri-apps/api/event', () => ({
    listen: vi.fn(() => Promise.resolve()),
    emit: vi.fn(() => Promise.resolve())
}))

// Mock Tauri dialog plugin
vi.mock('@tauri-apps/plugin-dialog', () => ({
    open: vi.fn(() => Promise.resolve(null))
}))

init({
    fallbackLocale: 'en',
    initialLocale: 'en',
});

locale.set('en')

it('is displayed correctly', async () => {
    await new Promise(r => setTimeout(r));
    page.set({view: views.BOOKS, mode: modes.EDIT})
    mockIPC((cmd, args) => {
        switch (cmd) {
            case "accounts": return account_data
        }

    })

    const {container} = await render(EditBooks, {})
    expect(container.outerHTML).toMatchSnapshot();
});

