import unplugin from './unplugin'
import type { Options } from './options'

export default (options: Options) => ({
  name: 'vue-jsx',
  hooks: {
    'astro:config:setup': (astro: any) => {
      astro.config.vite.plugins ||= []
      astro.config.vite.plugins.push(unplugin.vite(options))
    },
  },
})
