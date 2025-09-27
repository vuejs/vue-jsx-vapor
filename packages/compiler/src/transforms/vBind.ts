import { camelize, extend } from '@vue/shared'
import { createSimpleExpression, resolveExpression } from '../utils'
import type { DirectiveTransform } from '../transform'
import { isReservedProp } from './transformElement'

export const transformVBind: DirectiveTransform = (dir, node, context) => {
  const { name, value, loc } = dir
  if (!loc || name.type === 'JSXNamespacedName') return

  const [nameString, ...modifiers] = name.name.split('_')

  const exp = resolveExpression(value, context)
  let arg = createSimpleExpression(nameString, true, dir.name)

  if (arg.isStatic && isReservedProp(arg.content)) return

  let camel = false
  if (modifiers.includes('camel')) {
    if (arg.isStatic) {
      arg = extend({}, arg, { content: camelize(arg.content) })
    } else {
      camel = true
    }
  }

  return {
    key: arg,
    value: exp,
    loc,
    runtimeCamelize: camel,
    modifier: modifiers.includes('prop')
      ? '.'
      : modifiers.includes('attr')
        ? '^'
        : undefined,
  }
}
