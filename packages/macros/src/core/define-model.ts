import { useModelHelperId } from './helper'
import { importHelperFn } from './utils'
import type { CallExpression } from '@babel/types'
import type MagicString from 'magic-string'

export function transformDefineModel(
  node: CallExpression,
  propsName: string,
  s: MagicString,
): void {
  s.overwrite(
    node.callee.start!,
    node.callee.end!,
    importHelperFn(s, 'useModel', undefined, useModelHelperId),
  )
  s.appendRight(
    node.arguments[0]?.start || node.end! - 1,
    `${propsName}, ${
      node.arguments[0]?.type === 'StringLiteral' ? '' : `'modelValue',`
    }`,
  )
}
