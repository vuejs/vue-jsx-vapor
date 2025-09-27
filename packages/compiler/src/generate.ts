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
} from './utils/generate'
import type { BlockIRNode, RootIRNode } from './ir'
import type { SimpleExpressionNode } from './utils'
import type { ParserPlugin } from '@babel/parser'
import type { RawSourceMap } from 'source-map-js'

export type CodegenOptions = {
  /**
   * Generate source map?
   * @default false
   */
  sourceMap?: boolean
  /**
   * Filename for source map generation.
   * Also used for self-recursive reference in templates
   * @default 'index.jsx'
   */
  filename?: string
  /**
   * A list of parser plugins to enable for `@babel/parser`, which is used to
   * parse expressions in bindings and interpolations.
   * https://babeljs.io/docs/en/next/babel-parser#plugins
   */
  expressionPlugins?: ParserPlugin[]
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
      sourceMap: false,
      filename: `index.jsx`,
      expressionPlugins: ['jsx'],
      templates: [],
    }
    this.options = extend(defaultOptions, options)
    this.block = ir.block
  }
}

export interface VaporCodegenResult {
  ast: RootIRNode
  helpers: Set<string>
  templates: string[]
  delegates: Set<string>
  code: string
  map?: RawSourceMap
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
