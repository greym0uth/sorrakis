import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  plugins: [solidPlugin(), wasmPack([], ['@sorrakis/webgl'])],
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
