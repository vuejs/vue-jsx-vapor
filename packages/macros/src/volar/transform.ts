import { HELPER_PREFIX } from '@vue-macros/common'
import { replaceRange } from 'ts-macro'
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
      transformDefineStyle(defaultStyle, defineStyleIndex++, options),
    )

    if (!root?.body) continue

    const asyncModifier = root.modifiers?.find(
      (modifier) => modifier.kind === ts.SyntaxKind.AsyncKeyword,
    )
    if (asyncModifier && macros.defineComponent)
      replaceRange(codes, asyncModifier.pos, asyncModifier.end)
    const result = `({}) as __VLS_PickNotAny<typeof ${HELPER_PREFIX}ctx.render, {}> & { __ctx: typeof ${HELPER_PREFIX}ctx }`

    const propsType = root.parameters[0]?.type
      ? String(root.parameters[0].type.getText(ast))
      : '{}'
    replaceRange(
      codes,
      root.parameters.pos,
      root.parameters.pos,
      ts.isArrowFunction(root) && root.parameters.pos === root.pos ? '(' : '',
      `${HELPER_PREFIX}props: typeof ${HELPER_PREFIX}ctx.props & ${propsType}, `,
      `${HELPER_PREFIX}placeholder?: {}, `,
      `${HELPER_PREFIX}ctx = {} as Awaited<ReturnType<typeof ${
        HELPER_PREFIX
      }setup>>, `,
      `${HELPER_PREFIX}setup = (${asyncModifier ? 'async' : ''}(`,
    )
    if (ts.isArrowFunction(root)) {
      replaceRange(
        codes,
        root.end,
        root.end,
        `))${root.pos === root.parameters.pos ? ')' : ''} => `,
        result,
      )
    } else {
      replaceRange(
        codes,
        root.body.getStart(ast),
        root.body.getStart(ast),
        '=>',
      )
      replaceRange(codes, root.end, root.end, `)){ return `, result, '}')
    }

    ts.forEachChild(root.body, (node) => {
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
            const isRequired = ts.forEachChild(
              element,
              function isNonNullExpression(node): boolean {
                return (
                  ts.isNonNullExpression(node) ||
                  !!ts.forEachChild(node, isNonNullExpression)
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
        replaceRange(
          codes,
          node.getStart(ast),
          node.expression.getStart(ast),
          `const ${HELPER_PREFIX}render = `,
          shouldWrapByCall ? '(' : '',
        )
        replaceRange(
          codes,
          node.expression.end,
          node.expression.end,
          shouldWrapByCall ? ')()' : '',
          `
return {
  props: {} as {${props.join(', ')}},
  slots: {} as ${macros.defineSlots ?? '{}'},
  expose: (exposed: import('vue').ShallowUnwrapRef<${macros.defineExpose ?? '{}'}>) => {},
  render: ${HELPER_PREFIX}render,
}`,
        )
      }
    })
  }
}
