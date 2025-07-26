import { transformDefineStyle } from './define-style'
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

    if (!root?.body || (Object.keys(macros).length === 1 && macros.defineStyle))
      continue

    const asyncModifier = root.modifiers?.find(
      (modifier) => modifier.kind === ts.SyntaxKind.AsyncKeyword,
    )
    if (asyncModifier && macros.defineComponent)
      codes.replaceRange(asyncModifier.pos, asyncModifier.end)
    const result = `__ctx.render`

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

        const shouldWrapByCall =
          (ts.isArrowFunction(node.expression) ||
            ts.isFunctionExpression(node.expression)) &&
          macros.defineComponent
        codes.replaceRange(
          node.getStart(ast),
          node.expression.getStart(ast),
          `const __render = `,
          shouldWrapByCall ? '(' : '',
        )
        codes.replaceRange(
          node.expression.end,
          node.expression.end,
          shouldWrapByCall ? ')()' : '',
          `
return {} as {
  props: {${props.join(', ')}},
  context: {
    slots?: ${macros.defineSlots ?? '{}'},
    expose?: (exposed: import('vue').ShallowUnwrapRef<${macros.defineExpose ?? '{}'}>) => void,
    attrs?: Record<string, any>
  },
  render: typeof __render
}`,
        )
      }
    })
  }
}
