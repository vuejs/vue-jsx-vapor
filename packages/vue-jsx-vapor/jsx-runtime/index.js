import { Fragment } from 'vue'
import { h } from 'vue-jsx-vapor'

function jsx(type, props, key) {
  const { children, 'v-slots': vSlots } = props
  delete props.children
  delete props['v-slots']
  if (arguments.length > 2) props.key = key
  return h(type, props, vSlots || children)
}

export { Fragment, jsx, jsx as jsxDEV, jsx as jsxs }
