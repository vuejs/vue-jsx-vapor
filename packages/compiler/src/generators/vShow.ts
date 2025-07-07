import type { CodegenContext } from '../generate'
import type { DirectiveIRNode } from '../ir'
import { genExpression } from './expression'
import { genCall, NEWLINE, type CodeFragment } from './utils'

export function genVShow(
  oper: DirectiveIRNode,
  context: CodegenContext,
): CodeFragment[] {
  return [
    NEWLINE,
    ...genCall(context.helper('applyVShow'), `n${oper.element}`, [
      `() => (`,
      ...genExpression(oper.dir.exp!, context),
      `)`,
    ]),
  ]
}
