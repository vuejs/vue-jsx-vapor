import { defineConfig } from 'vite'

export default defineConfig({
  resolve: {
    conditions: ['jsx-vapor-dev'],
  },
  optimizeDeps: {
    exclude: ['vitepress'],
  },
})
