import type { TransformOptions } from '.'

export function transformDefineComponent(
  node: import('typescript').CallExpression,
  parent: import('typescript').Node,
  options: TransformOptions,
): void {
  const { codes, ast, ts } = options

  codes.replaceRange(node.arguments[0].end, node.end - 1)

  const componentOptions = node.arguments[1]
  codes.replaceRange(
    node.getStart(ast),
    node.expression.end + 1,
    ts.isExpressionStatement(parent) ? ';' : '',
    '(',
    [node.expression.getText(ast), node.getStart(ast)],
    '(() => ({}) as any,',
    componentOptions
      ? [componentOptions.getText(ast), componentOptions.getStart(ast)]
      : '',
    '), ',
  )
}
