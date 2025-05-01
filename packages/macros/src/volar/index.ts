import { HELPER_PREFIX, type Overwrite } from '@vue-macros/common'
import { replaceRange, type TsmVirtualCode } from 'ts-macro'
import type { OptionsResolved } from '../options'
import { transformDefineComponent } from './define-component'

export { transformJsxMacros } from './transform'
export { getGlobalTypes } from './global-types'

export type TransformOptions = Overwrite<
  TsmVirtualCode,
  {
    ts: typeof import('typescript')
  } & OptionsResolved
>

export type JsxMacros = {
  defineModel?: string[]
  defineSlots?: string
  defineExpose?: string
  defineStyle?: {
    expression: import('typescript').CallExpression
    isCssModules: boolean
  }[]
  defineComponent?: true
}
export type RootMap = Map<
  | import('typescript').ArrowFunction
  | import('typescript').FunctionExpression
  | import('typescript').FunctionDeclaration
  | undefined,
  JsxMacros
>

function getMacro(
  node: import('typescript').Node | undefined,
  ts: typeof import('typescript'),
  options: TransformOptions,
):
  | {
      expression: import('typescript').CallExpression
      initializer: import('typescript').Node
      isRequired: boolean
    }
  | undefined {
  if (!node) return

  if (ts.isVariableStatement(node)) {
    return ts.forEachChild(node.declarationList, (decl) => getExpression(decl))
  } else {
    return getExpression(node)
  }

  function getExpression(decl: import('typescript').Node) {
    if (ts.isVariableDeclaration(decl) && decl.initializer) {
      const initializer =
        ts.isCallExpression(decl.initializer) &&
        ts.isIdentifier(decl.initializer.expression) &&
        decl.initializer.expression.escapedText === '$' &&
        decl.initializer.arguments[0]
          ? decl.initializer.arguments[0]
          : decl.initializer
      const expression = getMacroExpression(initializer)
      if (expression) {
        return {
          expression,
          initializer: decl.initializer,
          isRequired: ts.isNonNullExpression(initializer),
        }
      }
    } else if (ts.isExpressionStatement(decl)) {
      const expression = getMacroExpression(decl.expression)
      if (expression)
        return {
          expression,
          initializer: decl.expression,
          isRequired: ts.isNonNullExpression(decl.expression),
        }
    }
  }

  function getMacroExpression(node: import('typescript').Node) {
    if (ts.isNonNullExpression(node)) {
      node = node.expression
    }
    if (!ts.isCallExpression(node)) return
    const expression = ts.isPropertyAccessExpression(node.expression)
      ? node.expression
      : node
    return (
      ts.isIdentifier(expression.expression) &&
      [
        ...options.defineModel.alias,
        ...options.defineSlots.alias,
        ...options.defineStyle.alias,
        ...options.defineExpose.alias,
        ...options.defineComponent.alias,
      ].includes(expression.expression.escapedText!) &&
      node
    )
  }
}

export function getRootMap(options: TransformOptions): RootMap {
  const { ts, ast, codes } = options
  const rootMap: RootMap = new Map()

  function walk(
    node: import('typescript').Node,
    parents: import('typescript').Node[],
  ) {
    ts.forEachChild(node, (child) => {
      parents.unshift(node)
      walk(child, parents)
      parents.shift()
    })

    const root =
      parents[1] &&
      (ts.isArrowFunction(parents[1]) ||
        ts.isFunctionExpression(parents[1]) ||
        ts.isFunctionDeclaration(parents[1]))
        ? parents[1]
        : undefined

    if (
      root &&
      parents[2] &&
      ts.isCallExpression(parents[2]) &&
      !parents[2].typeArguments &&
      options.defineComponent.alias.includes(parents[2].expression.getText(ast))
    ) {
      if (!rootMap.has(root)) rootMap.set(root, {})
      if (!rootMap.get(root)!.defineComponent) {
        rootMap.get(root)!.defineComponent = true
        transformDefineComponent(parents[2], parents[3], options)
      }
    }

    const macro = getMacro(node, ts, options)
    if (!macro) return

    const { expression, initializer } = macro
    let isRequired = macro.isRequired
    if (!rootMap.has(root)) rootMap.set(root, {})
    const macroName = expression.expression.getText(ast)
    if (macroName.startsWith('defineStyle')) {
      ;(rootMap.get(root)!.defineStyle ??= [])!.push({
        expression,
        isCssModules: !!ts.isVariableStatement(node),
      })
    }

    if (!root) return

    if (options.defineModel.alias.includes(macroName)) {
      const modelName =
        expression.arguments[0] &&
        ts.isStringLiteralLike(expression.arguments[0])
          ? expression.arguments[0].text
          : 'modelValue'
      const modelOptions =
        expression.arguments[0] &&
        ts.isStringLiteralLike(expression.arguments[0])
          ? expression.arguments[1]
          : expression.arguments[0]
      if (modelOptions && ts.isObjectLiteralExpression(modelOptions)) {
        let hasRequired = false
        for (const prop of modelOptions.properties) {
          if (
            ts.isPropertyAssignment(prop) &&
            prop.name.getText(ast) === 'required'
          ) {
            hasRequired = true
            isRequired = prop.initializer.kind === ts.SyntaxKind.TrueKeyword
          }
        }

        if (!hasRequired && isRequired) {
          replaceRange(
            codes,
            modelOptions.end - 1,
            modelOptions.end - 1,
            `${!modelOptions.properties.hasTrailingComma && modelOptions.properties.length ? ',' : ''} required: true`,
          )
        }
      } else if (isRequired) {
        replaceRange(
          codes,
          expression.arguments.end,
          expression.arguments.end,
          `${!expression.arguments.hasTrailingComma && expression.arguments.length ? ',' : ''} { required: true }`,
        )
      }

      const id = toValidAssetId(modelName, `${HELPER_PREFIX}model`)
      const typeString = `import('vue').UnwrapRef<typeof ${id}>`
      const defineModel = (rootMap.get(root)!.defineModel ??= [])
      defineModel.push(
        `${modelName.includes('-') ? `'${modelName}'` : modelName}${isRequired ? ':' : '?:'} ${typeString}`,
        `'onUpdate:${modelName}'?: ($event: ${typeString}) => any`,
      )
      if (expression.typeArguments?.[1]) {
        defineModel.push(
          `${modelName}Modifiers?: Partial<Record<${expression.typeArguments[1].getText(ast)}, boolean>>`,
        )
      }
      replaceRange(
        codes,
        initializer.getStart(ast),
        initializer.getStart(ast),
        `// @ts-ignore\n${id};\nlet ${id} = `,
      )
    } else if (options.defineSlots.alias.includes(macroName)) {
      replaceRange(
        codes,
        expression.getStart(ast),
        expression.getStart(ast),
        `// @ts-ignore\n${HELPER_PREFIX}slots;\nconst ${HELPER_PREFIX}slots = `,
      )
      rootMap.get(root)!.defineSlots = `Partial<typeof ${HELPER_PREFIX}slots>`
    } else if (options.defineExpose.alias.includes(macroName)) {
      replaceRange(
        codes,
        expression.getStart(ast),
        expression.getStart(ast),
        `// @ts-ignore\n${HELPER_PREFIX}exposed;\nconst ${HELPER_PREFIX}exposed = `,
      )
      rootMap.get(root)!.defineExpose = `typeof ${HELPER_PREFIX}exposed`
    }
  }

  ts.forEachChild(ast, (node) => walk(node, []))
  return rootMap
}

function toValidAssetId(name: string, type: string): string {
  return `_${type}_${name.replaceAll(/\W/g, (searchValue, replaceValue) => {
    return searchValue === '-' ? '_' : name.charCodeAt(replaceValue).toString()
  })}`
}
