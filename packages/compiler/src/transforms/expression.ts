import { DynamicFlag, IRNodeTypes, type OperationNode } from '../ir'
import { transformNode, type TransformContext } from '../transform'
import { createBranch, isConstantNode, resolveExpression } from '../utils'
import type { ConditionalExpression, LogicalExpression } from '@babel/types'

export function processConditionalExpression(
  node: ConditionalExpression,
  context: TransformContext,
) {
  const { test, consequent, alternate } = node

  context.dynamic.flags |= DynamicFlag.NON_TEMPLATE | DynamicFlag.INSERT
  const id = context.reference()
  const condition = resolveExpression(test, context)
  const [branch, onExit] = createBranch(consequent, context)
  const operation: OperationNode = {
    type: IRNodeTypes.IF,
    id,
    condition,
    positive: branch,
    once: context.inVOnce || isConstantNode(test),
  }

  return [
    () => {
      onExit()
      context.dynamic.operation = operation
    },
    () => {
      const [branch, onExit] = createBranch(alternate, context)
      operation.negative = branch
      transformNode(context)
      onExit()
    },
  ]
}

export function processLogicalExpression(
  node: LogicalExpression,
  context: TransformContext,
) {
  const { left, right, operator } = node

  context.dynamic.flags |= DynamicFlag.NON_TEMPLATE
  context.dynamic.flags |= DynamicFlag.INSERT

  const id = context.reference()
  const condition = resolveExpression(left, context)
  const [branch, onExit] = createBranch(
    operator === '&&' ? right : left,
    context,
  )
  const operation: OperationNode = {
    type: IRNodeTypes.IF,
    id,
    condition,
    positive: branch,
    once: context.inVOnce,
  }

  return [
    () => {
      onExit()
      context.dynamic.operation = operation
    },
    () => {
      const [branch, onExit] = createBranch(
        operator === '&&' ? left : right,
        context,
      )
      operation.negative = branch
      transformNode(context)
      onExit()
    },
  ]
}
