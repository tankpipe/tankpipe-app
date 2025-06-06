import { writable } from 'svelte/store'
import { SvelteMap } from 'svelte/reactivity';

let selectedTransactions = new SvelteMap()
const selector = writable({selectedTransactions: selectedTransactions, showMultiEdit: false, shapeMatch: false, isSelectAll: false, showMultipleSelect: true})

const shapeOf = (transaction) => {
    return transaction.entries.map(e => e.account_id).join("_")
}

const checkShapes = (selector, checkShape = null) => {
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
        } else if (selector.shapeMatch && checkShape == shape) {  // short circut - only need to check first
            return
        }

    }

    updateSelector({ shapeMatch: true })
}

const toggleSelected = (selector, transaction) => {

    if (selectedTransactions.has(transaction.id)) {
        selectedTransactions.delete(transaction.id)
        checkShapes(selector)
    } else {
        const shape = shapeOf(transaction)
        selectedTransactions.set(transaction.id, shape)
        if (selectedTransactions.size == 1) {
            updateSelector({ shapeMatch: true })
        } else {
            checkShapes(selector, shape)
        }

    }
    selector.showMultiEdit = selector.showMultipleSelect && selectedTransactions.size > 0
}

const toggleAllSelected = (selector, transactions) => {
    if (selector.isSelectAll) {
        selector.selectedTransactions.clear()
    } else {
        transactions.forEach(t => selector.selectedTransactions.set(t.id, shapeOf(t)))
    }
    checkShapes(selector)
        updateSelector({
            showMultiEdit: selector.showMultipleSelect && selector.selectedTransactions.size > 0,
            isSelectAll: !selector.isSelectAll,
        })

}

const toggleMultipleSelect = (selector) => {
    selector.showMultipleSelect = !selector.showMultipleSelect
    if (!selector.showMultipleSelect) {
            updateSelector({
                isSelectAll: false,
            })

        selectedTransactions.clear()
    }
        updateSelector({
            showMultiEdit: selector.showMultipleSelect && selectedTransactions.size > 0
        })


}

const clearSelected = (selector, ) => {
    selectedTransactions.clear()
    updateSelector({
        showMultiEdit: false,
        isSelectAll: false,
    })
}

const isSelected = (selector, transaction) => {
    return selectedTransactions.has(transaction.id)
}

const updateSelector = (update) => {
    selector.set(
        Object.assign(selector, update)
    )
}

export {
    selector,
    toggleSelected,
    toggleAllSelected,
    toggleMultipleSelect,
    clearSelected,
    isSelected
}