import { IRSlotType } from '../ir'
import { isJSXComponent, resolveExpression } from '../utils'
import type { DirectiveTransform } from '../transform'

export const transformVSlots: DirectiveTransform = (dir, node, context) => {
  if (!isJSXComponent(node)) return

  if (dir.value?.type === 'JSXExpressionContainer') {
    context.slots = [
      {
        slotType: IRSlotType.EXPRESSION,
        slots: resolveExpression(dir.value.expression, context),
      },
    ]
  }
}
