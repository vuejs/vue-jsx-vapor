import TwoslashFloatingVue from '@shikijs/vitepress-twoslash/client'
import DefaultTheme from 'vitepress/theme'
// https://vitepress.dev/guide/custom-theme
import * as Vue from 'vue'
import JsxRepl from './components/JsxRepl.vue'
import type { Theme } from 'vitepress'
import './style.css'
import '@shikijs/vitepress-twoslash/style.css'

export default {
  extends: DefaultTheme,
  Layout: () => {
    return Vue.h(DefaultTheme.Layout, null, {
      // https://vitepress.dev/guide/extending-default-theme#layout-slots
    })
  },
  enhanceApp({ app }) {
    app.use(TwoslashFloatingVue as any)
    app.use(Vue.vaporInteropPlugin as any)
    app.component('JsxRepl', JsxRepl)
  },
} satisfies Theme
