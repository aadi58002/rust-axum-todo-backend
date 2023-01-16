import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: vitePreprocess(),

  kit: {
    alias: {
      types: 'src/lib/types/*',
      atoms: 'src/lib/components/atoms/*',
      molecules: 'src/lib/components/molecules/*',
      organisms: 'src/lib/components/organisms/*'
    },
    adapter: adapter()
  }
};

export default config;
