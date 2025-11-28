import { transform } from '@vue-jsx-vapor/compiler-rs'
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
  const vapor = params.get('vapor')
  return transform(code, {
    filename: id,
    sourceMap: needSourceMap,
    interop: vapor ? false : options?.interop,
    hmr: needHMR,
    ssr,
  })
}
