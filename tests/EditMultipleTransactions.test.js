import { render, cleanup, fireEvent } from '@testing-library/svelte'
import EditMultipleTransactions from '../src/transactions/EditMultipleTransactions.svelte'
import {accounts} from '../src/stores/accounts.js'
import {page, views, modes} from '../src/stores/page.js'
import account_data from './data/account_data.json'
import transaction_data from './data/transaction_data.json'
import { locale } from 'svelte-i18n'
import '../src/utils/i18n'
import { config } from '../src/stores/config.js'

locale.set('en')
accounts.set(account_data)
const loadTransactions = () => {}
const onClose = () => {}
config.set({display_date_format: "US"})

afterEach(() => cleanup())

it('is displayed correctly for multiple', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.MULTI_EDIT})
    const select = render(EditMultipleTransactions, {loadTransactions: loadTransactions, curAccount: account_data[0], onClose: onClose, transactions: transaction_data})
    expect(select.container.outerHTML).toMatchSnapshot();
});

it('disables already-selected accounts in other entry selects (simple)', async () => {
    page.set({view: views.TRANSACTIONS, mode: modes.MULTI_EDIT})
    const { container } = render(EditMultipleTransactions, {
        loadTransactions,
        curAccount: account_data[0],
        onClose,
        transactions: transaction_data
    })

    const getOptionByText = (selectEl, text) =>
        Array.from(selectEl.querySelectorAll('option')).find(o => o.textContent === text)

    const selectOption = async (selectEl, text) => {
        const option = getOptionByText(selectEl, text)
        const options = Array.from(selectEl.querySelectorAll('option'))
        const optionIndex = options.indexOf(option)
        selectEl.selectedIndex = optionIndex
        option.selected = true
        await fireEvent.change(selectEl)
        await new Promise(resolve => setTimeout(resolve, 0))
    }

    // Simple mode: two selects
    let selects = Array.from(container.querySelectorAll('select'))
    expect(selects.length).toBeGreaterThanOrEqual(2)
    let debitSelect = selects[0]
    let creditSelect = selects[1]

    await selectOption(debitSelect, 'Account 1')

    selects = Array.from(container.querySelectorAll('select'))
    creditSelect = selects[1]

    expect(getOptionByText(creditSelect, 'Account 1').disabled).toBe(true)
})

// TODO Multi Edit needs work for compound entries
// it('disables already-selected accounts in other entry selects (compound)', async () => {
//     page.set({view: views.TRANSACTIONS, mode: modes.MULTI_EDIT})
//     const { container } = render(EditMultipleTransactions, {
//         loadTransactions,
//         curAccount: account_data[0],
//         onClose,
//         transactions: transaction_data
//     })

//     const getOptionByText = (selectEl, text) =>
//         Array.from(selectEl.querySelectorAll('option')).find(o => o.textContent === text)

//     const selectOption = async (selectEl, text) => {
//         const option = getOptionByText(selectEl, text)
//         selectEl.value = option.value
//         await fireEvent.change(selectEl)
//         await new Promise(resolve => setTimeout(resolve, 0))
//     }

//     const compoundCheckbox = container.querySelector('input#compound')
//     compoundCheckbox.click()
//     await new Promise(resolve => setTimeout(resolve, 10))

//     let selects = Array.from(container.querySelectorAll('select'))
//     expect(selects.length).toBeGreaterThanOrEqual(2)
//     let row0Select = selects[0]
//     let row1Select = selects[1]

//     await selectOption(row0Select, 'Account 1')

//     // Set row1 explicitly to Account 2 (so we're not relying on default selection)
//     await selectOption(row1Select, 'Account 2')

//     selects = Array.from(container.querySelectorAll('select'))
//     row0Select = selects[0]
//     row1Select = selects[1]

//     expect(getOptionByText(row1Select, 'Account 1').disabled).toBe(true)
//     expect(getOptionByText(row0Select, 'Account 2').disabled).toBe(true)

//     // Change row0 to Account 3; row1 should now disable Account 3 and re-enable Account 1
//     await selectOption(row0Select, 'Account 3')

//     selects = Array.from(container.querySelectorAll('select'))
//     row1Select = selects[1]
//     expect(getOptionByText(row1Select, 'Account 3').disabled).toBe(true)
//     expect(getOptionByText(row1Select, 'Account 1').disabled).toBe(false)
// })
