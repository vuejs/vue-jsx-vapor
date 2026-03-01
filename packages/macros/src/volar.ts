import { createPlugin, type PluginReturn } from 'ts-macro'
import { createFilter } from 'unplugin-utils'
import { resolveOptions, type Options } from './options'
import { getGlobalTypes, getRootMap, transformJsxMacros } from './volar/index'

const REGEX_VUE_SFC: RegExp = /\.vue$/

const plugin: PluginReturn<Options | undefined> = createPlugin(
  ({ ts }, userOptions = {}) => {
    const resolvedOptions = resolveOptions(userOptions!)
    ;(resolvedOptions.include as any[]).push(REGEX_VUE_SFC)
    const filter = createFilter(
      resolvedOptions.include,
      resolvedOptions.exclude,
    )

    return {
      name: '@vue-jsx-vapor/macros',
      resolveVirtualCode(virtualCode) {
        const { filePath, codes } = virtualCode
        if (!filter(filePath)) return

        const options = {
          ts,
          ...virtualCode,
          ...resolvedOptions,
        }
        const rootMap = getRootMap(options)
        if (rootMap.size) {
          transformJsxMacros(rootMap, options)
        }
        /**
         * JSX and inline import types co-usage will break TS.
         * So use `import ''` to fixed it
         * ```tsx
         * const foo = <div></div>
         * const bar = {} as import('vue').ShallowUnwrapRef<any>
         * ```
         */
        codes.unshift("import '';\n")
        codes.push(getGlobalTypes(rootMap, options))
      },
    }
  },
)
export default plugin
export { plugin as 'module.exports' }
