import {
  HELPER_PREFIX,
  importHelperFn,
  walkIdentifiers,
  type MagicStringAST,
} from '@vue-macros/common'
import { restructure } from '../restructure'
import {
  getDefaultValue,
  isFunctionalNode,
  prependFunctionalNode,
  type FunctionalNode,
} from '../utils'
import type { Macros } from '..'
import { transformAwait } from './await'
import { transformReturn } from './return'
import type { Node, ObjectExpression } from '@babel/types'

export function transformDefineComponent(
  root: FunctionalNode,
  propsName: string,
  macros: Macros,
  s: MagicStringAST,
  autoReturnFunction = false,
): void {
  if (!macros.defineComponent) return

  const defineComponentName = s.sliceNode(macros.defineComponent.callee)
  if (
    defineComponentName &&
    !['defineComponent', 'defineVaporComponent'].includes(defineComponentName)
  ) {
    importHelperFn(s, 0, 'defineComponent', defineComponentName)
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
          0,
          'useFullProps',
          undefined,
          'vue-jsx-vapor',
        )}()`,
      )
      s.overwrite(
        root.params[0].start!,
        root.params[0].end!,
        `${HELPER_PREFIX}props`,
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
        const isRequired = prop.value.right.type === 'TSNonNullExpression'

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
            const useAttrs = importHelperFn(s, 0, 'useAttrs')
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
    const argument = macros.defineComponent.arguments[1]
    if (!argument) {
      s.appendRight(
        root.end!,
        `, {${hasRestProp ? 'inheritAttrs: false,' : ''} props: {\n${propsString}\n} }`,
      )
    } else if (argument.type === 'ObjectExpression') {
      const resolvedPropsString = `{\n${propsString}\n}`
      const prop = prependObjectExpression(
        argument,
        'props',
        resolvedPropsString,
        s,
      )
      if (
        prop &&
        prop.type === 'ObjectProperty' &&
        prop.value.type === 'ObjectExpression'
      ) {
        s.appendLeft(prop.value.start!, `{...${resolvedPropsString}, ...`)
        s.appendRight(prop.value.end!, '}')
      }
      if (hasRestProp) {
        prependObjectExpression(argument, 'inheritAttrs', 'false', s)
      }
    }
  }

  transformAwait(root, s)
  if (autoReturnFunction) {
    transformReturn(root, s)
  }
}

function prependObjectExpression(
  argument: ObjectExpression,
  name: string,
  value: string,
  s: MagicStringAST,
) {
  const prop = argument.properties?.find(
    (prop) =>
      prop.type === 'ObjectProperty' &&
      prop.key.type === 'Identifier' &&
      prop.key.name === name,
  )
  if (!prop) {
    s.appendRight(argument.start! + 1, `${name}: ${value},`)
  }
  return prop
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
  s: MagicStringAST,
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
        options[prop.key.name] = s.sliceNode(prop.value)
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

function getTypeAndValue(s: MagicStringAST, node: Node) {
  let value = ''
  let type = ''
  let skipFactory = false
  if (node.type === 'StringLiteral') {
    type = 'String'
    value = `'${node.value}'`
  } else if (node.type === 'BooleanLiteral') {
    type = 'Boolean'
    value = String(node.value)
  } else if (node.type === 'NumericLiteral') {
    type = 'Number'
    value = String(node.value)
  } else if (node.type === 'ObjectExpression') {
    type = 'Object'
    value = `() => (${s.sliceNode(node)})`
  } else if (node.type === 'ArrayExpression') {
    type = 'Array'
    value = `() => (${s.sliceNode(node)})`
  } else if (isFunctionalNode(node)) {
    type = 'Function'
    value = s.sliceNode(node)
  } else if (node.type === 'Identifier') {
    if (node.name === 'undefined') {
      value = 'undefined'
    } else {
      skipFactory = true
      value = s.sliceNode(node)
    }
  } else if (node.type === 'NullLiteral') {
    value = 'null'
  }
  return { value, type, skipFactory }
}
