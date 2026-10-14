import { transform, type CompilerOptions } from '@vue-jsx/compiler'
import type { Options } from '../options'
import type { UnpluginMessage } from 'unplugin'

export type { Options }

export function transformVueJsxVapor(code: string, id: string, options: CompilerOptions = {}) {
  const params = new URLSearchParams(id)
  const vapor = params.has('vapor')
  const errors: UnpluginMessage[] = []
  const warnings: UnpluginMessage[] = []
  const result = transform(code, {
    filename: id,
    ...options,
    vapor: vapor || (options.vapor ?? true),
    onError: (error) => {
      errors.push(error as UnpluginMessage)
      options.onError?.(error)
    },
    onWarn: (warning) => {
      warnings.push(warning as UnpluginMessage)
      options.onWarn?.(warning)
    },
  })
  return { ...result, errors, warnings }
}
