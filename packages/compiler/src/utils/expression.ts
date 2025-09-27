import { parseExpression } from '@babel/parser'
import { isGloballyAllowed, makeMap } from '@vue/shared'
import type { TransformContext } from '../transform'
import { unwrapTSNode } from './ast'
import { getText, resolveJSXText } from './text'
import type {
  BigIntLiteral,
  JSXAttribute,
  Node,
  NumericLiteral,
  SourceLocation,
  StringLiteral,
} from '@babel/types'

export interface SimpleExpressionNode {
  content: string
  isStatic: boolean
  loc: SourceLocation | null | undefined
  ast?: Node | null | false
}

export const locStub: SourceLocation = {
  start: { line: 1, column: 0, index: 0 },
  end: { line: 1, column: 0, index: 0 },
  filename: '',
  identifierName: undefined,
}
export function createSimpleExpression(
  content: string,
  isStatic: boolean = false,
  loc: SourceLocation | null = locStub,
): SimpleExpressionNode {
  return {
    loc,
    content,
    isStatic,
  }
}

export const isLiteralWhitelisted: (key: string) => boolean =
  /*@__PURE__*/ makeMap('true,false,null,this')
export function isConstantExpression(exp: SimpleExpressionNode) {
  return (
    isLiteralWhitelisted(exp.content) ||
    isGloballyAllowed(exp.content) ||
    getLiteralExpressionValue(exp) !== null
  )
}

export function getLiteralExpressionValue(
  exp: SimpleExpressionNode,
): number | string | boolean | null {
  if (exp.ast) {
    if (
      ['StringLiteral', 'NumericLiteral', 'BigIntLiteral'].includes(
        exp.ast.type,
      )
    ) {
      return (exp.ast as StringLiteral | NumericLiteral | BigIntLiteral).value
    } else if (
      exp.ast.type === 'TemplateLiteral' &&
      exp.ast.expressions.length === 0
    ) {
      return exp.ast.quasis[0].value.cooked!
    }
  }
  return exp.isStatic ? exp.content : null
}

export const EMPTY_EXPRESSION = createSimpleExpression('', true)

export function resolveExpression(
  node: Node | undefined | null,
  context: TransformContext,
): SimpleExpressionNode {
  if (!node) return EMPTY_EXPRESSION
  node = unwrapTSNode(
    node.type === 'JSXExpressionContainer' ? node.expression : node,
  ) as Node
  const isStatic =
    node.type === 'StringLiteral' ||
    node.type === 'JSXText' ||
    node.type === 'JSXIdentifier'
  const source =
    node.type === 'JSXEmptyExpression'
      ? ''
      : node.type === 'JSXIdentifier'
        ? node.name
        : node.type === 'StringLiteral'
          ? node.value
          : node.type === 'JSXText'
            ? resolveJSXText(node)
            : node.type === 'Identifier'
              ? node.name
              : context.ir.source.slice(node.start!, node.end!)
  const location = node.loc
  return resolveSimpleExpression(source, isStatic, location, node)
}

export function resolveSimpleExpression(
  source: string,
  isStatic: boolean,
  location?: SourceLocation | null,
  ast?: false | Node | null,
) {
  const result = createSimpleExpression(source, isStatic, location)
  result.ast = ast ?? null
  return result
}

export function resolveExpressionWithFn(node: Node, context: TransformContext) {
  const text = getText(node, context)
  return node.type === 'Identifier'
    ? resolveSimpleExpression(text, false, node.loc)
    : resolveSimpleExpression(
        text,
        false,
        node.loc,
        parseExpression(`(${text})=>{}`, {
          plugins: context.options.expressionPlugins,
        }),
      )
}

export function propToExpression(
  prop: JSXAttribute,
  context: TransformContext,
) {
  return prop.type === 'JSXAttribute' &&
    prop.value?.type === 'JSXExpressionContainer'
    ? resolveExpression(prop.value.expression, context)
    : EMPTY_EXPRESSION
}
