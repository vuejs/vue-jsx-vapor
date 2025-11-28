import { sxzz } from '@sxzz/eslint-config'
import vueJsxVapor from './packages/eslint/src/index'

export default [
  ...(await sxzz()
    .removeRules(
      'unicorn/filename-case',
      'import/no-default-export',
      'unicorn/no-new-array',
      'unicorn/prefer-dom-node-remove',
      'unused-imports/no-unused-imports',
      '@eslint-community/eslint-comments/no-unlimited-disable',
    )
    .append([
      {
        name: 'docs',
        files: ['**/*.md/*.tsx'],
        rules: {
          'no-var': 'off',
          'no-mutable-exports': 'off',
          'no-duplicate-imports': 'off',
          'import/first': 'off',
          'unused-imports/no-unused-vars': 'off',
        },
      },
    ])),
  vueJsxVapor({
    ignores: ['**/docs/**'],
  }),
]
