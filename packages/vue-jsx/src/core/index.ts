import { transform, type CompilerOptions } from '@vue-jsx/compiler'

export type { CompilerOptions }

export function transformVueJsx(
  code: string,
  id: string,
  options?: CompilerOptions,
) {
  const params = new URLSearchParams(id)
  const vapor = params.get('vapor')
  return transform(code, {
    filename: id,
    interop: vapor ? false : options?.interop,
    ...options,
  })
}
