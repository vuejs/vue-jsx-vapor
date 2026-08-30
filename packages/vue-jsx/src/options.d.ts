import type { CompilerOptions } from '@vue-jsx/compiler'
import type { Options as MacrosOptions } from '@vue-jsx/macros'
import type { FilterPattern } from 'unplugin'

export interface Options extends CompilerOptions {
  // define your plugin options here
  include?: FilterPattern
  exclude?: FilterPattern
  /**
   * @default false
   * By default, only JSX elements inside `defineVaporComponent` / `defineVaporCustomElement`,
   * or in files ending with `.vapor.jsx` / `.vapor.tsx` (e.g., `Comp.vapor.tsx`), will be compiled to Vapor DOM.
   * Set this to `true` if you want all JSX elements to be compiled to Vapor DOM.
   */
  vapor?: boolean
  /** @default true */
  ref?:
    | {
        alias?: string[]
      }
    | boolean
  /** @default false */
  macros?: MacrosOptions | boolean
}
