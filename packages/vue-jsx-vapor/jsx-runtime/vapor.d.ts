import { Fragment, type Block, type ReservedProps, type VNode } from 'vue'
import type { NativeElements, NodeChild } from 'vue-jsx-vapor'

declare function jsx(type: any, props: any, key: any): Block
declare global {
  namespace JSX {
    type Element = NodeChild | null
    interface ElementClass {
      $props: {}
    }
    interface ElementAttributesProperty {
      $props: {}
    }
    interface ElementChildrenAttribute {
      'v-slots': {}
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
