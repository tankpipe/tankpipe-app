import { render } from '@testing-library/svelte'
import Schedules from '../src/Schedules.svelte'
import {page, views, modes} from '../src/page'
import schedules_data from './data/schedules_data.json'
import {accounts} from '../src/accounts.js'
import account_data from './data//account_data.json'
import { mockIPC } from "@tauri-apps/api/mocks"

accounts.set(account_data)

it('is displayed correctly', async () => {
    page.set({view: views.SCHEDULES, mode: modes.LIST})
    mockIPC((cmd, args) => {
        switch (cmd) {
            case "schedules": return schedules_data
        }

    });

    const {findByText, container} = render(Schedules)
    const _waitForRenderUpdate = await findByText('Test Schedule 1')
    expect(container.outerHTML).toMatchSnapshot();
});

