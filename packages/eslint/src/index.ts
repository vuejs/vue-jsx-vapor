import rules, { type Rules } from './rules/index.ts'
import type { FlatConfig } from '@typescript-eslint/utils/ts-eslint'

export const plugins = {
  'vue-jsx': {
    rules,
  },
}

export { rules, type Rules }

const config: (options?: FlatConfig.Config) => FlatConfig.Config = ({
  rules = {},
  ...options
} = {}) => ({
  name: 'vue-jsx',
  plugins,
  rules: {
    'style/jsx-sort-props': 'off',
    'react/jsx-sort-props': 'off',
    'vue-jsx/jsx-sort-props': rules['vue-jsx/jsx-sort-props'] || [
      'warn',
      {
        callbacksLast: true,
        shorthandFirst: true,
        reservedFirst: ['v-if', 'v-else-if', 'v-else', 'v-for', 'key', 'ref', 'v-model'],
        reservedLast: ['v-slot', 'v-slots', 'v-text', 'v-html'],
      },
    ],
    'vue-jsx/define-style': rules['vue-jsx/define-style'] || 'warn',
  },
  ...options,
})

export default config
