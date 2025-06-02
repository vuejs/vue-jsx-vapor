import { Fragment, h, type VNode } from 'vue'
import type { NativeElements, ReservedProps } from 'vue-jsx-vapor'

function jsx(type: any, props: any, key: any): ReturnType<typeof h> {
  const { children } = props
  delete props.children
  if (arguments.length > 2) {
    props.key = key
  }
  return h(type, props, children)
}

export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }

declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    export interface Element extends VNode {}
    export interface ElementClass {
      $props: {}
    }
    export interface ElementAttributesProperty {
      $props: {}
    }
    export interface IntrinsicElements extends NativeElements {
      [name: string]: any
    }
    export interface IntrinsicAttributes extends ReservedProps {}
  }
}
