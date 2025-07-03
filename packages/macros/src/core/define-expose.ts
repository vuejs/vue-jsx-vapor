import { importHelperFn, type MagicStringAST } from '@vue-macros/common'
import type { CallExpression } from '@babel/types'

export function transformDefineExpose(
  node: CallExpression,
  s: MagicStringAST,
): void {
  s.overwriteNode(node.callee, ';')
  s.appendRight(
    node.arguments[0]?.start || node.end! - 1,
    `${importHelperFn(s, 0, 'getCurrentInstance')}(true).exposed = `,
  )
}
