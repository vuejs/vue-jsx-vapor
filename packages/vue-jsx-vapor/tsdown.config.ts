import { config } from '../../tsdown.config.ts'

export default config({
  entry: ['./src/*.ts', '!./**.d.ts'],
})
