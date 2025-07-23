import macros from '@vue-jsx-vapor/macros/raw'
import { transformJsxDirective } from '@vue-macros/jsx-directive/api'
import { createFilter } from 'unplugin-utils'
import { transformVueJsxVapor } from './core'
import { injectHMRAndSSR } from './core/hmr'
import { ssrRegisterHelperCode, ssrRegisterHelperId } from './core/ssr'
import { transformVueJsx } from './core/vue-jsx'
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
    {
      name: 'vue-jsx-vapor',
      vite: {
        config(config) {
          return {
            // only apply esbuild to ts files
            // since we are handling jsx and tsx now
            esbuild: {
              include: /\.ts$/,
            },
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
        if (id === ssrRegisterHelperId) return id
      },
      loadInclude(id) {
        if (id === ssrRegisterHelperId) return true
      },
      load(id) {
        if (id === ssrRegisterHelperId) return ssrRegisterHelperCode
      },
      transformInclude,
      transform(code, id, opt?: { ssr?: boolean }) {
        const result = transformVueJsxVapor(code, id, options, needSourceMap)
        if (result?.code) {
          ;(needHMR || opt?.ssr) &&
            injectHMRAndSSR(result, id, { ssr: opt?.ssr, root })
          return {
            code: result.code,
            map: result.map,
          }
        }
      },
    },
    {
      name: '@vue-macros/jsx-directive',
      transformInclude,
      transform(code, id, opt?: { ssr?: boolean }) {
        if (options.interop || opt?.ssr) {
          return transformJsxDirective(code, id, {
            lib: 'vue',
            prefix: 'v-',
            version: 3.6,
          })
        }
      },
    },
    {
      name: '@vitejs/plugin-vue-jsx',
      transformInclude,
      transform(code, id, opt?: { ssr?: boolean }) {
        if (options.interop || opt?.ssr) {
          return transformVueJsx(code, id, needSourceMap)
        }
      },
    },
    ...(options.macros === false
      ? []
      : options.macros
        ? [macros(options.macros === true ? undefined : options.macros)]
        : []),
  ]
}
export default plugin
