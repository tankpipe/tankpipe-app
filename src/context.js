import { writable } from 'svelte/store'

const context = writable({})

const initializeContext = () => {
    context.set({
        hasBooks: false,
    })
}

const updateContext = (update) => {
    context.set(
        Object.assign(context, update)
    )
}

export {context, initializeContext, updateContext}