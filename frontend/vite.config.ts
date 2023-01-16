import { sveltekit } from '@sveltejs/kit/vite';
import { extractorSvelte } from '@unocss/core';
import UnoCSS from 'unocss/vite';
import {
  presetTypography,
  presetIcons,
  presetUno,
  presetAttributify,
  transformerDirectives,
  transformerVariantGroup,
  transformerCompileClass
} from 'unocss';

const config = {
  plugins: [
    UnoCSS({
      extractors: [extractorSvelte],
      presets: [presetAttributify(), presetTypography(), presetIcons({ scale: 2.0 }), presetUno()],
      transformers: [transformerDirectives(), transformerVariantGroup(), transformerCompileClass()],
    }),
    sveltekit()
  ]
};

export default config;
