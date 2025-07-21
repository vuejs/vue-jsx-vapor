import prettier from '@prettier/sync'
import type { MessageIds, RuleOptions } from './types'
import type { TSESTree } from '@typescript-eslint/utils'
import type { RuleModule } from '@typescript-eslint/utils/ts-eslint'

const rule: RuleModule<MessageIds, RuleOptions> = {
  defaultOptions: [
    {
      tabWidth: 2,
    },
  ],
  meta: {
    type: 'layout',
    docs: {
      description: 'Enforce consistent formatting in defineStyle CSS',
    },
    fixable: 'code',
    messages: {
      'define-style': 'Style in defineStyle should be properly formatted',
      'define-style-syntax-error': 'Syntax error in defineStyle',
    },
    schema: [
      {
        type: 'object',
        properties: {
          tabWidth: {
            type: 'number',
            default: 2,
          },
        },
      },
    ],
  },
  create(context) {
    const configuration = context.options[0] || {}
    const tabWidth = configuration.tabWidth || 2
    return {
      CallExpression(node) {
        const callee =
          node.callee.type === 'MemberExpression'
            ? node.callee.object
            : node.callee
        const parser =
          node.callee.type === 'MemberExpression' &&
          node.callee.property.type === 'Identifier'
            ? node.callee.property.name
            : 'css'
        if (callee.type === 'Identifier' && callee.name === 'defineStyle') {
          const arg = node.arguments[0]

          if (arg?.type === 'TemplateLiteral') {
            let index = 0
            const cssRaw = context.sourceCode.text.slice(
              arg.range[0] + 1,
              arg.range[1] - 1,
            )
            //   .replaceAll('${', '--')
            // arg.quasis[0].value.raw.replaceAll('${', '-')

            let formattedCss = ''
            try {
              formattedCss = prettier
                .format(
                  arg.quasis
                    .map(
                      (i) =>
                        i.value.raw +
                        (arg.expressions[index]
                          ? `-MACROS_START-${context.sourceCode.text.slice(
                              ...arg.expressions[index++].range,
                            )}-MACROS_END-`
                          : ''),
                    )
                    .join(''),
                  { parser, tabWidth },
                )
                .replaceAll('-MACROS_START-', '${')
                .replaceAll('-MACROS_END-', '}')
            } catch {
              return context.report({
                node: arg,
                messageId: 'define-style-syntax-error',
              })
            }

            const line = callee.loc.start.line
            function getOffset(node: TSESTree.Node) {
              if (node.parent?.loc.start.line === line) {
                return getOffset(node.parent)
              }
              return node.loc.start.column
            }
            const column = getOffset(callee)
            const placeholder = ' '.repeat(column + tabWidth)
            const result = `\n${placeholder}${formattedCss
              .slice(0, -1)
              .replaceAll('\n', `\n${placeholder}`)}\n${' '.repeat(column)}`
            if (result !== cssRaw) {
              context.report({
                node: arg,
                messageId: 'define-style',
                fix(fixer) {
                  return fixer.replaceTextRange(
                    [arg.range[0] + 1, arg.range[1] - 1],
                    result,
                  )
                },
              })
            }
          }
        }
      },
    }
  },
}
export default rule
