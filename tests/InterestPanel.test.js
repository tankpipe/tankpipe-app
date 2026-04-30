import { fireEvent, render, waitFor } from '@testing-library/svelte'
import InterestPanel from '../src/accounts/InterestPanel.svelte'
import { accounts } from '../src/stores/accounts.js'
import { locale } from 'svelte-i18n'
import '../src/utils/i18n'

locale.set('en')

it('restricts income and interest account selects by account_type', async () => {
  accounts.set([
    { id: 'a1', name: 'Bank', account_type: 'Asset' },
    { id: 'l1', name: 'Credit Card', account_type: 'Liability' },
    { id: 'e1', name: 'Owner Equity', account_type: 'Equity' },
    { id: 'r1', name: 'Interest Income', account_type: 'Revenue' },
    { id: 'x1', name: 'Interest Expense', account_type: 'Expense' },
  ])

  const loadAccounts = () => {}

  const mountAndStartEditing = async (curAccount) => {
    const { container } = render(InterestPanel, { curAccount, loadAccounts, editMode: true })
    const addButton = container.querySelector('button.toolbar-icon[title="Add an interest entry"]')
    await fireEvent.click(addButton)
    await waitFor(() => expect(container.querySelectorAll('select')).toHaveLength(1))
    return { container }
  }

  // Asset: income_account_id shows Revenue only
  {
    const { container } = await mountAndStartEditing({
      id: 'a1',
      name: 'Bank',
      account_type: 'Asset',
      interest_id: null,
    })

    const selectsBeforeAdvanced = Array.from(container.querySelectorAll('select'))
    expect(selectsBeforeAdvanced).toHaveLength(1)
    const incomeOptionTexts = Array.from(selectsBeforeAdvanced[0].querySelectorAll('option')).map(
      (o) => o.textContent
    )
    expect(incomeOptionTexts).toEqual(['None', 'Interest Income'])

    const advancedLabel = container.querySelector('label.toggle-label')
    const advancedToggle = advancedLabel.closest('.toolbar').querySelector('button.toolbar-icon')
    await fireEvent.click(advancedToggle)
    await waitFor(() => expect(container.querySelectorAll('select')).toHaveLength(2))

    const selectsAfterAdvanced = Array.from(container.querySelectorAll('select'))
    expect(selectsAfterAdvanced).toHaveLength(2)
    const interestOptionTexts = Array.from(selectsAfterAdvanced[1].querySelectorAll('option')).map(
      (o) => o.textContent
    )
    expect(interestOptionTexts).toEqual(['None', 'Bank', 'Credit Card'])
  }

  // Liability: income_account_id shows Expense only
  {
    const { container } = await mountAndStartEditing({
      id: 'l1',
      name: 'Credit Card',
      account_type: 'Liability',
      interest_id: null,
    })

    const selectsBeforeAdvanced = Array.from(container.querySelectorAll('select'))
    expect(selectsBeforeAdvanced).toHaveLength(1)
    const incomeOptionTexts = Array.from(selectsBeforeAdvanced[0].querySelectorAll('option')).map(
      (o) => o.textContent
    )
    expect(incomeOptionTexts).toEqual(['None', 'Interest Expense'])
  }
})
