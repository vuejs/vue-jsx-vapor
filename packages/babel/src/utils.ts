import type { NodePath } from '@babel/traverse'
import type { JSXElement, JSXFragment, Node } from '@babel/types'

export function isConditionalExpression(path: NodePath<Node> | null): boolean {
  return !!(
    path &&
    (path?.type === 'LogicalExpression' ||
      path.type === 'ConditionalExpression') &&
    (path.parent.type === 'JSXExpressionContainer' ||
      isConditionalExpression(path.parentPath))
  )
}

export function isJSXElement(
  node?: Node | null,
): node is JSXElement | JSXFragment {
  return !!node && (node.type === 'JSXElement' || node.type === 'JSXFragment')
}
