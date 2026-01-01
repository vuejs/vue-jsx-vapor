import { babelParse, getLang, walkAST } from 'ast-kit'
import MagicString from 'magic-string'
import { transformDefineComponent } from './define-component'
import { transformDefineExpose } from './define-expose'
import { transformDefineModel } from './define-model'
import { transformDefineSlots } from './define-slots'
import { transformDefineStyle } from './define-style'
import {
  getParamsStart,
  HELPER_PREFIX,
  isFunctionalNode,
  type FunctionalNode,
} from './utils'
import type { OptionsResolved } from '../options'
import type {
  CallExpression,
  LVal,
  Node,
  Program,
  VoidPattern,
} from '@babel/types'

interface CodeTransform {
  code: string
  map: any
}

export { isFunctionalNode }

export { restructure } from './restructure'

export type DefineStyle = {
  expression: CallExpression
  isCssModules: boolean
  lang: string
}

export type Macros = {
  defineComponent?: CallExpression
  defineModel?: {
    expression: CallExpression
    isRequired: boolean
  }[]
  defineSlots?: {
    expression: CallExpression
    id?: VoidPattern | LVal
  }
  defineExpose?: CallExpression
  defineStyle?: DefineStyle[]
}

export function transformJsxMacros(
  code: string,
  id: string,
  importMap: Map<string, string>,
  options: OptionsResolved,
): CodeTransform | undefined {
  const s = new MagicString(code)
  const lang = getLang(id)
  if (lang === 'dts') return
  const ast = babelParse(s.original, lang)
  const rootMap = getRootMap(ast, s, options)

  let defineStyleIndex = 0
  for (const [root, macros] of rootMap) {
    macros.defineStyle?.forEach((defineStyle) => {
      transformDefineStyle(
        defineStyle,
        defineStyleIndex++,
        root,
        s,
        importMap,
        macros,
      )
    })

    if (root === undefined) continue

    let propsName = `${HELPER_PREFIX}props`
    if (root.params[0]) {
      if (root.params[0].type === 'Identifier') {
        propsName = root.params[0].name
      } else if (root.params[0].type === 'ObjectPattern') {
        const lastProp = root.params[0].properties.at(-1)
        if (
          !macros.defineComponent &&
          lastProp?.type === 'RestElement' &&
          lastProp.argument.type === 'Identifier'
        ) {
          propsName = lastProp.argument.name
        } else {
          s.appendRight(
            root.params[0].extra?.trailingComma
              ? (root.params[0].extra?.trailingComma as number) + 1
              : lastProp?.end || root.params[0].end! - 1,
            `${
              !root.params[0].extra?.trailingComma &&
              root.params[0].properties.length
                ? ','
                : ''
            } ...${HELPER_PREFIX}props`,
          )
        }
      }
    } else if (macros.defineModel?.length) {
      s.appendRight(getParamsStart(root, s.original), propsName)
    }

    if (macros.defineComponent) {
      transformDefineComponent(
        root,
        propsName,
        macros,
        s,
        options.defineComponent?.autoReturnFunction,
      )
    }
    if (macros.defineModel?.length) {
      macros.defineModel.forEach(({ expression }) => {
        transformDefineModel(expression, propsName, s)
      })
    }
    if (macros.defineSlots) {
      transformDefineSlots(macros.defineSlots.expression, s)
    }
    if (macros.defineExpose) {
      transformDefineExpose(macros.defineExpose, s)
    }
  }

  if (s.hasChanged()) {
    return {
      code: s.toString(),
      get map() {
        return s.generateMap({
          source: id,
          includeContent: true,
          hires: 'boundary',
        })
      },
    }
  }
}

function getRootMap(ast: Program, s: MagicString, options: OptionsResolved) {
  const parents: (Node | undefined | null)[] = []
  const rootMap = new Map<FunctionalNode | undefined, Macros>()
  walkAST<Node>(ast, {
    enter(node, parent) {
      parents.unshift(parent)
      const root = isFunctionalNode(parents[1]) ? parents[1] : undefined

      if (
        root &&
        parents[2]?.type === 'CallExpression' &&
        options.defineComponent.alias.includes(
          s.slice(parents[2].callee.start!, parents[2].callee.end!),
        )
      ) {
        if (!rootMap.has(root)) rootMap.set(root, {})
        if (!rootMap.get(root)!.defineComponent) {
          rootMap.get(root)!.defineComponent = parents[2]
        }
      }

      const expression =
        node.type === 'VariableDeclaration'
          ? node.declarations[0].init?.type === 'CallExpression' &&
            s.slice(
              node.declarations[0].init.callee.start!,
              node.declarations[0].init.callee.end!,
            ) === '$'
            ? node.declarations[0].init.arguments[0]
            : node.declarations[0].init
          : node.type === 'ExpressionStatement'
            ? node.expression
            : undefined
      if (!expression) return
      const macroExpression = getMacroExpression(expression, options)
      if (!macroExpression) return
      if (!rootMap.has(root)) rootMap.set(root, {})
      const macro =
        macroExpression.callee.type === 'MemberExpression'
          ? macroExpression.callee.object
          : macroExpression.callee
      const macroName = s.slice(macro.start!, macro.end!)
      if (macroName) {
        if (options.defineModel.alias.includes(macroName)) {
          ;(rootMap.get(root)!.defineModel ??= []).push({
            expression: macroExpression,
            isRequired: expression.type === 'TSNonNullExpression',
          })
        } else if (options.defineStyle.alias.includes(macroName)) {
          const lang =
            macroExpression.callee.type === 'MemberExpression' &&
            macroExpression.callee.property.type === 'Identifier'
              ? macroExpression.callee.property.name
              : 'css'
          ;(rootMap.get(root)!.defineStyle ??= []).push({
            expression: macroExpression,
            isCssModules: node.type === 'VariableDeclaration',
            lang,
          })
        } else if (options.defineSlots.alias.includes(macroName)) {
          rootMap.get(root)!.defineSlots = {
            expression: macroExpression,
            id:
              node.type === 'VariableDeclaration'
                ? node.declarations[0].id
                : undefined,
          }
        } else if (options.defineExpose.alias.includes(macroName)) {
          rootMap.get(root)!.defineExpose = macroExpression
        }
      }
    },
    leave() {
      parents.shift()
    },
  })
  return rootMap
}

export function getMacroExpression(
  node: Node,
  options: OptionsResolved,
): CallExpression | undefined {
  if (node.type === 'TSNonNullExpression') {
    node = node.expression
  }

  if (node.type === 'CallExpression') {
    if (
      node.callee.type === 'MemberExpression' &&
      node.callee.object.type === 'Identifier' &&
      node.callee.object.name === 'defineStyle'
    ) {
      return node
    } else if (
      node.callee.type === 'Identifier' &&
      [
        ...options.defineComponent.alias,
        ...options.defineSlots.alias,
        ...options.defineModel.alias,
        ...options.defineExpose.alias,
        ...options.defineStyle.alias,
      ].includes(node.callee.name!)
    ) {
      return node
    }
  }
}
