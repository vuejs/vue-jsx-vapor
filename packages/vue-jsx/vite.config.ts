import process from 'node:process'

import { defineConfig } from 'vite-plus'

export default defineConfig({
  pack: {
    entry: ['./src/*.ts', '!./**.d.ts'],
    clean: true,
    fixedExtension: false,
    watch: !!process.env.DEV,
    outputOptions: {
      exports: 'named',
    },
    deps: {
      neverBundle: ['vue'],
      onlyBundle: ['@vue-macros/volar', 'ts-macro', 'muggle-string'],
    },
  },
})
