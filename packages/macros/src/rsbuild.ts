import unplugin from './index'
import type { Options } from './options'

export default (options: Options = {}) => ({
  name: 'rsbuild:vue-jsx-vapor-macros',
  setup(api: any) {
    api.modifyBundlerChain((chain: any) => {
      chain.plugin('vue-jsx-vapor-macros').use(unplugin.rspack(options))
    })
  },
})
