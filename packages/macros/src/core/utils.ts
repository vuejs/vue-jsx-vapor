/* eslint-disable node/prefer-global/process */
import type {
  ArrowFunctionExpression,
  FunctionDeclaration,
  FunctionExpression,
  Node,
} from '@babel/types'
import type MagicString from 'magic-string'

export type FunctionalNode =
  | FunctionDeclaration
  | FunctionExpression
  | ArrowFunctionExpression

export function prependFunctionalNode(
  node: FunctionalNode,
  s: MagicString,
  result: string,
): void {
  const isBlockStatement = node.body.type === 'BlockStatement'
  const start = node.body.extra?.parenthesized
    ? (node.body.extra.parenStart as number)
    : node.body.start!
  s.appendRight(
    start + (isBlockStatement ? 1 : 0),
    `${result};${isBlockStatement ? '' : 'return '}`,
  )
  if (!isBlockStatement) {
    s.appendLeft(start, '{')
    s.appendRight(node.end!, '}')
  }
}

export function isFunctionalNode(node?: Node | null): node is FunctionalNode {
  return !!(
    node &&
    (node.type === 'ArrowFunctionExpression' ||
      node.type === 'FunctionDeclaration' ||
      node.type === 'FunctionExpression')
  )
}

export function getParamsStart(node: FunctionalNode, code: string): number {
  return node.params[0]
    ? node.params[0].start!
    : node.start! +
        (code.slice(node.start!, node.body.start!).match(/\(\s*\)/)?.index ||
          0) +
        1
}

export function getDefaultValue(node: Node): Node {
  if (node.type === 'TSNonNullExpression') {
    return getDefaultValue(node.expression)
  }
  if (node.type === 'TSAsExpression') {
    return getDefaultValue(node.expression)
  }
  return node
}

let require: NodeJS.Require | undefined

export function getRequire() {
  if (require) return require

  try {
    // @ts-expect-error check api
    if (globalThis.process?.getBuiltinModule) {
      const module = process.getBuiltinModule('node:module')
      // unenv has implemented `getBuiltinModule` but has yet to support `module.createRequire`
      if (module?.createRequire) {
        return (require = module.createRequire(import.meta.url))
      }
    }
  } catch {}
}

const importedMap = new WeakMap<MagicString, Set<string>>()
export const HELPER_PREFIX = '__'
export function importHelperFn(
  s: MagicString,
  imported: string,
  local: string = imported,
  from = 'vue',
) {
  const cacheKey = `${from}@${imported}`
  if (!importedMap.get(s)?.has(cacheKey)) {
    s.appendLeft(
      0,
      `\nimport ${
        imported === 'default'
          ? HELPER_PREFIX + local
          : `{ ${imported} as ${HELPER_PREFIX + local} }`
      } from ${JSON.stringify(from)};`,
    )
    if (importedMap.has(s)) {
      importedMap.get(s)!.add(cacheKey)
    } else {
      importedMap.set(s, new Set([cacheKey]))
    }
  }

  return `${HELPER_PREFIX}${local}`
}
