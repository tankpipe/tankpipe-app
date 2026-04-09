import { render, fireEvent, waitFor } from '@testing-library/svelte'
import Schedules from '../src/schedules/Schedules.svelte'
import {page, views, modes} from '../src/stores/page.js'
import schedules_data from './data/schedules_data.json'
import {accounts} from '../src/stores/accounts.js'
import account_data from './data//account_data.json'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/utils/i18n'

locale.set('en')

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

it('reuses refreshed schedule data when reopening edit after save', async () => {
    page.set({view: views.SCHEDULES, mode: modes.LIST})
    let currentPage
    const unsubscribe = page.subscribe((value) => {
        currentPage = value
    })

    let schedulesState = structuredClone(schedules_data)
    const updatedName = 'Updated Schedule Name'

    mockIPC((cmd, args) => {
        switch (cmd) {
            case 'schedules':
                return schedulesState
            case 'end_date':
                return { date: new Date('2026-12-31') }
            case 'modifiers':
                return []
            case 'schedule_transactions':
                return []
            case 'update_schedule': {
                const incoming = args.schedule
                schedulesState = schedulesState.map((schedule) =>
                    schedule.id === incoming.id
                        ? { ...schedule, ...incoming, entries: incoming.entries ?? schedule.entries }
                        : schedule
                )
                return incoming
            }
            case 'get_schedule':
                return schedulesState.find((s) => s.id === args.scheduleId) ?? null
            default:
                return []
        }
    })

    const { container, findByText } = render(Schedules)
    await findByText('Test Schedule 1')

    // Open schedule in view mode.
    const firstCard = container.querySelector('.card')
    await fireEvent.click(firstCard)
    await waitFor(() => expect(currentPage.mode).toBe(modes.VIEW))

    // From view, open edit mode using the first top-right toolbar icon.
    const viewEditButton = container.querySelector('.form .toolbar.toolbar-right .toolbar-icon')
    await fireEvent.click(viewEditButton)
    await waitFor(() => expect(currentPage.mode).toBe(modes.EDIT))

    // Update the schedule name and save.
    const nameInput = container.querySelector('.description-input')
    await fireEvent.input(nameInput, { target: { value: updatedName } })
    const saveButton = container.querySelector('.buttons .og-button:last-child')
    await fireEvent.click(saveButton)

    await waitFor(() => {
        const scheduleInList = schedulesState.find((s) => s.id === schedules_data[0].id)
        expect(scheduleInList.name).toBe(updatedName)
    })

    // Go back to view mode, then reopen edit mode.
    const viewButton = container.querySelector('.list-toolbar .toolbar-icon')
    await fireEvent.click(viewButton)
    await waitFor(() => expect(currentPage.mode).toBe(modes.VIEW))
    await findByText(updatedName)

    const editAgainButton = container.querySelector('.form .toolbar.toolbar-right .toolbar-icon')
    await fireEvent.click(editAgainButton)
    await waitFor(() => expect(currentPage.mode).toBe(modes.EDIT))

    await waitFor(() => {
        const reopenedNameInput = container.querySelector('.description-input')
        expect(reopenedNameInput.value).toBe(updatedName)
    })

    unsubscribe()
})
