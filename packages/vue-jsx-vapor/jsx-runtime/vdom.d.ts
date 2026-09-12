import type { Fragment, VNode } from 'vue'
import type { JSX as VueJSX } from 'vue-jsx-vapor'

declare global {
  namespace JSX {
    type Element = VueJSX.Element
    type ElementChildrenAttribute = VueJSX.ElementChildrenAttribute
    type IntrinsicElements = VueJSX.IntrinsicElements
    type IntrinsicAttributes = VueJSX.IntrinsicAttributes
    type LibraryManagedAttributes<Component, Props> =
      VueJSX.LibraryManagedAttributes<Component, Props>
  }
}

declare function jsx(type: any, props: any, key: any): VNode

export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }
