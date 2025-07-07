import {
  genCall,
  genExpression,
  NEWLINE,
  type CodeFragment,
  type CodegenContext,
} from '@vue/compiler-vapor'
import {
  IRNodeTypes,
  type CreateNodesIRNode,
  type OperationNode,
  type SetNodesIRNode,
} from './ir'
import type { SimpleExpressionNode } from '@vue/compiler-dom'

export const customGenOperation = (
  oper: OperationNode,
  context: CodegenContext,
) => {
  if (oper.type === IRNodeTypes.CREATE_NODES) {
    return genCreateNodes(oper, context)
  } else if (oper.type === IRNodeTypes.SET_NODES) {
    return genSetNodes(oper, context)
  }
}

export function genSetNodes(
  oper: SetNodesIRNode,
  context: CodegenContext,
): CodeFragment[] {
  const { helper } = context
  const { element, values, generated } = oper
  return [
    NEWLINE,
    ...genCall(
      helper('setNodes'),
      `${generated ? 'x' : 'n'}${element}`,
      combineValues(values, context),
    ),
  ]
}

export function genCreateNodes(
  oper: CreateNodesIRNode,
  context: CodegenContext,
): CodeFragment[] {
  const { helper } = context
  const { id, values } = oper
  return [
    NEWLINE,
    `const n${id} = `,
    ...genCall(helper('createNodes'), values && combineValues(values, context)),
  ]
}

function combineValues(
  values: SimpleExpressionNode[],
  context: CodegenContext,
): CodeFragment[] {
  return values.flatMap((value, i) => {
    const exp = genExpression(value, context)
    if (i > 0) {
      exp.unshift(', ')
    }
    return exp
  })
}
