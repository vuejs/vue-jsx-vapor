import Vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'
import Inspect from 'vite-plugin-inspect'
import VueJsx from 'vue-jsx/vite'

const vdom = ['*/vdom/*.tsx']

export default defineConfig({
  plugins: [
    Vue(),
    VueJsx({ include: vdom }),
    VueJsx({ exclude: vdom, vapor: true }),
    Inspect(),
  ],
})
