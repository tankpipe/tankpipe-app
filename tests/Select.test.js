import { render } from '@testing-library/svelte'
import Select from '../src/Select.svelte'

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
