import { writable, get } from 'svelte/store'

const context = writable({})

const initializeContext = () => {
    context.set({
        hasBooks: false,
        initialising: true
    })
}

const updateContext = (update) => {
    context.update(current => Object.assign({}, current, update))
}

const setInitialising = (isInitialising) => {
    context.update(value => (Object.assign(value, {initialising: isInitialising})))
}

const isInitialising = () => {
    return context.initialising
}

const hasBooks = () => {
    return context.hasBooks
}


const setHasBooks = (hasBooks) => {
    context.update(value => (Object.assign(value, {hasBooks: hasBooks})))
}

export {context, initializeContext, updateContext, setInitialising, isInitialising, hasBooks, setHasBooks}