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
  type __Props = Parameters<typeof __setup>[0]
  type __Slots = Parameters<typeof __setup>[1] extends { slots?: infer Slots } | undefined ? Slots : {}
  type __Exposed = Parameters<typeof __setup>[1] extends { expose?: (exposed: infer Exposed) => any } | undefined ? Exposed : {}`,
          '\n  const __component = ',
          [node.expression.getText(ast), node.expression.getStart(ast)],
          `({`,
          isVapor
            ? ''
            : `...{} as {
    setup: () => __Exposed,
    slots: import('vue').SlotsType<__Slots>
  },`,
          ...(compOptions
            ? ['...', [compOptions.getText(ast), compOptions.getStart(ast)]]
            : []),
          `})
  type __Instance = {${isVapor ? '\n/** @deprecated This is only a type when used in Vapor Instances. */' : ''}
    $props: __Props
  } & (typeof __component extends new (...args: any) => any ? InstanceType<typeof __component> : typeof __component)
  return {} as {
    new (props: __Props): __Instance,
    setup: (props: __Props, ctx?: {
      attrs?: Record<string, any>
      slots?: __Slots,
      expose?: (exposed: keyof __Exposed extends never ? __Instance : __Exposed) => any
    }) => {},
  }`,
        ] as Code[])
  codes.replaceRange(node.end, node.end, ...result, `\n})()`)
}
