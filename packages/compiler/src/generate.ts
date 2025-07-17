import { extend, remove } from '@vue/shared'
import { genBlockContent } from './generators/block'
import { genTemplates } from './generators/template'
import { setTemplateRefIdent } from './generators/templateRef'
import {
  buildCodeFragment,
  codeFragmentToString,
  INDENT_END,
  INDENT_START,
  NEWLINE,
} from './generators/utils'
import type { BlockIRNode, RootIRNode } from './ir'
import type {
  CodegenOptions as BaseCodegenOptions,
  BaseCodegenResult,
  SimpleExpressionNode,
} from '@vue/compiler-dom'

export type CodegenOptions = Omit<
  BaseCodegenOptions,
  'optimizeImports' | 'inline' | 'bindingMetadata' | 'prefixIdentifiers'
> & {
  templates?: string[]
}

export class CodegenContext {
  options: Required<CodegenOptions>

  helpers: Set<string> = new Set<string>([])

  helper = (name: string) => {
    this.helpers.add(name)
    return `_${name}`
  }

  delegates: Set<string> = new Set<string>()

  identifiers: Record<string, (string | SimpleExpressionNode)[]> =
    Object.create(null)

  block: BlockIRNode
  withId<T>(
    fn: () => T,
    map: Record<string, string | SimpleExpressionNode | null>,
  ): T {
    const { identifiers } = this
    const ids = Object.keys(map)

    for (const id of ids) {
      identifiers[id] ||= []
      identifiers[id].unshift(map[id] || id)
    }

    const ret = fn()
    ids.forEach((id) => remove(identifiers[id], map[id] || id))

    return ret
  }

  enterBlock(block: BlockIRNode) {
    const parent = this.block
    this.block = block
    return (): BlockIRNode => (this.block = parent)
  }

  scopeLevel: number = 0
  enterScope(): [level: number, exit: () => number] {
    return [this.scopeLevel++, () => this.scopeLevel--] as const
  }

  constructor(
    public ir: RootIRNode,
    options: CodegenOptions,
  ) {
    const defaultOptions: Required<CodegenOptions> = {
      mode: 'module',
      sourceMap: false,
      filename: `template.vue.html`,
      scopeId: null,
      runtimeGlobalName: `Vue`,
      runtimeModuleName: `vue`,
      ssrRuntimeModuleName: 'vue/server-renderer',
      ssr: false,
      isTS: false,
      inSSR: false,
      templates: [],
      expressionPlugins: [],
    }
    this.options = extend(defaultOptions, options)
    this.block = ir.block
  }
}

export interface VaporCodegenResult
  extends Omit<BaseCodegenResult, 'preamble'> {
  ast: RootIRNode
  helpers: Set<string>
  templates: string[]
  delegates: Set<string>
}

// IR -> JS codegen
export function generate(
  ir: RootIRNode,
  options: CodegenOptions = {},
): VaporCodegenResult {
  const [frag, push] = buildCodeFragment()
  const context = new CodegenContext(ir, options)
  const { helpers } = context

  push(INDENT_START)
  if (ir.hasTemplateRef) {
    push(
      NEWLINE,
      `const ${setTemplateRefIdent} = ${context.helper('createTemplateRefSetter')}()`,
    )
  }
  push(...genBlockContent(ir.block, context, true))
  push(INDENT_END, NEWLINE)

  if (context.delegates.size) {
    context.helper('delegateEvents')
  }
  const templates = genTemplates(ir.templates, ir.rootTemplateIndex, context)

  const [code, map] = codeFragmentToString(frag, context)

  return {
    code,
    ast: ir,
    map: map && map.toJSON(),
    helpers,
    templates,
    delegates: context.delegates,
  }
}
