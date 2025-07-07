import { parse } from '@babel/parser'
import { generate, transform, type CompilerOptions } from '../../src'
import { IRNodeTypes, type RootNode } from '../../src/ir'
import type { JSXElement, JSXFragment } from '@babel/types'

export function makeCompile(options: CompilerOptions = {}) {
  return (source: string, overrideOptions: CompilerOptions = {}) => {
    const {
      body: [statement],
    } = parse(source, {
      sourceType: 'module',
      plugins: ['jsx', 'typescript'],
    }).program
    let children!: JSXElement[] | JSXFragment['children']
    let tagType
    if (statement.type === 'ExpressionStatement') {
      tagType = statement.expression.type
      children =
        statement.expression.type === 'JSXFragment'
          ? statement.expression.children
          : statement.expression.type === 'JSXElement'
            ? [statement.expression]
            : []
    }
    const ast: RootNode = {
      type: tagType === 'JSXFragment' ? 'JSXFragment' : IRNodeTypes.ROOT,
      children,
      source,
    }
    const ir = transform(ast, {
      expressionPlugins: ['typescript', 'jsx'],
      inline: true,
      prefixIdentifiers: false,
      ...options,
      ...overrideOptions,
    }) as any
    const { code, helpers, preamble } = generate(ir, {
      inline: true,
      prefixIdentifiers: false,
      ...options,
      ...overrideOptions,
    })
    return { ast, ir, code, helpers, preamble }
  }
}
