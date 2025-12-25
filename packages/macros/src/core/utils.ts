import type {
  ArrowFunctionExpression,
  FunctionDeclaration,
  FunctionExpression,
  Node,
} from '@babel/types'
import type { MagicStringAST } from '@vue-macros/common'

export type FunctionalNode =
  | FunctionDeclaration
  | FunctionExpression
  | ArrowFunctionExpression

export function prependFunctionalNode(
  node: FunctionalNode,
  s: MagicStringAST,
  result: string,
): void {
  const isBlockStatement = node.body.type === 'BlockStatement'
  const start = node.body.extra?.parenthesized
    ? (node.body.extra.parenStart as number)
    : node.body.start!
  s.appendRight(
    start + (isBlockStatement ? 1 : 0),
    `${result};${isBlockStatement ? '' : 'return '}`,
  )
  if (!isBlockStatement) {
    s.appendLeft(start, '{')
    s.appendRight(node.end!, '}')
  }
}

export function isFunctionalNode(node?: Node | null): node is FunctionalNode {
  return !!(
    node &&
    (node.type === 'ArrowFunctionExpression' ||
      node.type === 'FunctionDeclaration' ||
      node.type === 'FunctionExpression')
  )
}

export function getParamsStart(node: FunctionalNode, code: string): number {
  return node.params[0]
    ? node.params[0].start!
    : node.start! +
        (code.slice(node.start!, node.body.start!).match(/\(\s*\)/)?.index ||
          0) +
        1
}

export function getDefaultValue(node: Node): Node {
  if (node.type === 'TSNonNullExpression') {
    return getDefaultValue(node.expression)
  }
  if (node.type === 'TSAsExpression') {
    return getDefaultValue(node.expression)
  }
  return node
}
