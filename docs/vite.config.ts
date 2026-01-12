import { defineConfig } from 'vite'
import vueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  resolve: {
    conditions: ['jsx-vapor-dev'],
  },
  optimizeDeps: {
    exclude: ['vitepress'],
  },
  plugins: [vueJsxVapor({ interop: true, macros: true })],
})
