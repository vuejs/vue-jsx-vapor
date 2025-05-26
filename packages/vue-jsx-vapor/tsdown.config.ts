import { defineConfig } from 'tsdown'
import { config } from '../../tsdown.config.js'

export default [
  config({
    entry: ['./src/*.ts', '!./**.d.ts', '!./src/jsx-runtime.ts'],
  }),
  defineConfig({
    entry: { index: 'src/jsx-runtime.ts' },
    outDir: './jsx-runtime',
    format: ['esm', 'cjs'],
  }),
]
