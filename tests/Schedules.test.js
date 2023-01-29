import { render, waitFor } from '@testing-library/svelte'
import Schedules from '../src/Schedules.svelte'
import {page, views, modes} from '../src/page'
import schedules_data from './data/schedules_data.json'
import {accounts} from '../src/accounts.js'
import account_data from './data//account_data.json'

accounts.set(account_data)

it('is displayed correctly', async () => {
    page.set({view: views.SCHEDULES, mode: modes.LIST})
    const mockFetchSchedules = jest.fn(() => Promise.resolve(schedules_data));
    global.invoke = mockFetchSchedules;

    const {findByText, container} = render(Schedules)
    await waitFor(() => expect(mockFetchSchedules).toHaveBeenCalledTimes(1))
    const _waitForRenderUpdate = await findByText('Test Schedule 1')
    expect(container.outerHTML).toMatchSnapshot();
});

