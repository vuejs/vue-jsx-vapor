import { Fragment, type Block, type VNode } from 'vue'
import type { h, NativeElements, ReservedProps } from 'vue-jsx-vapor'

declare function jsx(type: any, props: any, key: any): ReturnType<typeof h>
declare global {
  namespace JSX {
    type Element = VNode | Block | Element[]
    interface ElementAttributesProperty {
      $props
    }
    interface IntrinsicElements extends NativeElements {
      [name: string]: any
    }
    interface IntrinsicAttributes extends ReservedProps {
      class?: unknown
      style?: unknown
    }
  }
}
export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }
