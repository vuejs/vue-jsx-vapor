import process from 'node:process'

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
    outputOptions: {
      exports: 'named',
    },
  },
})
