import { createDOMCompilerError, DOMErrorCodes } from '@vue/compiler-dom'
import { escapeHtml, isVoidTag } from '@vue/shared'
import { IRNodeTypes } from '../ir'
import {
  getLiteralExpressionValue,
  getText,
  resolveExpression,
  resolveLocation,
} from '../utils'
import type { DirectiveTransform } from '../transform'
import { EMPTY_EXPRESSION } from './utils'

export const transformVText: DirectiveTransform = (dir, node, context) => {
  let exp
  const loc = resolveLocation(dir.loc, context)
  if (!dir.value) {
    context.options.onError(
      createDOMCompilerError(DOMErrorCodes.X_V_TEXT_NO_EXPRESSION, loc),
    )
    exp = EMPTY_EXPRESSION
  } else {
    exp = resolveExpression(dir.value, context)
  }
  if (node.children.length) {
    context.options.onError(
      createDOMCompilerError(DOMErrorCodes.X_V_TEXT_WITH_CHILDREN, loc),
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
