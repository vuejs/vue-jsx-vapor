import { toValidAssetId } from '@vue/compiler-dom'
import type { CodegenContext } from '../generate'
import type { BlockIRNode } from '../ir'
import { genEffects, genOperations } from './operation'
import { genChildren, genSelf } from './template'
import {
  buildCodeFragment,
  DELIMITERS_ARRAY,
  genCall,
  genMulti,
  INDENT_END,
  INDENT_START,
  NEWLINE,
  type CodeFragment,
} from './utils'

export function genBlock(
  oper: BlockIRNode,
  context: CodegenContext,
  args: CodeFragment[] = [],
  root?: boolean,
): CodeFragment[] {
  return [
    '(',
    ...args,
    ') => {',
    INDENT_START,
    ...genBlockContent(oper, context, root),
    INDENT_END,
    NEWLINE,
    '}',
  ]
}

export function genBlockContent(
  block: BlockIRNode,
  context: CodegenContext,
  root?: boolean,
  genEffectsExtraFrag?: () => CodeFragment[],
): CodeFragment[] {
  const [frag, push] = buildCodeFragment()
  const { dynamic, effect, operation, returns } = block
  const resetBlock = context.enterBlock(block)

  if (root) {
    for (let name of context.ir.component) {
      const id = toValidAssetId(name, 'component')
      const maybeSelfReference = name.endsWith('__self')
      if (maybeSelfReference) name = name.slice(0, -6)
      push(
        NEWLINE,
        `const ${id} = `,
        ...genCall(
          context.helper('resolveComponent'),
          JSON.stringify(name),
          // pass additional `maybeSelfReference` flag
          maybeSelfReference ? 'true' : undefined,
        ),
      )
    }
    genResolveAssets('directive', 'resolveDirective')
  }

  for (const child of dynamic.children) {
    push(...genSelf(child, context))
  }
  for (const child of dynamic.children) {
    push(...genChildren(child, context, push, `n${child.id!}`))
  }

  push(...genOperations(operation, context))
  push(...genEffects(effect, context, genEffectsExtraFrag))

  push(NEWLINE, `return `)

  const returnNodes = returns.map((n) => `n${n}`)
  const returnsCode: CodeFragment[] =
    returnNodes.length > 1
      ? genMulti(DELIMITERS_ARRAY, ...returnNodes)
      : [returnNodes[0] || 'null']
  push(...returnsCode)

  resetBlock()
  return frag

  function genResolveAssets(kind: 'component' | 'directive', helper: string) {
    for (const name of context.ir[kind]) {
      push(
        NEWLINE,
        `const ${toValidAssetId(name, kind)} = `,
        ...genCall(context.helper(helper), JSON.stringify(name)),
      )
    }
  }
}
