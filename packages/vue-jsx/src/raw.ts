import macros from '@vue-jsx/macros/raw'
import {
  propsHelperCode,
  propsHelperId,
  ssrHelperCode,
  ssrHelperId,
  vaporHelperCode,
  vaporHelperId,
  vdomHelperCode,
  vdomHelperId,
} from '@vue-jsx/runtime/raw'
import { relative } from 'pathe'
import { normalizePath } from 'unplugin-utils'
import { transformVueJsx } from './core'
import type { Options } from './options'
import type { UnpluginOptions } from 'unplugin'

const plugin = (options: Options = {}): UnpluginOptions[] => {
  let root = ''
  let hmr = false
  let sourceMap = false
  const helperId = /^\/vue-jsx-vapor\//
  return [
    ...(options.macros === false
      ? []
      : options.macros
        ? macros(options.macros === true ? undefined : options.macros)
        : []),
    {
      enforce: 'pre',
      name: 'vue-jsx',
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
          hmr = config.command === 'serve'
          sourceMap = config.command === 'serve' || !!config.build.sourcemap
        },
      },
      resolveId: {
        filter: {
          id: helperId,
        },
        handler: (id) => id,
      },
      load: {
        filter: {
          id: helperId,
        },
        handler(id) {
          if (id === ssrHelperId) return ssrHelperCode
          if (id === propsHelperId) return propsHelperCode
          if (id === vdomHelperId) return vdomHelperCode
          if (id === vaporHelperId) return vaporHelperCode
        },
      },
      transform: {
        filter: {
          id: {
            include: options?.include || /\.[cm]?[jt]sx(?=$|[?#])/,
            exclude: options?.exclude || /node_modules/,
          },
        },
        handler(code, id, { ssr }: { ssr?: boolean } = {}) {
          const result = transformVueJsx(
            code,
            ssr ? normalizePath(relative(root, id)) : id,
            {
              hmr,
              sourceMap,
              ssr,
              ...options,
            },
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
