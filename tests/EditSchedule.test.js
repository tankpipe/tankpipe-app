import { render, cleanup, waitFor } from '@testing-library/svelte'
import EditSchedule from '../src/schedules/EditSchedule.svelte'
import { accounts } from '../src/stores/accounts.js'
import { page, views, modes } from '../src/stores/page.js'
import account_data from './data/account_data.json'
import { mockIPC } from "@tauri-apps/api/mocks"
import { locale } from 'svelte-i18n'
import '../src/utils/i18n'

locale.set('en')
accounts.set(account_data)

afterEach(() => cleanup())

it('disables already-selected accounts in other schedule entry selects', async () => {
  page.set({ view: views.SCHEDULES, mode: modes.NEW })
  mockIPC((cmd) => {
    switch (cmd) {
      case 'modifiers':
        return []
      default:
        return []
    }
  })

  const close = () => {}
  const loadSchedules = () => {}
  const view = () => {}

  const { container } = render(EditSchedule, {
    close,
    curSchedule: {},
    loadSchedules,
    view,
  })

  const addButton = container.querySelector('.gg-add-r')
  addButton.click()
  await new Promise((r) => setTimeout(r, 10))

  const getOptionByText = (selectEl, text) =>
    Array.from(selectEl.querySelectorAll('option')).find((o) => o.textContent === text)

  let selects = Array.from(container.querySelectorAll('select'))
  // First select is an entry account select; there may also be other selects (modifier/period)
  // Limit to the first two account selects in the entries table.
  const entrySelects = selects.filter((s) => s.closest('.entries'))
  expect(entrySelects).toHaveLength(2)

  const row0Select = entrySelects[0]
  const row1Select = entrySelects[1]

  const row0Account1 = getOptionByText(row0Select, 'Account 1')
  row0Select.selectedIndex = Array.from(row0Select.options).indexOf(row0Account1)
  row0Select.dispatchEvent(new Event('change', { bubbles: true }))
  await new Promise((r) => setTimeout(r, 10))

  // Re-query (keyed remount possible)
  selects = Array.from(container.querySelectorAll('select'))
  const entrySelectsAfter = selects.filter((s) => s.closest('.entries'))
  const row1After = entrySelectsAfter[1]
  expect(getOptionByText(row1After, 'Account 1').disabled).toBe(true)

  // Change row0 to Account 2; row1 should now disable Account 2 and re-enable Account 1
  const row0After = entrySelectsAfter[0]
  const row0Account2 = getOptionByText(row0After, 'Account 2')
  row0After.selectedIndex = Array.from(row0After.options).indexOf(row0Account2)
  row0After.dispatchEvent(new Event('change', { bubbles: true }))
  await new Promise((r) => setTimeout(r, 10))

  selects = Array.from(container.querySelectorAll('select'))
  const entrySelectsFinal = selects.filter((s) => s.closest('.entries'))
  const row1Final = entrySelectsFinal[1]
  expect(getOptionByText(row1Final, 'Account 2').disabled).toBe(true)
  expect(getOptionByText(row1Final, 'Account 1').disabled).toBe(false)
})

it('shows end date selected in edit mode when schedule has end_date', async () => {
  page.set({ view: views.SCHEDULES, mode: modes.EDIT })
  mockIPC((cmd) => {
    switch (cmd) {
      case 'modifiers':
        return []
      case 'schedule_transactions':
        return []
      default:
        return []
    }
  })

  const close = () => {}
  const loadSchedules = () => {}
  const view = () => {}
  const curSchedule = {
    id: 'sched-1',
    name: 'Pay',
    description: 'Monthly pay',
    amount: '1000',
    dr_account_id: account_data[0].id,
    cr_account_id: account_data[1].id,
    period: 'Months',
    frequency: 1,
    start_date: '2026-01-31',
    end_date: '2026-12-31',
    last_date: 'null',
    modifier_configs: [],
    schedule_modifiers: [],
    entries: [
      {
        id: 'e1',
        schedule_id: 'sched-1',
        account_id: account_data[0].id,
        description: 'Entry 1',
        amount: '1000',
        entry_type: 'Debit',
        date: '2026-01-31'
      },
      {
        id: 'e2',
        schedule_id: 'sched-1',
        account_id: account_data[1].id,
        description: 'Entry 2',
        amount: '1000',
        entry_type: 'Credit',
        date: '2026-01-31'
      }
    ]
  }

  const { container } = render(EditSchedule, {
    close,
    curSchedule,
    loadSchedules,
    view,
  })

  await waitFor(() => {
    const endRadio = container.querySelector('#end')
    const noEndRadio = container.querySelector('#noEnd')
    expect(endRadio).toBeTruthy()
    expect(noEndRadio).toBeTruthy()
    expect(endRadio.checked).toBe(true)
    expect(noEndRadio.checked).toBe(false)
  })
})
