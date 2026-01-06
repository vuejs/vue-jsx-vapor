import { transformJsxMacros } from './core'
import {
  useModelHelperCode,
  useModelHelperId,
  withDefaultsHelperCode,
  withDefaultsHelperId,
} from './core/helper'
import { transformStyle } from './core/style'
import { resolveOptions, type Options } from './options'
import type { UnpluginOptions } from 'unplugin'

const plugin = (userOptions: Options = {}): UnpluginOptions[] => {
  const options = resolveOptions(userOptions)
  const importMap = new Map()
  const defineStyleRegex = /^\/vue-jsx-vapor\/macros\/define-style/

  return [
    {
      name: '@vue-jsx-vapor/macros',
      enforce: 'pre',

      resolveId: {
        filter: {
          id: {
            include: [useModelHelperId, withDefaultsHelperId],
          },
        },
        handler(id) {
          return id
        },
      },
      load: {
        filter: {
          id: {
            include: [useModelHelperId, withDefaultsHelperId],
          },
        },
        handler(id) {
          if (id === useModelHelperId) return useModelHelperCode
          if (id === withDefaultsHelperId) return withDefaultsHelperCode
        },
      },

      transform: {
        filter: {
          id: {
            include: options.include || /\.[cm]?[jt]sx$/,
          },
        },
        handler(code, id, opt?: { ssr?: boolean }) {
          if (opt?.ssr) {
            options.defineComponent.autoReturnFunction = true
          }
          return transformJsxMacros(code, id, importMap, options)
        },
      },
    },
    {
      name: '@vue-jsx-vapor/macros/define-style',

      resolveId: {
        filter: {
          id: {
            include: defineStyleRegex,
          },
        },
        handler(id) {
          return id
        },
      },
      load: {
        filter: {
          id: {
            include: defineStyleRegex,
          },
        },
        handler(id) {
          return importMap.get(id)
        },
      },

      transform: {
        filter: {
          id: {
            include: defineStyleRegex,
          },
        },
        handler(code, id) {
          return transformStyle(code, id, options)
        },
      },
    },
  ]
}
export default plugin
