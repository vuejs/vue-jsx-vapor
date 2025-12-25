//#region rolldown:runtime
const __create = Object.create
const __defProp = Object.defineProperty
const __getOwnPropDesc = Object.getOwnPropertyDescriptor
const __getOwnPropNames = Object.getOwnPropertyNames
const __getProtoOf = Object.getPrototypeOf
const __hasOwnProp = Object.prototype.hasOwnProperty
const __copyProps = (to, from, except, desc) => {
  if ((from && typeof from === 'object') || typeof from === 'function')
    for (
      let keys = __getOwnPropNames(from), i = 0, n = keys.length, key;
      i < n;
      i++
    ) {
      key = keys[i]
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, {
          get: ((k) => from[k]).bind(null, key),
          enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable,
        })
    }
  return to
}
const __toESM = (mod, isNodeMode, target) => (
  (target = mod == null ? {} : __create(__getProtoOf(mod))),
  __copyProps(
    isNodeMode || !mod || !mod.__esModule
      ? __defProp(target, 'default', {
          value: mod,
          enumerable: true,
        })
      : target,
    mod,
  )
)
//#endregion
const vue = __toESM(require('vue'))
const vue_jsx_vapor = __toESM(require('vue-jsx-vapor'))

function jsx(type, props, key) {
  const { children, 'v-slots': vSlots } = props
  delete props.children
  delete props['v-slots']
  if (arguments.length > 2) props.key = key
  return (0, vue_jsx_vapor.h)(type, props, vSlots || children)
}

exports.Fragment = vue.Fragment
exports.jsx = jsx
exports.jsxDEV = jsx
exports.jsxs = jsx
