import { writable, get } from 'svelte/store'
import { SvelteMap } from 'svelte/reactivity';

/* Use SvelteMap so that changes applied to the map (like select all) are reflected in the UI. */
let selectedTransactions = new SvelteMap()
const selector = writable({ showMultiEdit: false, shapeMatch: false, isSelectAll: false, showMultipleSelect: false })

const shapeOf = (transaction) => {
    return transaction.entries.map(e => e.account_id).join("_")
}

const checkShapes = (checkShape = null) => {
    if (selectedTransactions.size == 0) {
        updateSelector({ shapeMatch: false })
        return
    }

    let firstShape = checkShape
    for (const shape of selectedTransactions.values()) {

        if (firstShape == null) {
            firstShape = shape
        } else if (shape != firstShape) {
            updateSelector({ shapeMatch: false })
            return
        } else if (get(selector).shapeMatch && checkShape == shape) {  // short circut - only need to check first
            return
        }

    }

    updateSelector({ shapeMatch: true })
}

const toggleSelected = (transaction) => {

    if (selectedTransactions.has(transaction.id)) {
        selectedTransactions.delete(transaction.id)
        checkShapes()
    } else {
        const shape = shapeOf(transaction)
        selectedTransactions.set(transaction.id, shape)
        if (selectedTransactions.size == 1) {
            updateSelector({ shapeMatch: true })
        } else {
            checkShapes(shape)
        }

    }
    updateSelector({ showMultiEdit: (get(selector).showMultipleSelect && selectedTransactions.size > 0) })

}

const toggleAllSelected = (transactions) => {
    if (get(selector).isSelectAll) {
        selectedTransactions.clear()
    } else {
        transactions.forEach(t => selectedTransactions.set(t.id, shapeOf(t)))
    }
    checkShapes()
    updateSelector({
        showMultiEdit: get(selector).showMultipleSelect && selectedTransactions.size > 0,
        isSelectAll: !get(selector).isSelectAll,
    })

}

const toggleMultipleSelect = () => {
    selector.update(value => (Object.assign(value, { showMultipleSelect: !value.showMultipleSelect })))

    if (!get(selector).showMultipleSelect) {
        updateSelector({
            isSelectAll: false,
        })

        selectedTransactions.clear()
    }

    updateSelector({
        showMultiEdit: get(selector).showMultipleSelect && selectedTransactions.size > 0
    })
}

const clearSelected = () => {
    selectedTransactions.clear()
    updateSelector({
        showMultiEdit: false,
        isSelectAll: false,
    })
}

const isSelected = (transaction) => {
    return selectedTransactions.has(transaction.id)
}

const getSelected = () => {
    return Array.from(selectedTransactions.keys())
}

const updateSelector = (update) => {
    selector.update(value => (Object.assign(value, update)))
}

export {
    selector,
    toggleSelected,
    toggleAllSelected,
    toggleMultipleSelect,
    clearSelected,
    isSelected,
    getSelected
}