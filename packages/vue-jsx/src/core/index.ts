import { transform, type CompilerOptions } from '@vue-jsx/compiler'

export type { CompilerOptions }

export function transformVueJsx(
  code: string,
  id: string,
  options?: CompilerOptions,
) {
  const params = new URLSearchParams(id)
  const vapor = params.has('vapor')
  return transform(code, {
    filename: id,
    ...options,
    vapor: vapor || options?.vapor,
  })
}
