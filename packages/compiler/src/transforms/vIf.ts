import { DynamicFlag, IRNodeTypes } from '../ir'
import {
  createStructuralDirectiveTransform,
  type NodeTransform,
  type TransformContext,
} from '../transform'
import {
  createBranch,
  createCompilerError,
  createSimpleExpression,
  ErrorCodes,
  isConstantNode,
  isEmptyText,
  resolveDirective,
} from '../utils'
import type { JSXAttribute, JSXElement } from '@babel/types'

export const transformVIf: NodeTransform = createStructuralDirectiveTransform(
  ['if', 'else', 'else-if'],
  processIf,
)

export const transformedIfNode = new WeakMap()

export function processIf(
  node: JSXElement,
  attribute: JSXAttribute,
  context: TransformContext,
): (() => void) | undefined {
  const dir = resolveDirective(attribute, context)
  if (dir.name !== 'else' && (!dir.exp || !dir.exp.content.trim())) {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_IF_NO_EXPRESSION, dir.loc),
    )
    const loc = dir.exp ? dir.exp.loc : node.loc
    dir.exp = createSimpleExpression(`true`, false, loc)
  }

  context.dynamic.flags |= DynamicFlag.NON_TEMPLATE
  transformedIfNode.set(node, dir)
  if (dir.name === 'if') {
    const id = context.reference()
    context.dynamic.flags |= DynamicFlag.INSERT
    const [branch, onExit] = createBranch(node, context)

    return () => {
      onExit()
      context.dynamic.operation = {
        type: IRNodeTypes.IF,
        id,
        condition: dir.exp!,
        positive: branch,
        once: context.inVOnce || isConstantNode(attribute.value!),
      }
    }
  } else {
    // check the adjacent v-if
    const siblingIf = getSiblingIf(context as TransformContext<JSXElement>)

    const siblings = context.parent && context.parent.dynamic.children
    let lastIfNode
    if (siblings) {
      let i = siblings.length
      while (i--) {
        if (
          siblings[i].operation &&
          siblings[i].operation!.type === IRNodeTypes.IF
        ) {
          lastIfNode = siblings[i].operation
          break
        }
      }
    }

    if (
      // check if v-if is the sibling node
      !siblingIf ||
      // check if IfNode is the last operation and get the root IfNode
      !lastIfNode ||
      lastIfNode.type !== IRNodeTypes.IF
    ) {
      context.options.onError(
        createCompilerError(ErrorCodes.X_V_ELSE_NO_ADJACENT_IF, node.loc),
      )
      return
    }

    while (lastIfNode.negative && lastIfNode.negative.type === IRNodeTypes.IF) {
      lastIfNode = lastIfNode.negative
    }

    // Check if v-else was followed by v-else-if
    if (dir.name === 'else-if' && lastIfNode.negative) {
      context.options.onError(
        createCompilerError(ErrorCodes.X_V_ELSE_NO_ADJACENT_IF, node.loc),
      )
    }

    const [branch, onExit] = createBranch(node, context)

    if (dir.name === 'else') {
      lastIfNode.negative = branch
    } else {
      lastIfNode.negative = {
        type: IRNodeTypes.IF,
        id: -1,
        condition: dir.exp!,
        positive: branch,
        once: context.inVOnce,
      }
    }

    return () => onExit()
  }
}

export function getSiblingIf(context: TransformContext<JSXElement>) {
  const parent = context.parent
  if (!parent) return

  const siblings = parent.node.children
  let sibling
  let i = siblings.indexOf(context.node)
  while (--i >= 0) {
    if (!isEmptyText(siblings[i])) {
      sibling = siblings[i]
      break
    }
  }

  if (
    sibling &&
    sibling.type === 'JSXElement' &&
    transformedIfNode.has(sibling)
  ) {
    return sibling
  }
}
