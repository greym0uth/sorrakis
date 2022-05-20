import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [solidPlugin(), wasm()],
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
