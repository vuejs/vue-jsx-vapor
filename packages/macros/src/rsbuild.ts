import unplugin from './index'
import type { Options } from './options'

export default (options: Options = {}) => ({
  name: 'rsbuild:vue-jsx-macros',
  setup(api: any) {
    api.modifyBundlerChain((chain: any) => {
      chain.plugin('vue-jsx-macros').use(unplugin.rspack(options))
    })
  },
})
