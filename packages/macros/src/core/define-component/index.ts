import { walkAST, walkIdentifiers } from 'ast-kit'
import { restructure } from '../restructure'
import {
  getDefaultValue,
  HELPER_PREFIX,
  importHelperFn,
  isFunctionalNode,
  prependFunctionalNode,
  type FunctionalNode,
} from '../utils'
import { transformAwait } from './await'
import type { Macros } from '..'
import type { Node } from '@babel/types'
import type MagicString from 'magic-string'

export function transformDefineComponent(
  root: FunctionalNode,
  propsName: string,
  macros: Macros,
  s: MagicString,
): void {
  if (!macros.defineComponent) return

  const defineComponentName = s.slice(
    macros.defineComponent.callee.start!,
    macros.defineComponent.callee.end!,
  )
  if (
    defineComponentName &&
    !['defineComponent', 'defineVaporComponent'].includes(defineComponentName)
  ) {
    importHelperFn(s, 'defineComponent', defineComponentName)
  }

  let hasRestProp = false
  const props: Record<string, string | null> = {}
  if (root.params[0]) {
    if (root.params[0].type === 'Identifier') {
      getWalkedIds(root, propsName).forEach((id) => (props[id] = null))
      prependFunctionalNode(
        root,
        s,
        `const ${propsName} = ${importHelperFn(
          s,
          'useFullProps',
          undefined,
          '/vue-jsx-vapor/props',
        )}()`,
      )
      s.overwrite(
        root.params[0].start!,
        root.params[0].end!,
        root.params.length > 1
          ? `${HELPER_PREFIX}props`
          : root.start === root.params[0].start!
            ? '()'
            : '',
      )
    } else if (root.params[0].type === 'ObjectPattern') {
      const restructuredProps = root.params[0]
      for (const prop of restructuredProps.properties) {
        if (prop.type !== 'ObjectProperty' || prop.key.type !== 'Identifier')
          continue
        const propName = prop.key.name
        if (prop.value.type !== 'AssignmentPattern') {
          props[propName] = null
          continue
        }
        const defaultValue = getDefaultValue(prop.value.right)
        let isRequired = false
        walkAST(prop.value.right, {
          enter(node) {
            if (node.type === 'TSNonNullExpression') {
              isRequired = true
              this.skip()
            }
          },
        })

        const propOptions = []
        if (isRequired) {
          propOptions.push('required: true')
        }
        if (defaultValue) {
          const { value, type, skipFactory } = getTypeAndValue(s, defaultValue)
          if (type) {
            propOptions.push(`type: ${type}`)
          }
          if (value) {
            propOptions.push(`default: ${value}`)
          }
          if (skipFactory) {
            propOptions.push('skipFactory: true')
          }
        }
        if (propOptions.length) {
          props[propName] = `{ ${propOptions.join(', ')} }`
        } else {
          props[propName] = null
        }
      }

      restructure(s, root, {
        skipDefaultProps: true,
        generateRestProps: (restPropsName, index, list) => {
          if (index === list.length - 1) {
            hasRestProp = true
            const useAttrs = importHelperFn(s, 'useAttrs')
            return `const ${restPropsName} = ${useAttrs}()`
          }
        },
      })
    }
  }

  transformDefineModel(s, macros.defineModel, props)

  const propsString = Object.entries(props)
    .map(([key, value]) => `'${key}': ${value}`)
    .join(', \n')
  if (propsString) {
    const resolvedPropsString = `${hasRestProp ? 'inheritAttrs: false, ' : ''}props: {\n${propsString}\n}`
    const compOptions = macros.defineComponent.arguments[1]
    if (!compOptions) {
      s.appendRight(root.end!, `, { ${resolvedPropsString} }`)
    } else if (compOptions.type === 'ObjectExpression') {
      s.appendLeft(compOptions.start!, `{ ${resolvedPropsString}, ...`)
      s.appendRight(compOptions.end!, ' }')
    }
  }

  transformAwait(root, s)
}

function getWalkedIds(root: FunctionalNode, propsName: string) {
  const walkedIds = new Set<string>()
  walkIdentifiers(root.body, (id, parent) => {
    if (
      id.name === propsName &&
      (parent?.type === 'MemberExpression' ||
        parent?.type === 'JSXMemberExpression' ||
        parent?.type === 'OptionalMemberExpression')
    ) {
      const prop =
        parent.property.type === 'Identifier' ||
        parent.property.type === 'JSXIdentifier'
          ? parent.property.name
          : parent.property.type === 'StringLiteral'
            ? parent.property.value
            : ''
      if (prop) walkedIds.add(prop)
    }
  })
  return walkedIds
}

function transformDefineModel(
  s: MagicString,
  defineModel: Macros['defineModel'],
  props: Record<string, string | null>,
) {
  for (const { expression, isRequired } of defineModel || []) {
    const modelOptions =
      expression.arguments[0]?.type === 'ObjectExpression'
        ? expression.arguments[0]
        : expression.arguments[1]?.type === 'ObjectExpression'
          ? expression.arguments[1]
          : undefined
    const options: any = {}
    if (isRequired) options.required = true
    let defaultValueNode: Node | undefined
    for (const prop of modelOptions?.properties || []) {
      if (
        prop.type === 'ObjectProperty' &&
        prop.key.type === 'Identifier' &&
        ['validator', 'type', 'required', 'default'].includes(prop.key.name)
      ) {
        if (prop.key.name === 'default') {
          defaultValueNode = prop.value
        }
        options[prop.key.name] = s.slice(prop.value.start!, prop.value.end!)
      }
    }
    if (defaultValueNode && !options.type) {
      const { value, type, skipFactory } = getTypeAndValue(s, defaultValueNode)
      if (type) {
        options.type = type
      }
      if (value) {
        options.default = value
      }
      if (skipFactory) {
        options.skipFactory = 'true'
      }
    }
    const propName =
      expression.arguments[0]?.type === 'StringLiteral'
        ? expression.arguments[0].value
        : 'modelValue'
    props[propName] = Object.keys(options).length
      ? `{ ${Object.entries(options)
          .map(([key, value]) => `${key}: ${value}`)
          .join(', ')} }`
      : null
    props[`onUpdate:${propName}`] = null
    props[`${propName === 'modelValue' ? 'model' : propName}Modifiers`] = null
  }
}

function getTypeAndValue(s: MagicString, node: Node) {
  let value = ''
  let type = ''
  let skipFactory = false
  switch (node.type) {
    case 'StringLiteral': {
      type = 'String'
      value = `'${node.value}'`

      break
    }
    case 'BooleanLiteral': {
      type = 'Boolean'
      value = String(node.value)

      break
    }
    case 'NumericLiteral': {
      type = 'Number'
      value = String(node.value)

      break
    }
    case 'ObjectExpression': {
      type = 'Object'
      value = `() => (${s.slice(node.start!, node.end!)})`

      break
    }
    case 'ArrayExpression': {
      type = 'Array'
      value = `() => (${s.slice(node.start!, node.end!)})`

      break
    }
    default:
      if (isFunctionalNode(node)) {
        type = 'Function'
        value = s.slice(node.start!, node.end!)
      } else if (node.type === 'Identifier') {
        if (node.name === 'undefined') {
          value = 'undefined'
        } else {
          skipFactory = true
          value = s.slice(node.start!, node.end!)
        }
      } else if (node.type === 'NullLiteral') {
        value = 'null'
      }
  }
  return { value, type, skipFactory }
}
