import rules, { type Rules } from './rules'
import type { Linter } from 'eslint'

export const plugins = {
  'vue-jsx-vapor': {
    rules,
  },
}

export { rules, type Rules }

const config: (
  options: Linter.Config<Rules>,
) => Rules & Record<string, unknown> = ({ rules = {}, ...options } = {}) => ({
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
        reservedLast: ['v-slot', 'v-slots', 'v-text', 'v-html'],
      },
    ],
    'vue-jsx-vapor/define-style': rules['vue-jsx-vapor/define-style'] || 'warn',
  },
  ...options,
})

export default config
