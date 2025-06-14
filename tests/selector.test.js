import { selector, toggleSelected, toggleAllSelected, toggleMultipleSelect, clearSelected, isSelected, getSelected } from '../src/selector'
import { get } from 'svelte/store'

const mockTransaction1 = {
    id: 1,
    entries: [
        { account_id: 'acc1' },
        { account_id: 'acc2' }
    ]
}

const mockTransaction2 = {
    id: 2,
    entries: [
        { account_id: 'acc1' },
        { account_id: 'acc2' }
    ]
}

const mockTransaction3 = {
    id: 3,
    entries: [
        { account_id: 'acc3' },
        { account_id: 'acc4' }
    ]
}

describe('Selector functionality', () => {
    beforeEach(() => {
        clearSelected()
        selector.set({
            showMultiEdit: false,
            shapeMatch: false,
            isSelectAll: false,
            showMultipleSelect: false
        })
    })

    it('toggles single transaction selection', () => {
        toggleMultipleSelect()
        toggleSelected(mockTransaction1)
        expect(isSelected(mockTransaction1)).toBe(true)
        expect(get(selector).showMultiEdit).toBe(true)
        expect(get(selector).shapeMatch).toBe(true)
    })

    it('handles multiple transactions with same shape', () => {
        toggleSelected(mockTransaction1)
        toggleSelected(mockTransaction2)
        expect(isSelected(mockTransaction1)).toBe(true)
        expect(isSelected(mockTransaction2)).toBe(true)
        expect(get(selector).shapeMatch).toBe(true)
    })

    it('handles multiple transactions with different shapes', () => {
        toggleSelected(mockTransaction1)
        toggleSelected(mockTransaction3)
        expect(isSelected(mockTransaction1)).toBe(true)
        expect(isSelected(mockTransaction3)).toBe(true)
        expect(get(selector).shapeMatch).toBe(false)
    })

    it('toggles all transactions correctly', () => {
        const transactions = [mockTransaction1, mockTransaction2, mockTransaction3]
        toggleAllSelected(transactions)
        expect(get(selector).isSelectAll).toBe(true)
        expect(getSelected().length).toBe(3)

        toggleAllSelected(transactions)
        expect(get(selector).isSelectAll).toBe(false)
        expect(getSelected().length).toBe(0)
    })

    it('handles multiple select toggle', () => {
        toggleMultipleSelect()
        expect(getSelected().length).toBe(0)
        expect(get(selector).showMultiEdit).toBe(false)
        toggleSelected(mockTransaction1)
        expect(get(selector).showMultipleSelect).toBe(true)
        expect(getSelected().length).toBe(1)
        expect(get(selector).showMultiEdit).toBe(true)
    })

    it('clears selection correctly', () => {
        toggleSelected(mockTransaction1)
        toggleSelected(mockTransaction2)
        clearSelected()
        expect(getSelected().length).toBe(0)
        expect(get(selector).showMultiEdit).toBe(false)
        expect(get(selector).isSelectAll).toBe(false)
    })

    it('returns correct selected transactions', () => {
        toggleSelected(mockTransaction1)
        toggleSelected(mockTransaction2)
        const selected = getSelected()
        expect(selected).toContain(mockTransaction1.id)
        expect(selected).toContain(mockTransaction2.id)
        expect(selected.length).toBe(2)
    })
})
