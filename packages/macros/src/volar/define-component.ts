import type { TransformOptions } from '.'
import type { Code } from 'ts-macro'

export function transformDefineComponent(
  node: import('typescript').CallExpression,
  parent: import('typescript').Node,
  options: TransformOptions,
): void {
  const { codes, ast, ts } = options

  const [comp, compOptions] = node.arguments
  const isVapor = node.expression.getText(ast) === 'defineVaporComponent'

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
      ? ['\nreturn __setup']
      : ([
          `
  type __Setup = typeof __setup
  type __Props = Parameters<__Setup>[0]
  type __Slots = Parameters<__Setup>[1] extends { slots?: infer Slots } | undefined ? Slots : {}
  type __Exposed = Parameters<__Setup>[1] extends { expose?: (exposed: infer Exposed) => any } | undefined ? Exposed : {}`,
          '\n  const __component = ',
          isVapor
            ? `// @ts-ignore\n(defineVaporComponent,await import('vue-jsx-vapor')).`
            : '',
          [node.expression.getText(ast), node.expression.getStart(ast)],
          `({\n`,
          `...{} as {
    setup: (props: __Props) => __Exposed,
    render: () => ReturnType<__Setup>
    slots: ${isVapor ? '__Slots' : `import('vue').SlotsType<__Slots>`} 
  },`,
          ...(compOptions
            ? ['...', [compOptions.getText(ast), compOptions.getStart(ast)]]
            : []),
          `})
  return {} as Omit<typeof __component, 'constructor'> & {
    new (props?: __Props): InstanceType<typeof __component> & {${isVapor ? '\n/** @deprecated This is only a type when used in Vapor Instances. */\n$props: __Props' : ''}},
  }`,
        ] as Code[])
  codes.replaceRange(node.end, node.end, ...result, `\n})()`)
}
