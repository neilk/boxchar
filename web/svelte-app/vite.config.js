import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    port: 8000,
    host: true // Expose on network
  },
  preview: {
    port: 4173,
    host: true // Expose preview server on network too
  }
})
