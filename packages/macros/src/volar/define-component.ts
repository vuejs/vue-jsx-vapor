import { allCodeFeatures, replaceRange } from 'ts-macro'
import type { TransformOptions } from '.'

export function transformDefineComponent(
  node: import('typescript').CallExpression,
  parent: import('typescript').Node,
  options: TransformOptions,
): void {
  const { codes, source, ast, ts } = options

  replaceRange(codes, node.arguments[0].end, node.end - 1)

  const componentOptions = node.arguments[1]
  replaceRange(
    codes,
    node.getStart(ast),
    node.expression.end + 1,
    ts.isExpressionStatement(parent) ? ';' : '',
    '(',
    [node.expression.getText(ast), source, node.getStart(ast), allCodeFeatures],
    '(() => ({}) as any, ',
    componentOptions
      ? [
          componentOptions.getText(ast),
          source,
          componentOptions.getStart(ast),
          allCodeFeatures,
        ]
      : '',
    '), ',
  )
}
