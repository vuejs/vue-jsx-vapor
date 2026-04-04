import { Fragment, type ReservedProps, type VNode, type VNodeChild } from 'vue'
import type { NativeElements } from 'vue-jsx-vapor'

declare function jsx(type: any, props: any, key: any): VNode
declare global {
  namespace JSX {
    type Element = VNodeChild | null
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
