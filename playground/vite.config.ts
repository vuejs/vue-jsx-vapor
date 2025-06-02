import { defineConfig } from 'vite'
import Inspect from 'vite-plugin-inspect'
import VueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [
    VueJsxVapor({
      macros: true,
    }),
    Inspect(),
  ],
})
