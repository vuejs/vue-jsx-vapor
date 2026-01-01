import { getRequire } from './utils'
import type { OptionsResolved } from '../options'

let compileStyleAsync: any
async function getCompileStyleAsync() {
  if (compileStyleAsync) {
    return compileStyleAsync
  }
  const require = getRequire()
  try {
    return (compileStyleAsync = require
      ? require('vue/compiler-sfc').compileStyleAsync
      : // @ts-ignore support browser
        (await import('https://esm.sh/@vue/compiler-sfc')).compileStyleAsync)
  } catch {}
}

export async function transformStyle(
  code: string,
  id: string,
  options: OptionsResolved,
): Promise<string> {
  const query = new URLSearchParams(id.split('?')[1])
  const compileStyleAsync = await getCompileStyleAsync()
  // cannot detect version
  if (!compileStyleAsync) {
    console.warn(`Cannot require vue/compiler-sfc, please install Vue first.`)
    return ''
  }

  const result = await compileStyleAsync({
    filename: id,
    id: `data-v-${query.get('scopeId')}`,
    isProd: options.isProduction,
    source: code,
    scoped: query.get('scoped') === 'true',
  })

  return result.code
}
