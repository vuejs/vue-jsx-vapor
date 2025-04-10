import prettier from '@prettier/sync'
import type { MessageIds, RuleOptions } from './types'
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
        const offset = callee.loc.start.column
        const parser =
          node.callee.type === 'MemberExpression' &&
          node.callee.property.type === 'Identifier'
            ? node.callee.property.name
            : 'css'
        if (callee.type === 'Identifier' && callee.name === 'defineStyle') {
          const arg = node.arguments[0]

          if (arg?.type === 'TemplateLiteral') {
            const cssRaw = arg.quasis[0].value.raw

            let formattedCss = ''
            try {
              formattedCss = prettier.format(cssRaw, { parser, tabWidth })
            } catch {
              return context.report({
                node: arg,
                messageId: 'define-style-syntax-error',
              })
            }

            const placeholder = ' '.repeat(offset + tabWidth)
            const result = `\n${placeholder}${formattedCss.slice(0, -1).replaceAll('\n', `\n${placeholder}`)}\n${' '.repeat(offset)}`
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
