import type { Fragment, VNode } from 'vue'
export type { JSX } from 'vue-jsx-vapor'

declare global {
  export type { JSX } from 'vue-jsx-vapor'
}

declare function jsx(type: any, props: any, key: any): VNode

export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }
