import unplugin from './unplugin'
import type { Options } from './core'

export default (options: Options = {}) => ({
  name: 'rsbuild:vue-jsx-vapor',
  setup(api: any) {
    api.modifyBundlerChain((chain: any) => {
      chain.plugin('vue-jsx-vapor').use(unplugin.rspack(options))
    })
  },
})
