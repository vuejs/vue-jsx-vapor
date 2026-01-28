import type { TransformOptions } from '.'
import type { Code } from 'ts-macro'

export function transformSlot(
  node: import('typescript').Node,
  options: TransformOptions,
) {
  const { ts, codes, ast } = options
  const slotNode = ts.isJsxElement(node)
    ? node.openingElement
    : ts.isJsxSelfClosingElement(node)
      ? node
      : null
  if (slotNode?.tagName.getText(ast) === 'slot') {
    let nameProp: Code[] = ['default']
    const props: Code[] = []
    let has_directive = false
    slotNode.attributes.forEachChild((node) => {
      if (ts.isJsxAttribute(node)) {
        const name = node.name.getText(ast)
        if (name.startsWith('v-')) {
          has_directive = true
          return
        }
        if (name === 'name' && node.initializer) {
          nameProp =
            ts.isJsxExpression(node.initializer) && node.initializer.expression
              ? [
                  '[',
                  [
                    node.initializer.expression.getText(ast),
                    node.initializer.expression.getStart(ast),
                  ],
                  ']',
                ]
              : [
                  [
                    node.initializer.getText(ast),
                    node.initializer.getStart(ast),
                  ],
                ]
        } else {
          const shouldResolve =
            name.includes('-') || ts.isJsxNamespacedName(node.name)
          props.push(
            shouldResolve ? '"' : '',
            [name, node.name.getStart(ast)],
            shouldResolve ? '"' : '',
            ': ',
            node.initializer
              ? ts.isJsxExpression(node.initializer) &&
                node.initializer.expression
                ? [
                    node.initializer.expression.getText(ast),
                    node.initializer.expression.getStart(ast),
                  ]
                : [
                    node.initializer.getText(ast),
                    node.initializer.getStart(ast),
                  ]
              : 'true',
            ', ',
          )
        }
      }
    })
    if (has_directive) {
      return false
    }
    codes.replaceRange(
      slotNode.attributes.pos,
      slotNode.attributes.end,
      ' {...(__slots[0] = { ',
      ...nameProp,
      ': { ',
      ...props,
      '}})}',
    )
    return true
  }
}
