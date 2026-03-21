import { render } from '@testing-library/svelte'
import Select from '../src/components/Select.svelte'

it('is displayed correctly', async () => {

    let items = []
    items.push({
        id: "a7bb5dd7-7787-404a-9678-00e6be32fe9e",
        name: "Account 1",
        account_type: "Debit",
	    balance: 0,
	    starting_balance: 0
    })

    let item = null
    const label = "Label 1"
    const select = render(Select, {item: item, items: items, label, none:true})

    expect(select.container.outerHTML).toMatchSnapshot();
});

it('shows in error', async () => {

    let items = []
    items.push({
        id: "a7bb5dd7-7787-404a-9678-00e6be32fe9e",
        name: "Account 1",
        account_type: "Debit",
	    balance: 0,
	    starting_balance: 0
    })

    let item = null
    const label = "Label 1"
    const select = render(Select, {item: item, items: items, label, none:true, inError:true})

    expect(select.container.outerHTML).toMatchSnapshot();
});

it('is displayed correctly with no label and no none', async () => {

    let items = []
    items.push({
        id: "a7bb5dd7-7787-404a-9678-00e6be32fe9e",
        name: "Account 1",
        account_type: "Debit",
	    balance: 0,
	    starting_balance: 0
    })

    let item = null
    const label = "Label 1"
    const select = render(Select, {item: item, items: items})

    expect(select.container.outerHTML).toMatchSnapshot();
});

it('disables options for disabledItems', async () => {
    const items = [
        { id: 'a1', name: 'Account 1', account_type: 'Debit', balance: 0, starting_balance: 0 },
        { id: 'a2', name: 'Account 2', account_type: 'Debit', balance: 0, starting_balance: 0 },
    ]

    const disabledItems = [items[1]]
    const select = render(Select, { item: null, items, disabledItems, valueField: 'id', none: true })

    const options = Array.from(select.container.querySelectorAll('option'))
    // options: None, Account 1, Account 2
    expect(options).toHaveLength(3)
    expect(options[1].disabled).toBe(false)
    expect(options[2].disabled).toBe(true)
})
