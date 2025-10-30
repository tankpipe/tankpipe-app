import { register, init, getLocaleFromNavigator, locale } from 'svelte-i18n'

register('en', () => import('./locales/en.json'))
locale.set('en')

init({
  fallbackLocale: 'en',
  initialLocale: getLocaleFromNavigator(),
});

console.log('Browser locale:', getLocaleFromNavigator())