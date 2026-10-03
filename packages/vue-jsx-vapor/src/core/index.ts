import { transform, type CompilerError } from '@vue-jsx-vapor/compiler-rs'
import type { Options } from '../options'

export type { Options }

export type CompilerDiagnostic = Omit<CompilerError, 'code'> & {
  code?: number
}

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
  const errors: CompilerDiagnostic[] = []
  const warnings: CompilerDiagnostic[] = []
  const { onError, onWarn, ...compiler } = options?.compiler || {}
  const result = transform(code, {
    filename: id,
    sourceMap: needSourceMap,
    interop: vapor ? false : options?.interop,
    hmr: needHMR,
    ssr,
    ...compiler,
    onError: (error) => {
      errors.push(error as CompilerDiagnostic)
      onError?.(error)
    },
    onWarn: (warning) => {
      warnings.push(warning as CompilerDiagnostic)
      onWarn?.(warning)
    },
  })
  return { ...result, errors, warnings }
}
