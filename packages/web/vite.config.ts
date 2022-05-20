import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  plugins: [solidPlugin(), wasmPack([], ['@greym0uth/playground'])],
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
