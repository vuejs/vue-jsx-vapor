import { Fragment, type Block } from 'vue'
import { h, type NativeElements, type ReservedProps } from 'vue-jsx-vapor'

function jsx(type: any, props: any, key: any): ReturnType<typeof h> {
  const { children, ['v-slots']: vSlots } = props
  delete props.children
  delete props['v-slots']
  if (arguments.length > 2) {
    props.key = key
  }
  return h(type, props, vSlots || children)
}

export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }

declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    // @ts-ignore
    export type Element = Block
    export interface ElementClass {
      $props: {}
    }
    export interface ElementAttributesProperty {
      $props: {}
    }
    export interface IntrinsicElements extends NativeElements {
      // allow arbitrary elements
      // @ts-ignore suppress ts:2374 = Duplicate string index signature.
      [name: string]: any
    }
    export interface IntrinsicAttributes extends ReservedProps {
      class?: unknown
      style?: unknown
    }
  }
}
