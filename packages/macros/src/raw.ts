import { transformJsxMacros } from './core'
import {
  useModelHelperCode,
  useModelHelperId,
  useSlotsHelperCode,
  useSlotsHelperId,
  withDefaultsHelperCode,
  withDefaultsHelperId,
} from './core/helper'
import { transformStyle } from './core/style'
import { resolveOptions, type Options } from './options'
import type { UnpluginOptions } from 'unplugin'

const plugin = (userOptions: Options = {}): UnpluginOptions[] => {
  const options = resolveOptions(userOptions)
  const importMap = new Map()
  const macrosHelperId = /^\/vue-jsx-vapor\/macros\//
  const defineStyleHelperId = /^\/vue-jsx-vapor\/macros\/define-style/

  return [
    {
      name: '@vue-jsx-vapor/macros',
      enforce: 'pre',

      resolveId: {
        filter: {
          id: macrosHelperId,
        },
        handler(id) {
          return id
        },
      },
      load: {
        filter: {
          id: macrosHelperId,
        },
        handler(id) {
          if (id === useModelHelperId) return useModelHelperCode
          if (id === withDefaultsHelperId) return withDefaultsHelperCode
          if (id === useSlotsHelperId) return useSlotsHelperCode
        },
      },

      transform: {
        filter: {
          id: {
            include: options.include,
            exclude: options.exclude,
          },
          code: [
            ...options.defineComponent.alias,
            ...options.defineExpose.alias,
            ...options.defineModel.alias,
            ...options.defineSlots.alias,
            ...options.defineStyle.alias,
          ],
        },
        handler(code, id) {
          return transformJsxMacros(code, id, importMap, options)
        },
      },
    },
    {
      name: '@vue-jsx-vapor/macros/define-style',

      resolveId: {
        filter: {
          id: defineStyleHelperId,
        },
        handler(id) {
          return id
        },
      },
      load: {
        filter: {
          id: defineStyleHelperId,
        },
        handler(id) {
          return importMap.get(id)
        },
      },

      transform: {
        filter: {
          id: defineStyleHelperId,
        },
        handler(code, id) {
          return transformStyle(code, id, options)
        },
      },
    },
  ]
}
export default plugin
