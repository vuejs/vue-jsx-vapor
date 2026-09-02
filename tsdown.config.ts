import process from 'node:process'
import { defineConfig, type UserConfig } from 'tsdown'
import Raw from 'unplugin-raw/rolldown'

export const config = (options: UserConfig = {}) =>
  defineConfig({
    entry: ['./src/*.ts', '!./**.d.ts'],
    clean: true,
    fixedExtension: false,
    watch: !!process.env.DEV,
    external: ['vue'],
    plugins: [Raw({ transform: true })],
    outputOptions: {
      exports: 'named',
    },
    ...options,
  })

export default config()
