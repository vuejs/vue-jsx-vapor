import { parse } from '@babel/parser'
import { compile } from '@vue-jsx-vapor/compiler'
import { walkAST } from 'ast-kit'
import { SourceMapConsumer } from 'source-map-js'
import { isJSXElement } from './utils'
import type { Options } from '.'
import type { VisitNodeFunction } from '@babel/traverse'
import type { JSXElement, JSXFragment, Node } from '@babel/types'

export const transformJSX: VisitNodeFunction<
  Options,
  JSXElement | JSXFragment
> = (path, state) => {
  if (isJSXElement(path.parent)) return

  const root = state.roots.shift()
  if (!root || !root.inVaporComponent) return

  const isTS = state.filename?.endsWith('tsx')
  const { code, map, helpers, templates, delegates } = compile(root.node, {
    isTS,
    filename: state.filename,
    sourceMap: !!state.file.opts.sourceMaps,
    source: ' '.repeat(root.node.start || 0) + root.source,
    templates: state.templates.slice(),
    ...state.opts.compile,
  })

  helpers.forEach((helper) => state.importSet.add(helper))
  delegates.forEach((delegate) => state.delegateEventSet.add(delegate))
  state.templates.push(...templates.slice(state.templates.length))

  const ast = parse(`(() => {${code}})()`, {
    sourceFilename: state.filename,
    plugins: isTS ? ['jsx', 'typescript'] : ['jsx'],
  })

  if (map) {
    const consumer = new SourceMapConsumer(map)
    walkAST<Node>(ast, {
      enter(id) {
        if (
          (id.type === 'Identifier' || id.type === 'JSXIdentifier') &&
          id.loc
        ) {
          const originalLoc = consumer.originalPositionFor(id.loc.start)
          if (originalLoc.column) {
            id.loc.start.line = originalLoc.line
            id.loc.start.column = originalLoc.column
            id.loc.end.line = originalLoc.line
            id.loc.end.column = originalLoc.column + id.name.length
          }
        }
      },
    })
  }

  path.replaceWith(ast.program.body[0])
}
