import { useSlotsHelperId } from './helper'
import { importHelperFn } from './utils'
import type { CallExpression } from '@babel/types'
import type MagicString from 'magic-string'

export function transformDefineSlots(
  node: CallExpression,
  s: MagicString,
): void {
  s.overwrite(
    node.callee.start!,
    node.callee.end!,
    importHelperFn(s, 'useSlots', undefined, useSlotsHelperId),
  )
}
