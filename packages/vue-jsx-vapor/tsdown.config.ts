import { config } from '../../tsdown.config'

export default config({
  entry: ['./src/*.ts', '!./**.d.ts'],
})
