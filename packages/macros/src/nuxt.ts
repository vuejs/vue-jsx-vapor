import { addVitePlugin, addWebpackPlugin, defineNuxtModule } from '@nuxt/kit'
import vite from './vite'
import webpack from './webpack'
import type { Options } from './options'
import '@nuxt/schema'

export interface ModuleOptions extends Options {}

export default defineNuxtModule<ModuleOptions>({
  meta: {
    name: '@vue-jsx/macros',
    configKey: 'vueJsxMacros',
  },
  setup(options) {
    addVitePlugin(() => vite(options))
    addWebpackPlugin(() => webpack(options))

    // ...
  },
})
