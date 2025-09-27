import { isGloballyAllowed, isHTMLTag, isString, isSVGTag } from '@vue/shared'
import { IRNodeTypes, type RootNode } from '../ir'
import type { SimpleExpressionNode } from './expression'
import type {
  Expression,
  JSXElement,
  JSXFragment,
  Node,
  ObjectProperty,
} from '@babel/types'

export function findProp(
  expression: Expression | undefined,
  key: string | RegExp,
) {
  if (expression?.type === 'JSXElement') {
    for (const attr of expression.openingElement.attributes) {
      const name =
        attr.type === 'JSXAttribute' &&
        (attr.name.type === 'JSXIdentifier'
          ? attr.name.name
          : attr.name.type === 'JSXNamespacedName'
            ? attr.name.namespace.name
            : ''
        ).split('_')[0]
      if (name && (isString(key) ? name === key : key.test(name))) {
        return attr
      }
    }
  }
}

export const TS_NODE_TYPES: string[] = [
  'TSAsExpression', // foo as number
  'TSTypeAssertion', // (<number>foo)
  'TSNonNullExpression', // foo!
  'TSInstantiationExpression', // foo<string>
  'TSSatisfiesExpression', // foo satisfies T
]

export function unwrapTSNode(node: Node): Node {
  if (TS_NODE_TYPES.includes(node.type)) {
    return unwrapTSNode((node as any).expression)
  } else {
    return node
  }
}

export function isJSXComponent(node: Node) {
  if (node.type !== 'JSXElement') return false

  const { openingElement } = node
  if (openingElement.name.type === 'JSXIdentifier') {
    const name = openingElement.name.name
    return !isHTMLTag(name) && !isSVGTag(name)
  } else {
    return openingElement.name.type === 'JSXMemberExpression'
  }
}

export function isMemberExpression(exp: SimpleExpressionNode) {
  if (!exp.ast) return
  const ret = unwrapTSNode(exp.ast) as Expression
  return (
    ret.type === 'MemberExpression' ||
    ret.type === 'OptionalMemberExpression' ||
    (ret.type === 'Identifier' && ret.name !== 'undefined')
  )
}

export function isJSXElement(
  node?: Node | null,
): node is JSXElement | JSXFragment {
  return !!node && (node.type === 'JSXElement' || node.type === 'JSXFragment')
}

export function isTemplate(node: Node) {
  if (
    node.type === 'JSXElement' &&
    node.openingElement.name.type === 'JSXIdentifier'
  ) {
    return node.openingElement.name.name === 'template'
  }
}

export function isStaticNode(node: Node): boolean {
  node = unwrapTSNode(node)

  switch (node.type) {
    case 'UnaryExpression': // void 0, !true
      return isStaticNode(node.argument)

    case 'LogicalExpression': // 1 > 2
    case 'BinaryExpression': // 1 + 2
      return isStaticNode(node.left) && isStaticNode(node.right)

    case 'ConditionalExpression': {
      // 1 ? 2 : 3
      return (
        isStaticNode(node.test) &&
        isStaticNode(node.consequent) &&
        isStaticNode(node.alternate)
      )
    }

    case 'SequenceExpression': // (1, 2)
    case 'TemplateLiteral': // `foo${1}`
      return node.expressions.every((expr) => isStaticNode(expr))

    case 'ParenthesizedExpression': // (1)
      return isStaticNode(node.expression)

    case 'StringLiteral':
    case 'NumericLiteral':
    case 'BooleanLiteral':
    case 'NullLiteral':
    case 'BigIntLiteral':
      return true
  }
  return false
}

export function isConstantNode(node: Node): boolean {
  if (isStaticNode(node)) return true

  node = unwrapTSNode(node)
  switch (node.type) {
    case 'Identifier':
      return node.name === 'undefined' || isGloballyAllowed(node.name)
    case 'RegExpLiteral':
      return true
    case 'ObjectExpression':
      return node.properties.every((prop) => {
        // { bar() {} } object methods are not considered static nodes
        if (prop.type === 'ObjectMethod') return false
        // { ...{ foo: 1 } }
        if (prop.type === 'SpreadElement') return isConstantNode(prop.argument)
        // { foo: 1 }
        return (
          (!prop.computed || isConstantNode(prop.key)) &&
          isConstantNode(prop.value)
        )
      })
    case 'ArrayExpression':
      return node.elements.every((element) => {
        // [1, , 3]
        if (element === null) return true
        // [1, ...[2, 3]]
        if (element.type === 'SpreadElement')
          return isConstantNode(element.argument)
        // [1, 2]
        return isConstantNode(element)
      })
  }
  return false
}

export const isFnExpression: (exp: SimpleExpressionNode) => boolean = (exp) => {
  try {
    if (!exp.ast) return false
    let ret = exp.ast
    // parser may parse the exp as statements when it contains semicolons
    if (ret.type === 'Program') {
      ret = ret.body[0]
      if (ret.type === 'ExpressionStatement') {
        ret = ret.expression
      }
    }
    ret = unwrapTSNode(ret) as Expression
    return (
      ret.type === 'FunctionExpression' ||
      ret.type === 'ArrowFunctionExpression'
    )
  } catch {
    return false
  }
}

export const isFragmentNode = (
  node: Node | RootNode,
): node is JSXElement | JSXFragment | RootNode =>
  node.type === IRNodeTypes.ROOT ||
  node.type === 'JSXFragment' ||
  (node.type === 'JSXElement' && !!isTemplate(node))

export const isStaticProperty = (node?: Node): node is ObjectProperty =>
  !!node &&
  (node.type === 'ObjectProperty' || node.type === 'ObjectMethod') &&
  !node.computed

const nonIdentifierRE = /^$|^\d|[^$\w\u00A0-\uFFFF]/
export const isSimpleIdentifier = (name: string): boolean =>
  !nonIdentifierRE.test(name)
