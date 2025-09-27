import {
  genCall,
  getLiteralExpressionValue,
  isConstantNode,
  NEWLINE,
  type CodeFragment,
  type SimpleExpressionNode,
} from '../utils'
import type { CodegenContext } from '../generate'
import type {
  CreateNodesIRNode,
  GetTextChildIRNode,
  SetNodesIRNode,
  SetTextIRNode,
} from '../ir'
import { genExpression } from './expression'

export function genSetText(
  oper: SetTextIRNode,
  context: CodegenContext,
): CodeFragment[] {
  const { helper } = context
  const { element, values, generated } = oper
  const texts = combineValues(values, context, true, true)
  return [
    NEWLINE,
    ...genCall(helper('setText'), `${generated ? 'x' : 'n'}${element}`, texts),
  ]
}

export function genGetTextChild(
  oper: GetTextChildIRNode,
  context: CodegenContext,
): CodeFragment[] {
  return [
    NEWLINE,
    `const x${oper.parent} = ${context.helper('child')}(n${oper.parent})`,
  ]
}

export function genSetNodes(
  oper: SetNodesIRNode,
  context: CodegenContext,
): CodeFragment[] {
  const { helper } = context
  const { element, values, generated, once } = oper
  return [
    NEWLINE,
    ...genCall(
      helper('setNodes'),
      `${generated ? 'x' : 'n'}${element}`,
      combineValues(values, context, once),
    ),
  ]
}

export function genCreateNodes(
  oper: CreateNodesIRNode,
  context: CodegenContext,
): CodeFragment[] {
  const { helper } = context
  const { id, values, once } = oper
  return [
    NEWLINE,
    `const n${id} = `,
    ...genCall(
      helper('createNodes'),
      values && combineValues(values, context, once),
    ),
  ]
}

function combineValues(
  values: SimpleExpressionNode[],
  context: CodegenContext,
  once: boolean,
  setText?: boolean,
): CodeFragment[] {
  return values.flatMap((value, i) => {
    const { content, isStatic, ast } = value
    let exp = genExpression(
      value,
      context,
      undefined,
      !once &&
        !setText &&
        !!content &&
        !isStatic &&
        !!ast &&
        !isConstantNode(ast),
    )
    if (setText && getLiteralExpressionValue(value) == null) {
      // dynamic, wrap with toDisplayString
      exp = genCall(context.helper('toDisplayString'), exp)
    }
    if (i > 0) {
      exp.unshift(setText ? ' + ' : ', ')
    }
    return exp
  })
}
