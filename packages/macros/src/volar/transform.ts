import { transformDefineStyle } from './define-style'
import { transformSlot } from './slot'
import type { RootMap, TransformOptions } from '.'

export function transformJsxMacros(
  rootMap: RootMap,
  options: TransformOptions,
): void {
  const { ts, codes, ast } = options

  let defineStyleIndex = 0
  for (const [root, macros] of rootMap) {
    macros.defineStyle?.forEach((defaultStyle) =>
      transformDefineStyle(defaultStyle, defineStyleIndex++, root, options),
    )

    if (
      !root?.body ||
      ts.isExpression(root.body) ||
      (Object.keys(macros).length === 1 && macros.defineStyle)
    )
      continue

    const asyncModifier = root.modifiers?.find(
      (modifier) => modifier.kind === ts.SyntaxKind.AsyncKeyword,
    )
    if (asyncModifier && macros.defineComponent)
      codes.replaceRange(asyncModifier.pos, asyncModifier.end)
    const result =
      '({}) as typeof __ctx.render & { __ctx?: { props: typeof __ctx.props } & typeof __ctx.context }'

    const propsType = root.parameters[0]?.type
      ? root.parameters[0].type.getText(ast)
      : '{}'
    codes.replaceRange(
      root.parameters.pos,
      root.parameters.pos,
      ts.isArrowFunction(root) && root.parameters.pos === root.pos ? '(' : '',
      `__props: typeof __ctx.props & ${propsType}, `,
      `__context?: typeof __ctx.context, `,
      `__ctx = {} as Awaited<ReturnType<typeof __fn>>, `,
      `__fn = (${asyncModifier ? 'async' : ''}(`,
    )
    if (ts.isArrowFunction(root)) {
      codes.replaceRange(
        root.end,
        root.end,
        `))${root.pos === root.parameters.pos ? ')' : ''} => `,
        result,
      )
    } else {
      codes.replaceRange(root.body.getStart(ast), root.body.getStart(ast), '=>')
      codes.replaceRange(root.end, root.end, `)){ return `, result, '}')
    }

    if (macros.slots && !macros.defineSlots) {
      macros.defineSlots = '__ResolveSlots<typeof __slots>'
      const start = root.body.getStart(ast)
      codes.replaceRange(start + 1, start + 1, 'let __slots;')
      macros.slots.map((node) => transformSlot(node, options))
    }

    root.body.forEachChild((node) => {
      if (ts.isReturnStatement(node) && node.expression) {
        const props = [...(macros.defineModel ?? [])]
        const elements =
          root.parameters[0] &&
          !root.parameters[0].type &&
          ts.isObjectBindingPattern(root.parameters[0].name)
            ? root.parameters[0].name.elements
            : []
        for (const element of elements) {
          if (ts.isIdentifier(element.name)) {
            const isRequired = element.forEachChild(
              function isNonNullExpression(node): boolean {
                return (
                  ts.isNonNullExpression(node) ||
                  !!node.forEachChild(isNonNullExpression)
                )
              },
            )
            props.push(
              `${element.name.escapedText}${
                isRequired ? ':' : '?:'
              } typeof ${element.name.escapedText}`,
            )
          }
        }

        const isDefineComponent =
          macros.defineComponent &&
          ['defineComponent', 'defineCustomElement'].includes(
            macros.defineComponent.expression.getText(ast),
          )
        codes.replaceRange(
          node.getStart(ast),
          node.expression.getStart(ast),
          'const ',
          [`__rndr`, node.getStart(ast), { verification: true }],
          isDefineComponent
            ? macros.slots
              ? ' = ('
              : ': () => JSX.Element = '
            : ' = ',
        )
        codes.replaceRange(
          node.expression.end,
          node.expression.end,
          isDefineComponent && macros.slots ? ')()' : '',
          `
return {} as {
  props: {${props.join(', ')}},
  context: ${
    root.parameters[1]?.type ? `${root.parameters[1].type.getText(ast)} & ` : ''
  }{
    slots: ${macros.defineSlots ?? '{}'},
    expose: (exposed: import('vue').ShallowUnwrapRef<${macros.defineExpose ?? 'Record<string, any>'}>) => void,
    attrs: Record<string, any>
  },
  render: ${isDefineComponent ? `ReturnType<` : ''}typeof __rndr${isDefineComponent ? '>' : ''}
}`,
        )
      }
    })
  }
}
