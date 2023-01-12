import { sveltekit } from '@sveltejs/kit/vite'
import { extractorSvelte } from '@unocss/core'
import UnoCSS from 'unocss/vite'

/** @type {import('vite').UserConfig} */
const config = {
  plugins: [
    UnoCSS({
      extractors: [extractorSvelte],
      mode: 'svelte-scoped',
      /* options */
    }),
    sveltekit(),
  ],
};

export default config;
