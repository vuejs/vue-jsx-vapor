import { config } from '../../tsdown.config.ts'

export default config({
  external: ['@babel/parser'],
  inlineOnly: ['ast-kit', '@babel/types', 'pathe'],
})
