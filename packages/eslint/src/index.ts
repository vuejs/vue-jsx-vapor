import rules, { type Rules } from './rules'
import type { Linter } from 'eslint'

export const plugins = {
  'vue-jsx-vapor': {
    rules,
  },
}

export { rules, type Rules }

export default ({ rules = {}, ...options }: Linter.Config<Rules> = {}) => ({
  name: 'vue-jsx-vapor',
  plugins,
  rules: {
    'style/jsx-sort-props': 'off',
    'react/jsx-sort-props': 'off',
    'vue-jsx-vapor/jsx-sort-props': rules['vue-jsx-vapor/jsx-sort-props'] || [
      'warn',
      {
        callbacksLast: true,
        shorthandFirst: true,
        reservedFirst: [
          'v-if',
          'v-else-if',
          'v-else',
          'v-for',
          'key',
          'ref',
          'v-model',
        ],
        reservedLast: ['v-text', 'v-html', 'v-slots', 'v-slot'],
      },
    ],
    'vue-jsx-vapor/define-style': rules['vue-jsx-vapor/define-style'] || 'warn',
  } satisfies Rules & Record<string, unknown>,
  ...options,
})
