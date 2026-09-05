import Vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'
import Inspect from 'vite-plugin-inspect'
import VueJsx from 'vue-jsx/vite'

export default defineConfig({
  plugins: [Vue(), VueJsx({ vapor: true }), Inspect()],
})
