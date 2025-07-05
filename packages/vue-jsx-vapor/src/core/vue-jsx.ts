import { transformSync } from '@babel/core'
// @ts-ignore missing type
import babelTypescript from '@babel/plugin-transform-typescript'
import jsx from '@vue/babel-plugin-jsx'

export function transformVueJsx(
  code: string,
  id: string,
  needSourceMap = false,
) {
  const result = transformSync(code, {
    plugins: [
      jsx,
      ...(id.endsWith('.tsx')
        ? [[babelTypescript, { isTSX: true, allowExtensions: true }]]
        : []),
    ],
    filename: id,
    sourceMaps: needSourceMap,
    sourceFileName: id,
    babelrc: false,
    configFile: false,
  })
  if (result && result.code !== code) {
    return {
      code: result.code,
      map: result.map,
    }
  }
}
