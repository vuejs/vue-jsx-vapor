import { extend, isBuiltInDirective, isVoidTag, makeMap } from '@vue/shared'
import {
  DynamicFlag,
  IRDynamicPropsKind,
  IRNodeTypes,
  type IRProp,
  type IRProps,
  type IRPropsDynamicAttribute,
  type IRPropsStatic,
} from '../ir'
import {
  createSimpleExpression,
  isJSXComponent,
  isTemplate,
  isValidHTMLNesting,
  resolveExpression,
  resolveSimpleExpression,
  type SimpleExpressionNode,
} from '../utils'
import type {
  DirectiveTransformResult,
  NodeTransform,
  TransformContext,
} from '../transform'
import type { JSXAttribute, JSXElement, JSXSpreadAttribute } from '@babel/types'

export const isReservedProp = /* #__PURE__ */ makeMap(
  // the leading comma is intentional so empty string "" is also included
  ',key,ref,ref_for,ref_key,',
)

const isEventRegex = /^on[A-Z]/
const isDirectiveRegex = /^v-[a-z]/

export const transformElement: NodeTransform = (node, context) => {
  let effectIndex = context.block.effect.length
  const getEffectIndex = () => effectIndex++
  let operationIndex = context.block.operation.length
  const getOperationIndex = () => operationIndex++
  return function postTransformElement() {
    ;({ node } = context)
    if (node.type !== 'JSXElement' || isTemplate(node)) return

    const {
      openingElement: { name },
    } = node
    const tag =
      name.type === 'JSXIdentifier'
        ? name.name
        : name.type === 'JSXMemberExpression'
          ? context.ir.source.slice(name.start!, name.end!)
          : ''
    const isComponent = isJSXComponent(node)
    const propsResult = buildProps(
      node,
      context as TransformContext<JSXElement>,
      isComponent,
    )

    let { parent } = context
    while (
      parent &&
      parent.parent &&
      parent.node.type === 'JSXElement' &&
      isTemplate(parent.node)
    ) {
      parent = parent.parent
    }
    const singleRoot =
      context.root === parent && parent.node.type !== 'JSXFragment'
    ;(isComponent ? transformComponentElement : transformNativeElement)(
      tag,
      propsResult,
      singleRoot,
      context as TransformContext<JSXElement>,
      getEffectIndex,
      getOperationIndex,
    )
  }
}

function transformComponentElement(
  tag: string,
  propsResult: PropsResult,
  singleRoot: boolean,
  context: TransformContext<JSXElement>,
) {
  let asset = context.options.withFallback

  const dotIndex = tag.indexOf('.')
  if (dotIndex > 0) {
    const ns = tag.slice(0, dotIndex)
    if (ns) {
      tag = ns + tag.slice(dotIndex)
    }
  }

  if (tag.includes('-')) {
    asset = true
  }

  if (asset) {
    context.component.add(tag)
  }

  context.dynamic.flags |= DynamicFlag.NON_TEMPLATE | DynamicFlag.INSERT

  context.dynamic.operation = {
    type: IRNodeTypes.CREATE_COMPONENT_NODE,
    id: context.reference(),
    tag,
    props: propsResult[0] ? propsResult[1] : [propsResult[1]],
    asset,
    root: singleRoot && !context.inVFor,
    slots: [...context.slots],
    once: context.inVOnce,
  }
  context.slots = []
}

function transformNativeElement(
  tag: string,
  propsResult: PropsResult,
  singleRoot: boolean,
  context: TransformContext<JSXElement>,
  getEffectIndex: () => number,
  getOperationIndex: () => number,
) {
  let template = ''

  template += `<${tag}`

  const dynamicProps: string[] = []
  if (propsResult[0] /* dynamic props */) {
    const [, dynamicArgs, expressions] = propsResult
    context.registerEffect(
      expressions,
      {
        type: IRNodeTypes.SET_DYNAMIC_PROPS,
        element: context.reference(),
        props: dynamicArgs,
        root: singleRoot,
      },
      getEffectIndex,
      getOperationIndex,
    )
  } else {
    for (const prop of propsResult[1]) {
      const { key, values } = prop
      if (key.isStatic && values.length === 1 && values[0].isStatic) {
        template += ` ${key.content}`
        if (values[0].content) template += `="${values[0].content}"`
      } else {
        dynamicProps.push(key.content)
        context.registerEffect(
          values,
          {
            type: IRNodeTypes.SET_PROP,
            element: context.reference(),
            prop,
            tag,
            root: singleRoot,
          },
          getEffectIndex,
          getOperationIndex,
        )
      }
    }
  }

  template += `>${context.childrenTemplate.join('')}`
  // TODO remove unnecessary close tag, e.g. if it's the last element of the template
  if (!isVoidTag(tag)) {
    template += `</${tag}>`
  }

  if (singleRoot) {
    context.ir.rootTemplateIndex = context.ir.templates.length
  }

  if (
    context.parent &&
    context.parent.node.type === 'JSXElement' &&
    context.parent.node.openingElement.name.type === 'JSXIdentifier' &&
    !isValidHTMLNesting(context.parent.node.openingElement.name.name, tag)
  ) {
    context.reference()
    context.dynamic.template = context.pushTemplate(template)
    context.dynamic.flags |= DynamicFlag.INSERT | DynamicFlag.NON_TEMPLATE
  } else {
    context.template += template
  }
}

export type PropsResult =
  | [dynamic: true, props: IRProps[], expressions: SimpleExpressionNode[]]
  | [dynamic: false, props: IRPropsStatic]

export function buildProps(
  node: JSXElement,
  context: TransformContext<JSXElement>,
  isComponent: boolean,
): PropsResult {
  const props = node.openingElement.attributes
  if (props.length === 0) return [false, []]

  const dynamicArgs: IRProps[] = []
  const dynamicExpr: SimpleExpressionNode[] = []
  let results: DirectiveTransformResult[] = []

  function pushMergeArg() {
    if (results.length) {
      dynamicArgs.push(dedupeProperties(results))
      results = []
    }
  }

  for (const prop of props) {
    if (prop.type === 'JSXSpreadAttribute' && prop.argument) {
      const value = resolveExpression(prop.argument, context)
      dynamicExpr.push(value)
      pushMergeArg()
      dynamicArgs.push({
        kind: IRDynamicPropsKind.EXPRESSION,
        value,
      })
      continue
    }

    const result = transformProp(prop, node, context)
    if (result) {
      dynamicExpr.push(result.key, result.value)
      if (isComponent && !result.key.isStatic) {
        // v-bind:[name]="value" or v-on:[name]="value"
        pushMergeArg()
        dynamicArgs.push(
          extend(resolveDirectiveResult(result), {
            kind: IRDynamicPropsKind.ATTRIBUTE,
          }) as IRPropsDynamicAttribute,
        )
      } else {
        // other static props
        results.push(result)
      }
    }
  }

  // has dynamic key or v-bind="{}"
  if (dynamicArgs.length || results.some(({ key }) => !key.isStatic)) {
    // take rest of props as dynamic props
    pushMergeArg()
    return [true, dynamicArgs, dynamicExpr]
  }

  const irProps = dedupeProperties(results)
  return [false, irProps]
}

function transformProp(
  prop: JSXAttribute | JSXSpreadAttribute,
  node: JSXElement,
  context: TransformContext<JSXElement>,
): DirectiveTransformResult | void {
  if (prop.type === 'JSXSpreadAttribute') return
  let name =
    prop.name.type === 'JSXIdentifier'
      ? prop.name.name
      : prop.name.type === 'JSXNamespacedName'
        ? prop.name.namespace.name
        : ''
  name = name.split('_')[0]

  if (
    !isDirectiveRegex.test(name) &&
    !isEventRegex.test(name) &&
    (!prop.value || prop.value.type === 'StringLiteral')
  ) {
    if (isReservedProp(name)) return
    return {
      key: resolveSimpleExpression(name, true, prop.name.loc!),
      value:
        prop.value && prop.value.type === 'StringLiteral'
          ? resolveSimpleExpression(prop.value.value, true, prop.value.loc!)
          : createSimpleExpression('true', false),
    }
  }

  name = isEventRegex.test(name)
    ? 'on'
    : isDirectiveRegex.test(name)
      ? name.slice(2)
      : 'bind'
  const directiveTransform = context.options.directiveTransforms[name]
  if (directiveTransform) {
    return directiveTransform(prop, node, context)
  }

  if (!isBuiltInDirective(name)) {
    const fromSetup = `v-${name}`
    if (fromSetup) {
      name = fromSetup
    } else {
      context.directive.add(name)
    }

    // TODO
    // context.registerOperation({
    //   type: IRNodeTypes.DIRECTIVE,
    //   element: context.reference(),
    //   dir: prop,
    //   name,
    //   asset: !fromSetup,
    // })
  }
}

// Dedupe props in an object literal.
// Literal duplicated attributes would have been warned during the parse phase,
// however, it's possible to encounter duplicated `onXXX` handlers with different
// modifiers. We also need to merge static and dynamic class / style attributes.
function dedupeProperties(results: DirectiveTransformResult[]): IRProp[] {
  const knownProps: Map<string, IRProp> = new Map()
  const deduped: IRProp[] = []

  for (const result of results) {
    const prop = resolveDirectiveResult(result)
    // dynamic keys are always allowed
    if (!prop.key.isStatic) {
      deduped.push(prop)
      continue
    }
    const name = prop.key.content
    const existing = knownProps.get(name)
    if (existing) {
      if (name === 'style' || name === 'class') {
        mergePropValues(existing, prop)
      }
      // unexpected duplicate, should have emitted error during parse
    } else {
      knownProps.set(name, prop)
      deduped.push(prop)
    }
  }
  return deduped
}

function resolveDirectiveResult(prop: DirectiveTransformResult): IRProp {
  return extend({}, prop, {
    value: undefined,
    values: [prop.value],
  })
}

function mergePropValues(existing: IRProp, incoming: IRProp) {
  const newValues = incoming.values
  existing.values.push(...newValues)
}
