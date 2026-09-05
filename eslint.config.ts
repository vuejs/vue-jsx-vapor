import { createRequire } from 'node:module'

import vueJsxVapor from './packages/eslint/src/index'
import { sxzz } from './packages/macros/eslint-config'

// eslint-plugin-perfectionist@5.9 currently calls this TypeScript API, which
// was removed in TypeScript 7. Keep the compatibility surface local to ESLint
// until the plugin supports the new TypeScript API.
const typescript = createRequire(import.meta.url)('typescript') as {
  isExternalModuleNameRelative?: (name: string) => boolean
}
typescript.isExternalModuleNameRelative ??= (name) =>
  name.startsWith('.') || name.startsWith('/') || /^[a-z]:[\\/]/i.test(name)

export default [
  {
    ignores: [
      '**/wasi-worker**',
      '**/compiler.wasi**',
      '**/tutorial/**/*.tsx',
      '**/docs/**',
    ],
  },
  ...(await sxzz()
    .removeRules(
      'unicorn/filename-case',
      'import/no-default-export',
      'unicorn/no-new-array',
      'unicorn/prefer-dom-node-remove',
      'unused-imports/no-unused-imports',
      'unicorn/no-anonymous-default-export',
      'unicorn/prefer-code-point',
      'unicorn/no-array-sort',
      '@eslint-community/eslint-comments/no-unlimited-disable',
      'vue/no-mutating-props',
      'vue/no-dupe-keys',
      '@typescript-eslint/no-namespace',
      'unused-imports/no-unused-vars',
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
        },
      },
    ])),
  vueJsxVapor({
    ignores: ['**/docs/**'],
  }),
]
