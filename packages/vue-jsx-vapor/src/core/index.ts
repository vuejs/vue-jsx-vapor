import { transformSync } from '@babel/core'
// @ts-ignore missing type
import babelTypescript from '@babel/plugin-transform-typescript'
import jsx from '@vue-jsx-vapor/babel'
import type { Options } from '../options'

export type { Options }

export function transformVueJsxVapor(
  code: string,
  id: string,
  options?: Options,
  needSourceMap = false,
) {
  const params = new URLSearchParams(id)
  const vapor = params.get('vapor')
  return transformSync(code, {
    plugins: [
      [
        jsx,
        {
          compile: options?.compile,
          interop: vapor ? false : options?.interop,
        },
      ],
      ...(id.endsWith('.tsx')
        ? [[babelTypescript, { isTSX: true, allowExtensions: true }]]
        : []),
    ],
    filename: id,
    sourceMaps: needSourceMap,
    sourceFileName: id,
    babelrc: false,
    configFile: false,
    ast: true,
  })
}
