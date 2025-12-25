import unplugin from '.'
import type { Options } from './options'

export default (options: Options) => ({
  name: 'vue-jsx-vapor',
  hooks: {
    'astro:config:setup': (astro: any) => {
      astro.config.vite.plugins ||= []
      astro.config.vite.plugins.push(unplugin.vite(options))
    },
  },
})
