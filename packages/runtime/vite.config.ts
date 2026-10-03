import process from 'node:process'

import Raw from 'unplugin-raw/rolldown'

import { defineConfig } from 'vite-plus'

export default defineConfig({
  pack: {
    entry: ['./src/*.ts', '!./**.d.ts'],
    clean: true,
    fixedExtension: false,
    watch: !!process.env.DEV,
    deps: {
      neverBundle: ['vue'],
    },
    plugins: [Raw({ transform: true }) as any],
    outputOptions: {
      exports: 'named',
    },
  },
})
