import { config } from '../../tsdown.config.js'

export default config({
  entry: ['./src/*.ts', '!./**.d.ts'],
})
