import {
  HELPER_PREFIX,
  importHelperFn,
  walkIdentifiers,
  type MagicStringAST,
} from '@vue-macros/common'
import { withDefaultsHelperId } from './helper'
import {
  getDefaultValue,
  prependFunctionalNode,
  type FunctionalNode,
} from './utils'
import type { Node } from '@babel/types'

type Options = {
  withDefaultsFrom?: string
  skipDefaultProps?: boolean
  generateRestProps?: (
    restPropsName: string,
    index: number,
    list: Prop[],
  ) => string | undefined
}

type Prop = {
  path: string
  name: string
  value: string
  defaultValue?: string
  isRest?: boolean
}

export function restructure(
  s: MagicStringAST,
  node: FunctionalNode,
  options: Options = {},
): Prop[] {
  let index = 0
  const propList: Prop[] = []
  for (const param of node.params) {
    const path = `${HELPER_PREFIX}props${index++ || ''}`
    const props = getProps(s, options, param, path)
    if (props) {
      s.overwrite(param.start!, param.end!, path)
      propList.push(...props)
    }
  }

  if (propList.length) {
    const defaultValues: Record<string, Prop[]> = {}
    const rests = []
    for (const prop of propList) {
      if (prop.isRest) {
        rests.push(prop)
      }
      if (prop.defaultValue) {
        const paths = prop.path.split(/\.|\[/)
        if (!options.skipDefaultProps || paths.length !== 1) {
          ;(defaultValues[paths[0]] ??= []).push(prop)
        }
      }
    }

    for (const [index, rest] of rests.entries()) {
      prependFunctionalNode(
        node,
        s,
        options.generateRestProps?.(rest.name, index, rests) ??
          `\nconst ${rest.name} = ${importHelperFn(
            s,
            0,
            'createPropsRestProxy',
          )}(${rest.path}, [${rest.value}])`,
      )
    }

    for (const [path, values] of Object.entries(defaultValues)) {
      const createPropsDefaultProxy = importHelperFn(
        s,
        0,
        'createPropsDefaultProxy',
        undefined,
        options.withDefaultsFrom ?? withDefaultsHelperId,
      )
      const resolvedValues = values
        .map(
          (i) => `'${i.path.replace(path, '')}${i.value}': ${i.defaultValue}`,
        )
        .join(', ')
      prependFunctionalNode(
        node,
        s,
        `\n${path} = ${createPropsDefaultProxy}(${path}, {${resolvedValues}})`,
      )
    }

    walkIdentifiers(
      node.body,
      (id, parent) => {
        const prop = propList.find((i) => i.name === id.name)
        if (prop && !prop.isRest) {
          s.overwrite(
            id.start!,
            id.end!,
            `${
              parent?.type === 'ObjectProperty' && parent.shorthand
                ? `${id.name}: `
                : ''
            }${prop.path}${prop.value}`,
          )
        }
      },
      false,
    )
  }

  return propList
}

function getProps(
  s: MagicStringAST,
  options: Options,
  node: Node,
  path = '',
  props: Prop[] = [],
) {
  const properties =
    node.type === 'ObjectPattern'
      ? node.properties
      : node.type === 'ArrayPattern'
        ? node.elements
        : []
  if (!properties.length) return

  const propNames: string[] = []
  properties.forEach((prop, index) => {
    if (prop?.type === 'Identifier') {
      // { foo }
      props.push({
        name: prop.name,
        path,
        value: `[${index}]`,
      })
      propNames.push(`'${prop.name}'`)
    } else if (
      prop?.type === 'AssignmentPattern' &&
      prop.left.type === 'Identifier'
    ) {
      // [foo = 'foo']
      props.push({
        path,
        name: prop.left.name,
        value: `[${index}]`,
        defaultValue: s.sliceNode(getDefaultValue(prop.right)),
      })
      propNames.push(`'${prop.left.name}'`)
    } else if (
      prop?.type === 'ObjectProperty' &&
      prop.key.type === 'Identifier'
    ) {
      if (prop.value.type === 'AssignmentPattern') {
        if (prop.value.left.type === 'Identifier') {
          // { foo: bar = 'foo' }
          props.push({
            path,
            name: prop.value.left.name,
            value: `.${prop.key.name}`,
            defaultValue: s.sliceNode(getDefaultValue(prop.value.right)),
          })
        } else {
          // { foo: { bar } = {} }
          getProps(
            s,
            options,
            prop.value.left,
            `${path}.${prop.key.name}`,
            props,
          )
        }
      } else if (
        !getProps(s, options, prop.value, `${path}.${prop.key.name}`, props)
      ) {
        // { foo: bar }
        const name =
          prop.value.type === 'Identifier' ? prop.value.name : prop.key.name
        props.push({
          path,
          name,
          value: `.${prop.key.name}`,
        })
      }
      propNames.push(`'${prop.key.name}'`)
    } else if (
      prop?.type === 'RestElement' &&
      prop.argument.type === 'Identifier' &&
      !prop.argument.name.startsWith(`${HELPER_PREFIX}props`)
    ) {
      // { ...rest }
      props.push({
        path,
        name: prop.argument.name,
        value: propNames.join(', '),
        isRest: true,
      })
    } else if (prop) {
      getProps(s, options, prop, `${path}[${index}]`, props)
    }
  })
  return props.length ? props : undefined
}
