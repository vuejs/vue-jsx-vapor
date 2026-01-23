import Vue from '@vitejs/plugin-vue'
import DefineRender from '@vue-macros/define-render/vite'
import { defineConfig } from 'vite'
import Inspect from 'vite-plugin-inspect'
import VueJsxVapor from 'vue-jsx-vapor/vite'

const interops = ['*/vdom/*.tsx', '*/interop.tsx']
export default defineConfig({
  plugins: [
    Vue(),
    VueJsxVapor({
      include: interops,
      interop: true,
      macros: true,
      compiler: {
        runtimeModuleName: 'vue-jsx-vapor',
      },
    }),
    VueJsxVapor({
      exclude: interops,
      macros: true,
      compiler: {
        runtimeModuleName: 'vue-jsx-vapor',
      },
    }),
    DefineRender({
      vapor: true,
    }),
    Inspect(),
  ],
})
