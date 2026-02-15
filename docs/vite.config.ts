import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'

export default defineConfig({
  resolve: {
    conditions: ['jsx-vapor-dev'],
    alias: {
      '~': fileURLToPath(new URL('./', import.meta.url)),
    },
  },
  optimizeDeps: {
    exclude: ['vitepress'],
  },
})
