import type { TransformOptions } from '.'
import type { Code } from 'ts-macro'

export function transformDefineComponent(
  node: import('typescript').CallExpression,
  parent: import('typescript').Node,
  options: TransformOptions,
): void {
  const { codes, ast, ts } = options

  const [comp, compOptions] = node.arguments

  codes.replaceRange(comp.end, node.end - 1)

  codes.replaceRange(
    node.getStart(ast),
    node.expression.end,
    ts.isExpressionStatement(parent) ? ';' : '',
    `(() => {
const __setup = `,
  )

  const result =
    (ts.isArrowFunction(comp) || ts.isFunctionExpression(comp)) &&
    comp.typeParameters?.length
      ? ['__setup']
      : ([
          [node.expression.getText(ast), node.expression.getStart(ast)],
          `({
    __typeProps: {} as Parameters<typeof __setup>[0],
    ...{} as Parameters<typeof __setup>[1] extends { slots?: infer S, expose?: infer E } | undefined ? {
      setup: E extends (exposed: infer Exposed) => any ? () => Exposed : never,
      slots: S extends Record<string, any> ? import('vue').SlotsType<S> : never
    } : {},`,
          ...(compOptions
            ? [
                '\n    ...',
                [compOptions.getText(ast), compOptions.getStart(ast)],
              ]
            : []),
          `
  })`,
        ] as Code[])
  codes.replaceRange(node.end, node.end, '\n  return ', ...result, `\n})()`)
}
