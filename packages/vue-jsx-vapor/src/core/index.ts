import { transform, type CompilerOptions } from '@vue-jsx/compiler'
import type { Options } from '../options'

export type { Options }

export function transformVueJsxVapor(
  code: string,
  id: string,
  options: CompilerOptions = {},
) {
  const params = new URLSearchParams(id)
  const vapor = params.has('vapor')
  return transform(code, {
    filename: id,
    ...options,
    vapor: vapor || (options.vapor ?? true),
  })
}
