import type { DirectiveNode } from '../ir'
import type { TransformContext } from '../transform'
import {
  createSimpleExpression,
  resolveExpression,
  resolveExpressionWithFn,
  resolveSimpleExpression,
} from './expression'
import { getText } from './text'
import type { JSXAttribute } from '@babel/types'

const namespaceRE = /^(?:\$([\w-]+)\$)?([\w-]+)?/
export function resolveDirective(
  node: JSXAttribute,
  context: TransformContext,
  withFn = false,
): DirectiveNode {
  const { value, name } = node
  let nameString =
    name.type === 'JSXNamespacedName'
      ? name.namespace.name
      : name.type === 'JSXIdentifier'
        ? name.name
        : ''
  const isDirective = nameString.startsWith('v-')
  let modifiers: string[] = []
  let isStatic = true
  let argString = name.type === 'JSXNamespacedName' ? name.name.name : ''
  if (name.type !== 'JSXNamespacedName' && !argString) {
    ;[nameString, ...modifiers] = nameString.split('_')
  } else {
    const result = argString.match(namespaceRE)
    if (result) {
      let modifierString = ''
      ;[, argString, modifierString] = result
      if (argString) {
        argString = argString.replaceAll('_', '.')
        isStatic = false
        if (modifierString && modifierString.startsWith('_'))
          modifiers = modifierString.slice(1).split('_')
      } else if (modifierString) {
        ;[argString, ...modifiers] = modifierString.split('_')
      }
    }
  }

  const arg = isDirective
    ? argString && name.type === 'JSXNamespacedName'
      ? resolveSimpleExpression(argString, isStatic, name.name.loc)
      : undefined
    : resolveSimpleExpression(nameString, true, name.loc)

  const exp = value
    ? withFn && value.type === 'JSXExpressionContainer'
      ? resolveExpressionWithFn(value.expression, context)
      : resolveExpression(value, context)
    : undefined

  return {
    name: isDirective ? nameString.slice(2) : 'bind',
    rawName: getText(name, context),
    exp,
    arg,
    loc: node.loc!,
    modifiers: modifiers.map((modifier) => createSimpleExpression(modifier)),
  }
}
