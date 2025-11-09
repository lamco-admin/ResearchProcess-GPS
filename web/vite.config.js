import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import wasm from 'vite-plugin-wasm'

export default defineConfig({
  plugins: [
    svelte(),
    wasm()
  ],
  server: {
    port: 3000,
    fs: {
      allow: ['..']
    }
  },
  build: {
    target: 'esnext'
  },
  optimizeDeps: {
    exclude: ['rp-wasm']
  }
})
