import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    onwarn: (warning, handler) => {
        if (warning.code === 'a11y-click-events-have-key-events') return
        if (warning.code === "a11y-no-static-element-interactions") return
        if (warning.code.startsWith('a11y-')) return
        if (warning.code === 'missing-exports-condition') return
        if (warning.code === 'a11y-no-static-element-interactions') return
        if (warning.code === 'a11y-autofocus') return
        if (warning.code.startsWith('css-unused-selector')) return
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
