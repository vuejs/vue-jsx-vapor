import { createFilter, normalizePath } from '@vue-macros/common'
import { transformJsxMacros } from './core'
import {
  helperPrefix,
  useModelHelperCode,
  useModelHelperId,
  withDefaultsHelperCode,
  withDefaultsHelperId,
} from './core/helper'
import { transformStyle } from './core/style'
import { resolveOptions, type Options } from './options'
import type { UnpluginOptions } from 'unplugin'

const name = '@vue-jsx-vapor/macros'

const plugin = (userOptions: Options = {}): UnpluginOptions => {
  const options = resolveOptions(userOptions)
  const filter = createFilter(options)
  const importMap = new Map()

  return {
    name,
    enforce: 'pre',

    resolveId(id) {
      if (normalizePath(id).startsWith(helperPrefix)) return id
    },
    loadInclude(id) {
      return normalizePath(id).startsWith(helperPrefix)
    },
    load(_id) {
      const id = normalizePath(_id)
      if (id === useModelHelperId) return useModelHelperCode
      else if (id === withDefaultsHelperId) return withDefaultsHelperCode
      else if (importMap.get(id)) return importMap.get(id)
    },

    transformInclude(id) {
      if (importMap.get(id)) return true
      return filter(id)
    },
    transform(code, id, opt?: { ssr?: boolean }) {
      if (opt?.ssr) {
        options.defineComponent.autoReturnFunction = true
      }
      if (importMap.get(id)) return transformStyle(code, id, options)
      return transformJsxMacros(code, id, importMap, options)
    },
  }
}
export default plugin
