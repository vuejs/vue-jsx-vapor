import { transform } from '@vue-jsx-vapor/compiler-rs'
import type { Options } from '../options'
import type { UnpluginMessage } from 'unplugin'

export type { Options }

export function transformVueJsxVapor(
  code: string,
  id: string,
  options?: Options,
  needSourceMap = false,
  needHMR = false,
  ssr = false,
) {
  const params = new URLSearchParams(id)
  const vapor = params.get('vapor')
  const errors: UnpluginMessage[] = []
  const warnings: UnpluginMessage[] = []
  const { onError, onWarn, ...compiler } = options?.compiler || {}
  const result = transform(code, {
    filename: id,
    sourceMap: needSourceMap,
    interop: vapor ? false : options?.interop,
    hmr: needHMR,
    ssr,
    ...compiler,
    onError: (error) => {
      errors.push(error as UnpluginMessage)
      onError?.(error)
    },
    onWarn: (warning) => {
      warnings.push(warning as UnpluginMessage)
      onWarn?.(warning)
    },
  })
  return { ...result, errors, warnings }
}
