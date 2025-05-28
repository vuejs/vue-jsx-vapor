import { createUnplugin, type UnpluginFactory } from 'unplugin'
import plugin from './raw'
import type { Options } from './options'

export type { Options }

export const unpluginFactory: UnpluginFactory<Options | undefined, true> = (
  options = {},
) => {
  return plugin(options)
}

export const unplugin = /* #__PURE__ */ createUnplugin(unpluginFactory)

export default unplugin
