import { isHTMLTag, isSVGTag } from '@vue/shared'
import { createPlugin } from 'ts-macro'

export default createPlugin(({ ts }) => {
  return {
    name: '@vue-jsx-vapor/jsx-element',
    resolveVirtualCode({ ast, codes }) {
      let transformed = false
      ast.forEachChild(function walk(
        node,
        parent: import('typescript').Node = ast,
      ) {
        if (
          !ts.isJsxElement(parent) &&
          !ts.isJsxFragment(parent) &&
          !ts.isJsxExpression(parent) &&
          !(parent.parent ? isConditionalExpression(ts, parent) : false)
        ) {
          const openingElement = ts.isJsxElement(node)
            ? node.openingElement
            : ts.isJsxSelfClosingElement(node)
              ? node
              : null
          if (openingElement) {
            const tagName = openingElement.tagName.getText(ast)
            if (!tagName.includes('-')) {
              transformed = true
              codes.replaceRange(
                node.getStart(ast),
                node.getStart(ast) + 1,
                '(<',
              )
              codes.replaceRange(
                node.end,
                node.end,
                ' as unknown as __InferJsxElement<',
                isHTMLTag(tagName) || isSVGTag(tagName)
                  ? `'${tagName}'`
                  : `typeof ${tagName}`,
                '>)',
              )
            }
          }
        }

        if (
          !ts.isCallExpression(node) ||
          node.expression.getText(ast) !== 'defineSlots'
        ) {
          node.forEachChild((child) => walk(child, node))
        }
      })

      transformed &&
        codes.push(`
type __InferJsxElement<T> = T extends keyof HTMLElementTagNameMap
  ? HTMLElementTagNameMap[T]
  : T extends keyof SVGElementTagNameMap
    ? SVGElementTagNameMap[T]
    : T extends (...args: any[]) => any
      ? ReturnType<T>
      : T extends { new (...args: any[]): infer Instance }
        ? Instance extends { $: any }
          ? import('vue').VNode
          : Instance
        : JSX.Element
      `)
    },
  }
})

function isConditionalExpression(
  ts: typeof import('typescript'),
  node: import('typescript').Node | null,
): boolean {
  return !!(
    node &&
    (ts.isBinaryExpression(node) || ts.isConditionalExpression(node)) &&
    node.parent &&
    (ts.isJsxExpression(node.parent) ||
      isConditionalExpression(ts, node.parent))
  )
}
