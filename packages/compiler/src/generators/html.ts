import { genCall, NEWLINE, type CodeFragment } from '../utils'
import type { CodegenContext } from '../generate'
import type { SetHtmlIRNode } from '../ir'
import { genExpression } from './expression'

export function genSetHtml(
  oper: SetHtmlIRNode,
  context: CodegenContext,
): CodeFragment[] {
  const { helper } = context
  const { value, element } = oper
  return [
    NEWLINE,
    ...genCall(helper('setHtml'), `n${element}`, genExpression(value, context)),
  ]
}
