import { walkIdentifiers } from 'ast-kit'
import { withDefaultsHelperId } from './helper'
import {
  camelize,
  getDefaultValue,
  getPropertyAccess,
  HELPER_PREFIX,
  importHelperFn,
  prependFunctionalNode,
  resolveObjectKey,
  toStringLiteral,
  type FunctionalNode,
} from './utils'
import type { Node, ObjectProperty } from '@babel/types'
import type MagicString from 'magic-string'

type Options = {
  withDefaultsFrom?: string
  skipDefaultProps?: boolean
  // Read the keys of the first parameter camelized, as Vue normalizes prop names.
  camelizeKeys?: boolean
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
  // The key `createPropsDefaultProxy` reads `defaultValue` from.
  defaultKey?: string
  isRest?: boolean
}

// `.foo` or `[0]`, or the source of a computed key.
type KeySegment = string | { computed: string }

export function restructure(
  s: MagicString,
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
            'createPropsRestProxy',
          )}(${rest.path}, [${rest.value}])`,
      )
    }

    for (const [path, values] of Object.entries(defaultValues)) {
      const createPropsDefaultProxy = importHelperFn(
        s,
        'createPropsDefaultProxy',
        undefined,
        options.withDefaultsFrom ?? withDefaultsHelperId,
      )
      const resolvedValues = values
        .map((i) => `${i.defaultKey}: ${i.defaultValue}`)
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
  s: MagicString,
  options: Options,
  node: Node,
  path = '',
  props: Prop[] = [],
  keyPath: KeySegment[] = [],
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
      const defaultValue = getDefaultValue(prop.right)
      props.push({
        path,
        name: prop.left.name,
        value: `[${index}]`,
        defaultValue: s.slice(defaultValue.start!, defaultValue.end!),
        defaultKey: getDefaultKey([...keyPath, `[${index}]`]),
      })
      propNames.push(`'${prop.left.name}'`)
    } else if (prop?.type === 'ObjectProperty') {
      const key = getPropKey(s, options, prop, path)
      const propKeyPath = [...keyPath, key.segment]
      if (prop.value.type === 'AssignmentPattern') {
        if (prop.value.left.type === 'Identifier') {
          // { foo: bar = 'foo' }
          const defaultValue = getDefaultValue(prop.value.right)
          props.push({
            path,
            name: prop.value.left.name,
            value: key.access,
            defaultValue: s.slice(defaultValue.start!, defaultValue.end!),
            defaultKey: getDefaultKey(propKeyPath),
          })
        } else {
          // { foo: { bar } = {} }
          getProps(
            s,
            options,
            prop.value.left,
            `${path}${key.access}`,
            props,
            propKeyPath,
          )
        }
      } else if (
        !getProps(
          s,
          options,
          prop.value,
          `${path}${key.access}`,
          props,
          propKeyPath,
        )
      ) {
        // { foo: bar }
        const name =
          prop.value.type === 'Identifier'
            ? prop.value.name
            : prop.key.type === 'Identifier' && !prop.computed
              ? prop.key.name
              : ''
        props.push({
          path,
          name,
          value: key.access,
        })
      }
      propNames.push(key.restKey)
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
      getProps(s, options, prop, `${path}[${index}]`, props, [
        ...keyPath,
        `[${index}]`,
      ])
    }
  })
  return props.length ? props : undefined
}

// { foo: bar }, { 'foo-bar': bar }, { 0: bar } or { [foo]: bar }
// `access` reads the key, `restKey` excludes it from a rest element and
// `segment` locates its default.
function getPropKey(
  s: MagicString,
  options: Options,
  prop: ObjectProperty,
  path: string,
): { access: string; restKey: string; segment: KeySegment } {
  let key = resolveObjectKey(prop.key, prop.computed)
  if (key === undefined) {
    const source = s.slice(prop.key.start!, prop.key.end!)
    return {
      access: `[${source}]`,
      // A rest compares string keys, so `[1]` has to exclude '1'.
      restKey: `String(${source})`,
      segment: { computed: source },
    }
  }
  if (options.camelizeKeys && path === `${HELPER_PREFIX}props`) {
    key = camelize(key)
  }
  return {
    access: getPropertyAccess(key),
    restKey: toStringLiteral(key),
    segment: `.${key}`,
  }
}

// A static path such as '.foo[0]', or a template literal when the path goes
// through a computed key.
function getDefaultKey(keyPath: KeySegment[]): string {
  if (keyPath.every((i) => typeof i === 'string')) {
    return toStringLiteral(keyPath.join(''))
  }
  const path = keyPath
    .map((i) =>
      typeof i === 'string'
        ? i.replaceAll(/[`\\$]/g, String.raw`\$&`)
        : `.\${${i.computed}}`,
    )
    .join('')
  return `[\`${path}\`]`
}
