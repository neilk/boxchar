import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { copyFileSync, mkdirSync } from 'fs'
import { resolve } from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    {
      name: 'copy-wasm-files',
      buildStart() {
        // Copy WASM pkg files
        const pkgDir = resolve(__dirname, '../pkg')
        const publicDir = resolve(__dirname, 'public/pkg')

        try {
          mkdirSync(publicDir, { recursive: true })
          copyFileSync(resolve(pkgDir, 'boxchar_bg.wasm'), resolve(publicDir, 'boxchar_bg.wasm'))
          console.log('Copied WASM files to public/pkg')
        } catch (error) {
          console.error('Failed to copy WASM files:', error)
        }

        // Copy dictionary
        const dictFile = resolve(__dirname, '../dictionary.txt')
        const publicDict = resolve(__dirname, 'public/dictionary.txt')

        try {
          copyFileSync(dictFile, publicDict)
          console.log('Copied dictionary.txt to public')
        } catch (error) {
          console.error('Failed to copy dictionary:', error)
        }
      }
    }
  ],
  optimizeDeps: {
    exclude: ['../pkg/boxchar.js']
  }
})
