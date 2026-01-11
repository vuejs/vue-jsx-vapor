import type { CompilerOptions } from '@vue-jsx-vapor/compiler-rs'
import type { Options as MacrosOptions } from '@vue-jsx-vapor/macros'
import type { FilterPattern } from 'unplugin'

export interface Options {
  // define your plugin options here
  include?: FilterPattern
  exclude?: FilterPattern
  interop?: boolean
  compiler?: CompilerOptions
  sourceMap?: boolean
  /** @default true */
  ref?:
    | {
        alias?: string[]
      }
    | boolean
  /** @default false */
  macros?: MacrosOptions | boolean
}
