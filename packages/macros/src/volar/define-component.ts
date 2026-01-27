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

  const isFnComponent =
    ((ts.isArrowFunction(comp) || ts.isFunctionExpression(comp)) &&
      comp.typeParameters?.length) ||
    node.expression.getText(ast).includes('defineVapor')

  codes.replaceRange(
    node.getStart(ast),
    node.expression.end,
    ts.isExpressionStatement(parent) ? ';' : '',
    isFnComponent
      ? ''
      : `(() => {
const __setup = `,
  )

  if (isFnComponent) {
    codes.push(
      '\n;[',
      [node.expression.getText(ast), node.expression.getStart(ast)],
      ']',
    )
    return
  }

  codes.replaceRange(
    node.end,
    node.end,
    `
  type __Setup = typeof __setup
  type __Props = Parameters<__Setup>[0]
  type __Slots = Parameters<__Setup>[1] extends { slots?: infer Slots } | undefined ? Slots : {}
  type __Exposed = Parameters<__Setup>[1] extends { expose?: (exposed: infer Exposed) => any } | undefined ? Exposed : {}`,
    '\n  return ',
    [node.expression.getText(ast), node.expression.getStart(ast)],
    `({\n  ...{} as {
    setup: (props: __Props) => __Exposed,
    render: () => ReturnType<__Setup>
    slots: import('vue').SlotsType<__Slots>
  },`,
    ...((compOptions
      ? ['...', [compOptions.getText(ast), compOptions.getStart(ast)]]
      : []) as Code[]),
    `})`,
    `\n})()`,
  )
}
