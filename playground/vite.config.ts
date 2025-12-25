import Vue from '@vitejs/plugin-vue'
import DefineRender from '@vue-macros/define-render/vite'
import { defineConfig } from 'vite'
import Inspect from 'vite-plugin-inspect'
import VueJsxVapor from 'vue-jsx-vapor/vite'

export default defineConfig({
  plugins: [
    Vue(),
    VueJsxVapor({
      interop: true,
      macros: true,
    }),
    DefineRender({
      vapor: true,
    }),
    Inspect(),
  ],
})
