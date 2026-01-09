import macros from '@vue-jsx-vapor/macros/raw'
import {
  propsHelperCode,
  propsHelperId,
  vaporHelperCode,
  vaporHelperId,
  vdomHelperCode,
  vdomHelperId,
} from '@vue-jsx-vapor/runtime/raw'
import { relative } from 'pathe'
import { normalizePath } from 'unplugin-utils'
import { transformVueJsxVapor, type Options } from './core'
import { ssrRegisterHelperCode, ssrRegisterHelperId } from './core/ssr'
import type { UnpluginOptions } from 'unplugin'

const plugin = (options: Options = {}): UnpluginOptions[] => {
  let root = ''
  let needHMR = false
  let needSourceMap = options.sourceMap || false
  return [
    ...(options.macros === false
      ? []
      : options.macros
        ? macros(options.macros === true ? undefined : options.macros)
        : []),
    {
      enforce: 'pre',
      name: 'vue-jsx-vapor',
      vite: {
        config(config) {
          return {
            // only apply esbuild to ts files
            // since we are handling jsx and tsx now
            // esbuild: {
            //   include: /\.ts$/,
            // },
            define: {
              __VUE_OPTIONS_API__: config.define?.__VUE_OPTIONS_API__ ?? true,
              __VUE_PROD_DEVTOOLS__:
                config.define?.__VUE_PROD_DEVTOOLS__ ?? false,
              __VUE_PROD_HYDRATION_MISMATCH_DETAILS__:
                config.define?.__VUE_PROD_HYDRATION_MISMATCH_DETAILS__ ?? false,
            },
          }
        },
        configResolved(config) {
          root = config.root
          needHMR = config.command === 'serve'
          needSourceMap ||=
            config.command === 'serve' || !!config.build.sourcemap
        },
      },
      resolveId: {
        filter: {
          id: [ssrRegisterHelperId, propsHelperId, vdomHelperId, vaporHelperId],
        },
        handler: (id) => id,
      },
      load: {
        filter: {
          id: [ssrRegisterHelperId, propsHelperId, vdomHelperId, vaporHelperId],
        },
        handler(id) {
          if (id === ssrRegisterHelperId) return ssrRegisterHelperCode
          if (id === propsHelperId) return propsHelperCode
          if (id === vdomHelperId) return vdomHelperCode
          if (id === vaporHelperId) return vaporHelperCode
        },
      },
      transform: {
        filter: {
          id: {
            include: options?.include || /\.[cm]?[jt]sx$/,
            exclude: options?.exclude || /node_modules/,
          },
        },
        handler(code, id, opt?: { ssr?: boolean }) {
          const result = transformVueJsxVapor(
            code,
            opt?.ssr ? normalizePath(relative(root, id)) : id,
            options,
            needSourceMap,
            needHMR,
            opt?.ssr,
          )
          if (result?.code) {
            return {
              code: result.code,
              map: result.map ? JSON.parse(result.map) : null,
            }
          }
        },
      },
    },
  ]
}
export default plugin
