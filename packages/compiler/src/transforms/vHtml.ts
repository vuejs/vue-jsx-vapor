import { IRNodeTypes } from '../ir'
import {
  createCompilerError,
  EMPTY_EXPRESSION,
  ErrorCodes,
  resolveExpression,
} from '../utils'
import type { DirectiveTransform } from '../transform'

export const transformVHtml: DirectiveTransform = (dir, node, context) => {
  let exp
  if (!dir.value) {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_HTML_NO_EXPRESSION, dir.loc),
    )
    exp = EMPTY_EXPRESSION
  } else {
    exp = resolveExpression(dir.value, context)
  }
  if (node.children.length) {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_HTML_WITH_CHILDREN, dir.loc),
    )
    context.childrenTemplate.length = 0
  }

  context.registerEffect([exp], {
    type: IRNodeTypes.SET_HTML,
    element: context.reference(),
    value: exp,
  })
}
