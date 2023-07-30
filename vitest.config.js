import {defineConfig} from 'vitest/config'
import {svelte} from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte({hot: !process.env.VITEST})],
  test: {
    include: ['tests/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    globals: true,
    environment: 'jsdom',
    // https://github.com/vitest-dev/vitest/issues/2834
    alias: [{ find: /^svelte$/, replacement: 'svelte/internal' }],
  },
})