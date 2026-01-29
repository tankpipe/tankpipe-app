import { page, views, modes, isEditMode, isMultiEditMode, isSingleEditMode, isListMode } from '../src/page'
import { locale } from 'svelte-i18n'
import '../src/i18n'

locale.set('en')

describe('Page Mode Tests', () => {
    it('page store initializes with correct default values', () => {
        let currentPage
        page.subscribe(value => {
            currentPage = value
        })

        expect(currentPage.view).toBe(views.ACCOUNTS)
        expect(currentPage.mode).toBe(modes.LIST)
        expect(currentPage.payload).toEqual({})
    })

    it('isEditMode returns true for edit modes', () => {
        expect(isEditMode({ mode: modes.EDIT })).toBe(true)
        expect(isEditMode({ mode: modes.NEW })).toBe(true)
        expect(isEditMode({ mode: modes.MULTI_EDIT })).toBe(true)
        expect(isEditMode({ mode: modes.LIST })).toBe(false)
        expect(isEditMode({ mode: modes.LOAD })).toBe(false)
    })

    it('isSingleEditMode returns true only for single edit modes', () => {
        expect(isSingleEditMode({ mode: modes.EDIT })).toBe(true)
        expect(isSingleEditMode({ mode: modes.NEW })).toBe(true)
        expect(isSingleEditMode({ mode: modes.MULTI_EDIT })).toBe(false)
        expect(isSingleEditMode({ mode: modes.LIST })).toBe(false)
    })

    it('isMultiEditMode returns true only for multi edit mode', () => {
        expect(isMultiEditMode({ mode: modes.MULTI_EDIT })).toBe(true)
        expect(isMultiEditMode({ mode: modes.EDIT })).toBe(false)
        expect(isMultiEditMode({ mode: modes.NEW })).toBe(false)
        expect(isMultiEditMode({ mode: modes.LIST })).toBe(false)
    })

    it('isListMode returns true only for list mode', () => {
        expect(isListMode({ mode: modes.LIST })).toBe(true)
        expect(isListMode({ mode: modes.EDIT })).toBe(false)
        expect(isListMode({ mode: modes.NEW })).toBe(false)
        expect(isListMode({ mode: modes.MULTI_EDIT })).toBe(false)
    })

    it('views object contains all expected view types', () => {
        expect(Object.keys(views).length).toBe(8)
        expect(views.BOOKS).toBe('BOOKS')
        expect(views.ACCOUNTS).toBe('ACCOUNTS')
        expect(views.TRANSACTIONS).toBe('TRANSACTIONS')
        expect(views.SCHEDULES).toBe('SCHEDULES')
        expect(views.SETTINGS).toBe('SETTINGS')
        expect(views.NET_ASSETS).toBe('NET_ASSETS')
        expect(views.JOURNAL).toBe('JOURNAL')
        expect(views.MODIFIERS).toBe('MODIFIERS')
    })

    it('modes object contains all expected mode types', () => {
        expect(Object.keys(modes).length).toBe(6)
        expect(modes.NEW).toBe('NEW')
        expect(modes.EDIT).toBe('EDIT')
        expect(modes.MULTI_EDIT).toBe('MULTI_EDIT')
        expect(modes.LIST).toBe('LIST')
        expect(modes.LOAD).toBe('LOAD')
        expect(modes.VIEW).toBe('VIEW')
    })
})
