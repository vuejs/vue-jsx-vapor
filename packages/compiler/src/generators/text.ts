import { getLiteralExpressionValue } from '../utils'
import type { CodegenContext } from '../generate'
import type {
  CreateNodesIRNode,
  GetTextChildIRNode,
  SetNodesIRNode,
  SetTextIRNode,
} from '../ir'
import { genExpression } from './expression'
import { genCall, NEWLINE, type CodeFragment } from './utils'
import type { SimpleExpressionNode } from '@vue/compiler-dom'

export function genSetText(
  oper: SetTextIRNode,
  context: CodegenContext,
): CodeFragment[] {
  const { helper } = context
  const { element, values, generated } = oper
  const texts = combineValues(values, context, true)
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
  setText?: boolean,
): CodeFragment[] {
  return values.flatMap((value, i) => {
    let exp = genExpression(value, context)
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
