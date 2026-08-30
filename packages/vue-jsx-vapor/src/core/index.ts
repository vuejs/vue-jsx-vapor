import { transform } from '@vue-jsx/compiler'
import type { Options } from '../options'

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
  const vapor = params.has('vapor')
  return transform(code, {
    filename: id,
    sourceMap: needSourceMap,
    hmr: needHMR,
    ssr,
    ...options?.compiler,
    vapor: vapor || options?.compiler?.vapor || !options?.interop,
  })
}
