import macros from '@vue-jsx-vapor/macros/raw'
import {
  propsHelperCode,
  propsHelperId,
  ssrHelperCode,
  ssrHelperId,
  vaporHelperCode,
  vaporHelperId,
  vdomHelperCode,
  vdomHelperId,
} from '@vue-jsx-vapor/runtime/raw'
import { relative } from 'pathe'
import { normalizePath } from 'unplugin-utils'
import {
  transformVueJsxVapor,
  type CompilerDiagnostic,
  type Options,
} from './core'
import type { UnpluginMessage, UnpluginOptions } from 'unplugin'

const plugin = (options: Options = {}): UnpluginOptions[] => {
  let root = ''
  let needHMR = false
  let needSourceMap = options.sourceMap || false
  const helperId = /^\/vue-jsx-vapor\//
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
        handler(code, id, opt?: { ssr?: boolean }) {
          const result = transformVueJsxVapor(
            code,
            opt?.ssr ? normalizePath(relative(root, id)) : id,
            options,
            needSourceMap,
            needHMR,
            opt?.ssr,
          )
          for (const warning of result.warnings) {
            this.warn(toMessage(warning, id, code))
          }
          if (result.errors.length) {
            this.error(toMessage(result.errors[0], id, code))
          }
          if (result.code) {
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
function toMessage(
  error: CompilerDiagnostic,
  id: string,
  code: string,
): UnpluginMessage {
  const start = error.loc?.[0]
  let line = 0
  let column = 0
  if (start != null) {
    line = 1
    let byteOffset = 0
    for (const char of code) {
      if (byteOffset >= start) break
      const point = char.codePointAt(0)!
      byteOffset +=
        point < 0x80 ? 1 : point < 0x800 ? 2 : point < 0x10000 ? 3 : 4
      if (char === '\n') {
        line++
        column = 0
      } else {
        column += char.length
      }
    }
  }
  return {
    message: error.message,
    id,
    loc: start == null ? undefined : { file: id, line, column },
  }
}
export default plugin
