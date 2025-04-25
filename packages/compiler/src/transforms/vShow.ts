import { createDOMCompilerError, DOMErrorCodes } from '@vue/compiler-dom'
import { IRNodeTypes } from '../ir'
import { resolveDirective } from '../utils'
import type { DirectiveTransform } from '../transform'

export const transformVShow: DirectiveTransform = (_dir, node, context) => {
  const dir = resolveDirective(_dir, context)
  const { exp, loc } = dir
  if (!exp) {
    context.options.onError(
      createDOMCompilerError(DOMErrorCodes.X_V_SHOW_NO_EXPRESSION, loc),
    )
    return
  }

  context.registerOperation({
    type: IRNodeTypes.DIRECTIVE,
    element: context.reference(),
    dir,
    name: 'show',
    builtin: true,
  })
}
