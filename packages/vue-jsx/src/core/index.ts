import { transform, type CompilerOptions } from '@vue-jsx/compiler'
import { UnpluginMessage } from 'unplugin'

export type { CompilerOptions }

export function transformVueJsx(code: string, id: string, options: CompilerOptions = {}) {
  const params = new URLSearchParams(id)
  const vapor = params.has('vapor')
  const errors: UnpluginMessage[] = []
  const warnings: UnpluginMessage[] = []
  const result = transform(code, {
    filename: id,
    ...options,
    vapor: vapor || options?.vapor,
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
