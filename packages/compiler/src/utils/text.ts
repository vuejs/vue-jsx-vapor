import type { TransformContext } from '../transform'
import type { JSXText, Node } from '@babel/types'

const EMPTY_TEXT_REGEX =
  /^[\t\v\f \u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF]*[\n\r]\s*$/
const START_EMPTY_TEXT_REGEX = /^\s*[\n\r]/
const END_EMPTY_TEXT_REGEX = /[\n\r]\s*$/
export function resolveJSXText(node: JSXText) {
  if (EMPTY_TEXT_REGEX.test(String(node.extra?.raw))) {
    return ''
  }
  let value = node.value
  if (START_EMPTY_TEXT_REGEX.test(value)) {
    value = value.trimStart()
  }
  if (END_EMPTY_TEXT_REGEX.test(value)) {
    value = value.trimEnd()
  }
  return value
}

export function isEmptyText(node: Node) {
  return (
    (node.type === 'JSXText' &&
      EMPTY_TEXT_REGEX.test(String(node.extra?.raw))) ||
    (node.type === 'JSXExpressionContainer' &&
      node.expression.type === 'JSXEmptyExpression')
  )
}

export function getText(node: Node, content: TransformContext) {
  return content.ir.source.slice(node.start!, node.end!)
}
