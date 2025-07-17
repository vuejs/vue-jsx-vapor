import { allCodeFeatures, type Code } from 'ts-macro'
import type { DefineStyle, TransformOptions } from '.'

export function transformDefineStyle(
  { expression, isCssModules }: DefineStyle,
  index: number,
  options: TransformOptions,
): void {
  const { ts, codes, ast } = options
  if (
    isCssModules &&
    expression?.arguments[0] &&
    !expression.typeArguments &&
    ts.isTemplateLiteral(expression.arguments[0])
  ) {
    codes.replaceRange(
      expression.arguments.pos - 1,
      expression.arguments.pos - 1,
      `<{`,
      ...parseCssClassNames(
        expression.arguments[0].getText(ast).slice(1, -1),
      ).flatMap(
        ({ text, offset }) =>
          [
            `\n`,
            [
              `'${text.slice(1)}'`,
              `style_${index}`,
              expression.arguments.pos + offset + 1,
              { navigation: true },
            ],
            `: string`,
          ] as Code[],
      ),
      '\n}>',
    )
  }

  addEmbeddedCode(expression, index, options)
}

const commentReg = /(?<=\/\*)[\s\S]*?(?=\*\/)|(?<=\/\/)[\s\S]*?(?=\n)/g
const cssClassNameReg = /(?=(\.[a-z_][-\w]*)[\s.,+~>:#)[{])/gi
const fragmentReg = /(?<=\{)[^{]*(?=(?<!\\);)/g

function parseCssClassNames(css: string) {
  for (const reg of [commentReg, fragmentReg]) {
    css = css.replace(reg, (match) => ' '.repeat(match.length))
  }
  const matches = css.matchAll(cssClassNameReg)
  const result = []
  for (const match of matches) {
    const matchText = match[1]
    if (matchText) {
      result.push({ offset: match.index, text: matchText })
    }
  }
  return result
}

function addEmbeddedCode(
  expression: import('typescript').CallExpression,
  index: number,
  options: TransformOptions,
) {
  const { ts, ast } = options
  const languageId =
    ts.isPropertyAccessExpression(expression.expression) &&
    ts.isIdentifier(expression.expression.name)
      ? expression.expression.name.text
      : 'css'
  const style = expression.arguments[0]
  const styleText = style
    .getText(ast)
    .slice(1, -1)
    .replaceAll(/\$\{.*\}/g, (str) => '_'.repeat(str.length))
  options.embeddedCodes.push({
    id: `style_${index}`,
    languageId,
    snapshot: {
      getText: (start, end) => styleText.slice(start, end),
      getLength: () => styleText.length,
      getChangeRange: () => undefined,
    },
    mappings: [
      {
        sourceOffsets: [style.getStart(ast) + 1],
        generatedOffsets: [0],
        lengths: [styleText.length],
        data: allCodeFeatures,
      },
    ],
    embeddedCodes: [],
  })
}
