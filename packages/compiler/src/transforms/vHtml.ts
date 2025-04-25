import { createDOMCompilerError, DOMErrorCodes } from '@vue/compiler-dom'
import { IRNodeTypes } from '../ir'
import { resolveExpression, resolveLocation } from '../utils'
import type { DirectiveTransform } from '../transform'
import { EMPTY_EXPRESSION } from './utils'

export const transformVHtml: DirectiveTransform = (dir, node, context) => {
  let exp
  const loc = resolveLocation(dir.loc, context)
  if (!dir.value) {
    context.options.onError(
      createDOMCompilerError(DOMErrorCodes.X_V_HTML_NO_EXPRESSION, loc),
    )
    exp = EMPTY_EXPRESSION
  } else {
    exp = resolveExpression(dir.value, context)
  }
  if (node.children.length) {
    context.options.onError(
      createDOMCompilerError(DOMErrorCodes.X_V_HTML_WITH_CHILDREN, loc),
    )
    context.childrenTemplate.length = 0
  }

  context.registerEffect([exp], {
    type: IRNodeTypes.SET_HTML,
    element: context.reference(),
    value: exp,
  })
}
