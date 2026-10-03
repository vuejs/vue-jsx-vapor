import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite-plus'

// The vapor-only helpers (`insert`, `isFragment`, ...) live on the vapor build, so the
// test env has to resolve `vue` to it. `vue` is a dependency of @vue-jsx/runtime — there
// is no `node_modules/vue` at the root — and Vite bundles this config into
// `node_modules/.vite-temp/`, so probe both bases instead of trusting `import.meta.url`.
const vueVapor = (() => {
  const specifier = 'vue/dist/vue.runtime-with-vapor.esm-browser.js'
  const require = createRequire(import.meta.url)
  const bases = [
    `${process.cwd()}/packages/runtime`,
    fileURLToPath(new URL('packages/runtime', import.meta.url)),
  ]
  for (const base of bases) {
    try {
      return require.resolve(specifier, { paths: [base] })
    } catch {}
  }
  throw new Error(`[vitest.config] unable to resolve ${specifier}`)
})()

export default defineConfig({
  resolve: {
    conditions: ['jsx-vapor-dev'],
    alias: { vue: vueVapor },
  },
  test: {
    include: ['./packages/**/*.spec.ts'],
  },
  define: {
    __DEV__: true,
    __TEST__: true,
    __VERSION__: '"test"',
    __GLOBAL__: false,
    __ESM_BUNDLER__: true,
    __ESM_BROWSER__: false,
    __CJS__: true,
    __SSR__: true,
    __FEATURE_OPTIONS_API__: true,
    __FEATURE_SUSPENSE__: true,
    __FEATURE_PROD_DEVTOOLS__: false,
    __FEATURE_PROD_HYDRATION_MISMATCH_DETAILS__: false,
    __COMPAT__: true,
  },
})
