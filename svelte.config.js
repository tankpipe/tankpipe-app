import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    onwarn: (warning, handler) => {
        if (warning.code.startsWith('a11y-')) return
        if (warning.code.startsWith('a11y_')) return
        if (warning.code === 'missing-exports-condition') return
        if (warning.code.startsWith('css-')) return
        if (warning.code.startsWith('css_')) return
        handler(warning)
    },
	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter()
	}
};

export default config;
