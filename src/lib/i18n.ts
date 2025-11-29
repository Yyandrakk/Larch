import { register, init, getLocaleFromNavigator } from 'svelte-i18n';

// Register the English locale
register('en', () => import('./locales/en.json'));

// Initialize the library
init({
	fallbackLocale: 'en',
	initialLocale: getLocaleFromNavigator()
});
