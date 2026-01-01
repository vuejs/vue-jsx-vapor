import { walkAST } from 'ast-kit'
import hash from 'hash-sum'
import { helperPrefix } from './helper'
import { isFunctionalNode, type DefineStyle, type Macros } from '.'
import type { FunctionalNode } from './utils'
import type { Node } from '@babel/types'
import type MagicString from 'magic-string'

export function transformDefineStyle(
  defineStyle: DefineStyle,
  index: number,
  root: FunctionalNode | undefined,
  s: MagicString,
  importMap: Map<string, string>,
  { defineSlots }: Macros,
): void {
  const { expression, lang, isCssModules } = defineStyle
  if (expression.arguments[0]?.type !== 'TemplateLiteral') return

  let css = s
    .slice(expression.arguments[0].start!, expression.arguments[0].end!)
    .slice(1, -1)
  const scopeId = hash(css)
  const vars = new Map<string, string>()
  expression.arguments[0].expressions.forEach((exp) => {
    const cssVar = s.slice(exp.start!, exp.end!)
    const cssVarId = toCssVarId(cssVar, `--${scopeId}-`)
    s.overwrite(exp.start! - 2, exp.end! + 1, `var(${cssVarId})`)
    vars.set(cssVarId, cssVar)
  })

  let returnExpression = root && getReturnStatement(root)
  if (isFunctionalNode(returnExpression)) {
    returnExpression = getReturnStatement(returnExpression)
  }
  if (vars.size && returnExpression) {
    const children =
      returnExpression.type === 'JSXElement'
        ? [returnExpression]
        : returnExpression.type === 'JSXFragment'
          ? returnExpression.children
          : []
    const varString = Array.from(vars.entries())
      .map(([key, value]) => `'${key}': ${value}`)
      .join(', ')
    for (const child of children) {
      if (child.type === 'JSXElement') {
        s.appendRight(
          child.openingElement.name.end!,
          ` {...{style:{${varString}}}}`,
        )
      }
    }
  }

  let scoped = !!root
  if (expression.arguments[1]?.type === 'ObjectExpression') {
    for (const prop of expression.arguments[1].properties) {
      if (
        prop.type === 'ObjectProperty' &&
        prop.key.type === 'Identifier' &&
        prop.key.name === 'scoped' &&
        prop.value.type === 'BooleanLiteral'
      ) {
        scoped = prop.value.value
      }
    }
  }

  if (scoped && root) {
    const slotNames = defineSlots?.id
      ? defineSlots.id.type === 'Identifier'
        ? defineSlots.id.name
        : defineSlots.id.type === 'ObjectPattern'
          ? defineSlots.id.properties.map((prop) => {
              const value =
                prop.type === 'RestElement' ? prop.argument : prop.value
              return s.slice(value.start!, value.end!)
            })
          : []
      : []
    walkAST<Node>(root, {
      enter(node) {
        if (
          node.type === 'JSXElement' &&
          s.slice(
            node.openingElement.name.start!,
            node.openingElement.name.end!,
          ) !== 'template'
        ) {
          let subfix = ''
          if (slotNames.length) {
            const tagName =
              node.openingElement.name.type === 'JSXMemberExpression'
                ? node.openingElement.name.object
                : node.openingElement.name
            const name = s.slice(tagName.start!, tagName.end!)
            subfix = slotNames.includes(name) ? '-s' : ''
          }
          s.appendRight(
            node.openingElement.name.end!,
            ` data-v-${scopeId}${subfix}=""`,
          )
        }
      },
    })
  }

  css = s
    .slice(expression.arguments[0].start!, expression.arguments[0].end!)
    .slice(1, -1)
    .replaceAll(/\/\/(.*)(?=\n)/g, '/*$1*/')
  const module = isCssModules ? 'module.' : ''
  const importId = `${helperPrefix}/define-style/${index}?scopeId=${scopeId}&scoped=${scoped}&lang.${module}${lang}`
  importMap.set(importId, css)
  s.appendLeft(
    0,
    isCssModules
      ? `import style${index} from "${importId}";`
      : `import "${importId}";`,
  )
  s.overwrite(
    expression.start!,
    expression.end!,
    isCssModules ? `style${index}` : '',
  )
}

function getReturnStatement(root: FunctionalNode) {
  if (root.body.type === 'BlockStatement') {
    const returnStatement = root.body.body.find(
      (node) => node.type === 'ReturnStatement',
    )
    if (returnStatement) {
      return returnStatement.argument
    }
  } else {
    return root.body
  }
}

function toCssVarId(name: string, prefix = '') {
  return (
    prefix +
    name.replaceAll(/\W/g, (searchValue, replaceValue) => {
      return searchValue === '.'
        ? '-'
        : name.charCodeAt(replaceValue).toString()
    })
  )
}
