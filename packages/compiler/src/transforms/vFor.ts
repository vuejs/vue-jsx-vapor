import {
  createCompilerError,
  ErrorCodes,
  isConstantNode,
  type SimpleExpressionNode,
} from '@vue/compiler-dom'
import { DynamicFlag, IRNodeTypes } from '../ir'
import {
  createStructuralDirectiveTransform,
  type NodeTransform,
  type TransformContext,
} from '../transform'
import {
  findProp,
  isJSXComponent,
  propToExpression,
  resolveExpression,
  resolveExpressionWithFn,
  resolveLocation,
} from '../utils'
import { createBranch } from './utils'
import type { JSXAttribute, JSXElement } from '@babel/types'

export const transformVFor: NodeTransform = createStructuralDirectiveTransform(
  'for',
  processFor,
)

export function processFor(
  node: JSXElement,
  dir: JSXAttribute,
  context: TransformContext,
) {
  const { value, index, key, source } = getForParseResult(dir, context)
  if (!source) {
    context.options.onError(
      createCompilerError(
        ErrorCodes.X_V_FOR_MALFORMED_EXPRESSION,
        resolveLocation(dir.loc, context),
      ),
    )
    return
  }

  const keyProp = findProp(node, 'key')
  const keyProperty = keyProp && propToExpression(keyProp, context)
  const isComponent = isJSXComponent(node)
  const id = context.reference()
  context.dynamic.flags |= DynamicFlag.NON_TEMPLATE | DynamicFlag.INSERT
  const [render, exitBlock] = createBranch(node, context, true)
  return (): void => {
    exitBlock()

    const { parent } = context

    // if v-for is the only child of a parent element, it can go the fast path
    // when the entire list is emptied
    const isOnlyChild =
      parent &&
      parent.block.node !== parent.node &&
      parent.node.children.length === 1

    context.dynamic.operation = {
      type: IRNodeTypes.FOR,
      id,
      source,
      value,
      key,
      index,
      keyProp: keyProperty,
      render,
      once: context.inVOnce || !!(source.ast && isConstantNode(source.ast, {})),
      component: isComponent,
      onlyChild: !!isOnlyChild,
    }
  }
}

export function getForParseResult(
  dir: JSXAttribute,
  context: TransformContext,
) {
  let value: SimpleExpressionNode | undefined,
    index: SimpleExpressionNode | undefined,
    key: SimpleExpressionNode | undefined,
    source: SimpleExpressionNode | undefined
  if (dir.value) {
    if (
      dir.value.type === 'JSXExpressionContainer' &&
      dir.value.expression.type === 'BinaryExpression'
    ) {
      if (dir.value.expression.left.type === 'SequenceExpression') {
        const expressions = dir.value.expression.left.expressions
        value =
          expressions[0] && resolveExpressionWithFn(expressions[0], context)
        key = expressions[1] && resolveExpression(expressions[1], context)
        index = expressions[2] && resolveExpression(expressions[2], context)
      } else {
        value = resolveExpressionWithFn(dir.value.expression.left, context)
      }
      source = resolveExpression(dir.value.expression.right, context)
    }
  } else {
    context.options.onError(
      createCompilerError(
        ErrorCodes.X_V_FOR_NO_EXPRESSION,
        resolveLocation(dir.loc, context),
      ),
    )
  }

  return {
    value,
    index,
    key,
    source,
  }
}
