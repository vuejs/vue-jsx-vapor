import unplugin from './unplugin'
import type { Options } from './options'

export default (options: Options = {}) => ({
  name: 'rsbuild:vue-jsx-compiler',
  setup(api: any) {
    api.modifyBundlerChain((chain: any) => {
      chain.plugin('vue-jsx').use(unplugin.rspack(options))
    })
  },
})
