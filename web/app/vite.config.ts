import { sveltekit } from '@sveltejs/kit/vite';
import wasm from "vite-plugin-wasm";
import { defineConfig } from 'vitest/config';

export default defineConfig({
	server: {
		port: 4000,
	},
	plugins: [sveltekit(), wasm()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	build: {
		target: "es2022"
	}
});
