import jsxDirective from '@vue-macros/volar/jsx-directive'
import jsxRef from '@vue-macros/volar/jsx-ref'
import jsxMacros, { jsxElement } from './src/volar'

export default {
  exclude: ['**/slots.tsx'],
  plugins: [jsxDirective(), jsxRef(), jsxMacros(), jsxElement()],
}
