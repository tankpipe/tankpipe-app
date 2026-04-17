import { cleanup } from '@testing-library/svelte'
import { afterEach, beforeEach } from 'vitest'
import { locale, waitLocale } from 'svelte-i18n'
import '../src/utils/i18n.js'

locale.set('en')
await waitLocale()

beforeEach(async () => {
  locale.set('en')
  await waitLocale()
})

afterEach(() => {
  cleanup()
})
