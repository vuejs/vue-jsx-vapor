import process from 'node:process'
import { defineConfig, type Options } from 'tsdown'
import Raw from 'unplugin-raw/rolldown'

export const config = (options: Options = {}) =>
  defineConfig({
    entry: ['./src/*.ts', '!./**.d.ts'],
    clean: true,
    format: ['cjs', 'esm'],
    watch: !!process.env.DEV,
    dts: !process.env.DEV,
    external: ['vue'],
    define: {
      __DEV__: 'true',
    },
    plugins: [Raw()],
    outputOptions: {
      exports: 'named',
    },
    ...options,
  })

export default config()
