import { isFunctionalNode } from '@vue-jsx-vapor/macros/api'
import getHash from 'hash-sum'
import { injectSSR } from './ssr'
import type { BabelFileResult, types } from '@babel/core'

export interface HotComponent {
  local: string
  exported: string
  id: string
}

export function injectHMRAndSSR(
  result: BabelFileResult,
  id: string,
  options?: {
    ssr?: boolean
    root?: string
    defineComponentNames?: string[]
  },
) {
  const ssr = options?.ssr
  const defineComponentNames = options?.defineComponentNames ?? [
    'defineComponent',
    'defineVaporComponent',
  ]
  const { ast } = result

  // check for hmr injection
  const declaredComponents: string[] = []
  const hotComponents: HotComponent[] = []
  let hasDefaultExport = false

  for (const node of ast!.program.body) {
    if (node.type === 'VariableDeclaration') {
      const names = parseComponentDecls(node, defineComponentNames)
      if (names.length) {
        declaredComponents.push(...names)
      }
    }

    if (node.type === 'ExportNamedDeclaration') {
      if (node.declaration && node.declaration.type === 'VariableDeclaration') {
        hotComponents.push(
          ...parseComponentDecls(node.declaration, defineComponentNames).map(
            (name) => ({
              local: name,
              exported: name,
              id: getHash(id + name),
            }),
          ),
        )
      } else if (node.specifiers.length) {
        for (const spec of node.specifiers) {
          if (
            spec.type === 'ExportSpecifier' &&
            spec.exported.type === 'Identifier'
          ) {
            const matched = declaredComponents.find(
              (name) => name === spec.local.name,
            )
            if (matched) {
              hotComponents.push({
                local: spec.local.name,
                exported: spec.exported.name,
                id: getHash(id + spec.exported.name),
              })
            }
          }
        }
      }
    }

    if (node.type === 'ExportDefaultDeclaration') {
      if (node.declaration.type === 'Identifier') {
        const _name = node.declaration.name
        const matched = declaredComponents.find((name) => name === _name)
        if (matched) {
          hotComponents.push({
            local: _name,
            exported: 'default',
            id: getHash(`${id}default`),
          })
        }
      } else if (
        isDefineComponentCall(node.declaration, defineComponentNames) ||
        isFunctionalNode(node.declaration)
      ) {
        hasDefaultExport = true
        hotComponents.push({
          local: '__default__',
          exported: 'default',
          id: getHash(`${id}default`),
        })
      }
    }
  }

  if (hotComponents.length) {
    if (hasDefaultExport || ssr) {
      result.code = `${result.code!.replaceAll(
        `export default `,
        `const __default__ = `,
      )}\nexport default __default__;`
    }

    if (!ssr && !/\?vue&type=script/.test(id)) {
      let code = result.code
      let callbackCode = ``
      for (const { local, exported, id } of hotComponents) {
        code +=
          `\n${local}.__hmrId = "${id}";` +
          `\n__VUE_HMR_RUNTIME__.createRecord("${id}", ${local});`
        callbackCode += `
    __VUE_HMR_RUNTIME__.rerender(mod['${exported}'].__hmrId, mod['${exported}'].setup || mod['${exported}']);`
      }

      code += `
if (import.meta.hot) {
  import.meta.hot.accept(mod => {${callbackCode}\n  });
}`
      result.code = code
    }

    if (ssr) {
      result.code += injectSSR(id, hotComponents, options?.root)
    }
  }
}

function parseComponentDecls(
  node: types.VariableDeclaration,
  fnNames: string[],
) {
  const names = []
  for (const decl of node.declarations) {
    if (
      decl.id.type === 'Identifier' &&
      (isDefineComponentCall(decl.init, fnNames) || isFunctionalNode(decl.init))
    )
      names.push(decl.id.name)
  }
  return names
}

function isDefineComponentCall(
  node: types.Node | null | undefined,
  names: string[],
): node is types.CallExpression {
  return !!(
    node &&
    node.type === 'CallExpression' &&
    node.callee.type === 'Identifier' &&
    names.includes(node.callee.name)
  )
}
