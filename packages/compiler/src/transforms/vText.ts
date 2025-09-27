import { escapeHtml, isVoidTag } from '@vue/shared'
import { IRNodeTypes } from '../ir'
import {
  createCompilerError,
  EMPTY_EXPRESSION,
  ErrorCodes,
  getLiteralExpressionValue,
  getText,
  resolveExpression,
} from '../utils'
import type { DirectiveTransform } from '../transform'

export const transformVText: DirectiveTransform = (dir, node, context) => {
  let exp
  if (!dir.value) {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_TEXT_NO_EXPRESSION, dir.loc),
    )
    exp = EMPTY_EXPRESSION
  } else {
    exp = resolveExpression(dir.value, context)
  }
  if (node.children.length) {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_TEXT_WITH_CHILDREN, dir.loc as any),
    )
    context.childrenTemplate.length = 0
  }

  // v-text on void tags do nothing
  if (isVoidTag(getText(node.openingElement.name, context))) {
    return
  }

  const literal = getLiteralExpressionValue(exp)
  if (literal != null) {
    context.childrenTemplate = [escapeHtml(String(literal))]
  } else {
    context.childrenTemplate = [' ']
    context.registerOperation({
      type: IRNodeTypes.GET_TEXT_CHILD,
      parent: context.reference(),
    })
    context.registerEffect([exp], {
      type: IRNodeTypes.SET_TEXT,
      element: context.reference(),
      values: [exp],
      generated: true,
    })
  }
}
