import { transform } from '@vue-jsx-vapor/compiler-rs'
import type { Options } from '../options'

export interface CompilerError extends SyntaxError {
  loc?: [number, number]
}

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
  const errors: CompilerError[] = []
  const warnings: CompilerError[] = []
  const { onError, onWarn, ...compiler } = options?.compiler || {}
  const result = transform(code, {
    filename: id,
    sourceMap: needSourceMap,
    interop: vapor ? false : options?.interop,
    hmr: needHMR,
    ssr,
    ...compiler,
    onError: (error) => {
      errors.push(error as CompilerError)
      onError?.(error)
    },
    onWarn: (warning) => {
      warnings.push(warning as CompilerError)
      onWarn?.(warning)
    },
  })
  return { ...result, errors, warnings }
}
