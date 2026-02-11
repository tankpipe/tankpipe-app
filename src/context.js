import { writable } from 'svelte/store'

const context = writable({})

const initializeContext = () => {
    context.set({
        hasBooks: false,
        initialising: true
    })
}

const updateContext = (update) => {
    context.set(
        Object.assign(context, update)
    )
}

const setInitialising = (isInitialising) => {
    context.update(value => (Object.assign(value, {initialising: isInitialising})))
}

const isInitialising = () => {
    return context.initialising
}

export {context, initializeContext, updateContext, setInitialising, isInitialising}