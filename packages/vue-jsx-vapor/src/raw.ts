import macros from '@vue-jsx-vapor/macros/raw'
import {
  propsHelperCode,
  propsHelperId,
  vnodeHelperCode,
  vnodeHelperId,
} from '@vue-jsx-vapor/runtime/raw'
import { relative } from 'pathe'
import { createFilter, normalizePath } from 'unplugin-utils'
import { transformVueJsxVapor } from './core'
import { ssrRegisterHelperCode, ssrRegisterHelperId } from './core/ssr'
import type { Options } from './options'
import type { UnpluginOptions } from 'unplugin'

const plugin = (options: Options = {}): UnpluginOptions[] => {
  const transformInclude = createFilter(
    options?.include || /\.[cm]?[jt]sx$/,
    options?.exclude || /node_modules/,
  )
  let root = ''
  let needHMR = false
  let needSourceMap = options.sourceMap || false
  return [
    ...(options.macros === false
      ? []
      : options.macros
        ? [macros(options.macros === true ? undefined : options.macros)]
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
      resolveId(id) {
        if (
          id === ssrRegisterHelperId ||
          id === vnodeHelperId ||
          id === propsHelperId
        )
          return id
      },
      loadInclude(id) {
        if (
          id === ssrRegisterHelperId ||
          id === vnodeHelperId ||
          id === propsHelperId
        )
          return true
      },
      load(id) {
        if (id === ssrRegisterHelperId) return ssrRegisterHelperCode
        if (id === vnodeHelperId) return vnodeHelperCode
        if (id === propsHelperId) return propsHelperCode
      },
      transformInclude,
      transform(code, id, opt?: { ssr?: boolean }) {
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
  ]
}
export default plugin
