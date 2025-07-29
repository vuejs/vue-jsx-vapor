//#region rolldown:runtime
var __create = Object.create
var __defProp = Object.defineProperty
var __getOwnPropDesc = Object.getOwnPropertyDescriptor
var __getOwnPropNames = Object.getOwnPropertyNames
var __getProtoOf = Object.getPrototypeOf
var __hasOwnProp = Object.prototype.hasOwnProperty
var __copyProps = (to, from, except, desc) => {
  if ((from && typeof from === 'object') || typeof from === 'function')
    for (
      // eslint-disable-next-line vars-on-top
      var keys = __getOwnPropNames(from), i = 0, n = keys.length, key;
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
var __toESM = (mod, isNodeMode, target) => (
  (target = mod != null ? __create(__getProtoOf(mod)) : {}),
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
  const { children, ['v-slots']: vSlots } = props
  delete props.children
  delete props['v-slots']
  if (arguments.length > 2) props.key = key
  return (0, vue_jsx_vapor.h)(type, props, vSlots || children)
}

exports.Fragment = vue.Fragment
exports.jsx = jsx
exports.jsxDEV = jsx
exports.jsxs = jsx
