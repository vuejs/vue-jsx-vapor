import {
  EffectScope,
  Fragment,
  getCurrentInstance,
  type Block,
  type ComponentObjectPropsOptions,
  type ComponentTypeEmits,
  type EmitFn,
  type EmitsOptions,
  type ExtractDefaultPropTypes,
  type ExtractPropTypes,
  type StaticSlots,
  type TypeEmitsToOptions,
  type VaporComponent,
  type VaporComponentInstance,
  type VaporComponentInstanceConstructor,
  type VaporComponentOptions,
  type VaporPublicProps,
  type VaporRenderResult,
  type VNode,
} from 'vue'
import * as Vue from 'vue'
import type {
  IsKeyValues,
  Prettify,
  ResolvePropsWithSlots,
  ToResolvedProps,
} from './types'

// component

/*@__NO_SIDE_EFFECTS__*/
export function defineVaporSSRComponent(
  comp: VaporComponent,
  extraOptions: VaporComponent,
): VaporComponent {
  if (typeof comp === 'function') {
    return Object.assign({ name: comp.name }, extraOptions, {
      setup(props: any, ctx: any) {
        // @ts-ignore
        const result = comp(props, ctx)
        return () => result
      },
      __vapor: true,
    })
  }
  const setup = comp.setup
  if (setup) {
    comp.setup = (props, ctx) => {
      const result = setup(props, ctx)
      return () => result
    }
  }
  comp.__vapor = true
  return comp
}

type Tail<T extends any[]> = T extends [any, ...infer R] ? R : never

export const createComponent = (
  type: VaporComponent | typeof Fragment | string,
  ...args: Tail<Parameters<typeof Vue.createComponent>>
) => {
  if (type === Fragment) {
    const slots = args[1]
    return slots && typeof slots.default === 'function' ? slots.default() : []
  }
  return Vue.createComponentWithFallback(
    createProxyComponent(Vue.resolveDynamicComponent(type) as VaporComponent),
    ...args,
  )
}

const proxyCache = new WeakMap()
export function createProxyComponent(
  type: VaporComponent,
  normalizeNode?: (node: any) => Block,
) {
  if (typeof type === 'function') {
    const existing = proxyCache.get(type)
    if (existing) return existing

    // @ts-ignore
    const i = Vue.currentInstance || getCurrentInstance()
    const proxy = new Proxy(type, {
      apply(target, ctx, args) {
        // @ts-ignore
        if (typeof target.__setup === 'function') {
          // @ts-ignore
          target.__setup.apply(ctx, args)
        }
        const node = Reflect.apply(target, ctx, args)
        return normalizeNode ? normalizeNode(node) : node
      },
      get(target, p, receiver) {
        if (i && i.appContext.vapor && p === '__vapor') {
          return true
        }
        return Reflect.get(target, p, receiver)
      },
    })
    proxyCache.set(type, proxy)
    return proxy
  }
  return type
}

// block

type NodeChildAtom =
  | VNode
  | Block
  | string
  | number
  | boolean
  | null
  | undefined
  | void
  | (() => NodeChild)

export type NodeArrayChildren = Array<NodeArrayChildren | NodeChildAtom>

export type NodeChild = NodeChildAtom | NodeArrayChildren

export function normalizeNode(node: NodeChild): Block {
  if (node == null || typeof node === 'boolean') {
    return document.createComment('')
  } else if (Array.isArray(node) && node.length) {
    return node.map(normalizeNode)
  } else if (isBlock(node)) {
    return node
  } else if (typeof node === 'function') {
    return resolveValues([node], undefined, true)[0]
  } else {
    // strings and numbers
    return document.createTextNode(String(node))
  }
}

export function isBlock(val: NonNullable<unknown>): val is Block {
  return (
    val instanceof Node ||
    Array.isArray(val) ||
    Vue.isVaporComponent(val) ||
    Vue.isFragment(val)
  )
}

// node

function createFragment(
  nodes: Block,
  anchor: Node | undefined = document.createTextNode(''),
) {
  const frag = new Vue.VaporFragment(nodes)
  frag.anchor = anchor
  return frag
}

function normalizeBlock(
  node: any,
  anchor?: Node,
  processFunction = false,
): Block {
  if (node instanceof Node || Vue.isFragment(node)) {
    return node
  } else if (Vue.isVaporComponent(node)) {
    return createFragment(node, anchor)
  } else if (Array.isArray(node)) {
    return createFragment(
      node.map((i) => normalizeBlock(i, undefined, processFunction)),
      anchor,
    )
  } else if (processFunction && typeof node === 'function') {
    return resolveValues([node], anchor, true)[0]
  } else {
    const result = node == null || typeof node === 'boolean' ? '' : String(node)
    if (anchor) {
      anchor.textContent = result
      return anchor
    } else {
      return document.createTextNode(result)
    }
  }
}

function resolveValue(
  current: Block | undefined,
  value: any,
  anchor?: Node,
  processFunction = false,
) {
  anchor =
    anchor ||
    (current instanceof Node && current.nodeType === 3 ? current : undefined)
  const node = normalizeBlock(value, anchor, processFunction)
  if (current) {
    if (Vue.isFragment(current)) {
      if (current.anchor && current.anchor.parentNode) {
        Vue.remove(current.nodes, current.anchor.parentNode)
        Vue.insert(node, current.anchor.parentNode, current.anchor)
        if (!anchor) current.anchor.parentNode.removeChild(current.anchor)
        // @ts-ignore
        if (current.scope) current.scope.stop()
      }
    } else if (current instanceof Node) {
      if (
        current.nodeType === 3 &&
        (!(node instanceof Node) || node.nodeType !== 3)
      ) {
        current.textContent = ''
      }
      if (Vue.isFragment(node) && current.parentNode) {
        Vue.insert(node, current.parentNode, current)
        if (!anchor || current.nodeType !== 3) {
          current.parentNode.removeChild(current)
        }
      } else if (node instanceof Node) {
        if (current.nodeType === 3 && node.nodeType === 3) {
          current.textContent = node.textContent
          return current
        } else if (current.parentNode) {
          current.parentNode.replaceChild(node, current)
        }
      }
    }
  }
  return node
}

function resolveValues(
  values: any[] = [],
  _anchor?: Node,
  processFunction = false,
) {
  const nodes: Block[] = []
  const scopes: EffectScope[] = []
  for (const [index, value] of values.entries()) {
    const anchor = index === values.length - 1 ? _anchor : undefined
    if (typeof value === 'function') {
      Vue.renderEffect(() => {
        if (scopes[index]) scopes[index].stop()
        scopes[index] = new EffectScope()
        nodes[index] = scopes[index].run(() =>
          resolveValue(nodes[index], value(), anchor, processFunction),
        )!
      })
    } else {
      nodes[index] = resolveValue(nodes[index], value, anchor, processFunction)
    }
  }
  return nodes
}

export function setNodes(anchor: Node, ...values: any[]) {
  const resolvedValues = resolveValues(values, anchor)
  if (anchor.parentNode) Vue.insert(resolvedValues, anchor.parentNode, anchor)
}

export function createNodes(...values: any[]) {
  return resolveValues(values)
}

// defineVaporComponent

export type DefineVaporComponent<
  RuntimePropsOptions = {},
  RuntimePropsKeys extends string = string,
  InferredProps = string extends RuntimePropsKeys
    ? ComponentObjectPropsOptions extends RuntimePropsOptions
      ? {}
      : ExtractPropTypes<RuntimePropsOptions>
    : { [key in RuntimePropsKeys]?: any },
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
  TypeRefs extends Record<string, unknown> = {},
  MakeDefaultsOptional extends boolean = true,
  PublicProps = VaporPublicProps,
  ResolvedProps = ToResolvedProps<InferredProps, Emits>,
  Defaults = ExtractDefaultPropTypes<RuntimePropsOptions>,
> = VaporComponentInstanceConstructor<
  VaporComponentInstance<
    ResolvePropsWithSlots<
      MakeDefaultsOptional extends true
        ? keyof Defaults extends never
          ? Prettify<ResolvedProps> & PublicProps
          : Partial<Defaults> &
              Omit<Prettify<ResolvedProps> & PublicProps, keyof Defaults>
        : Prettify<ResolvedProps> & PublicProps,
      Slots,
      NodeChild
    >,
    Emits,
    Slots,
    Exposed,
    TypeBlock,
    TypeRefs
  >
> &
  VaporComponentOptions<
    RuntimePropsOptions | RuntimePropsKeys[],
    Emits,
    RuntimeEmitsKeys,
    Slots,
    Exposed
  >

export type DefineVaporSetupFnComponent<
  Props extends Record<string, any> = {},
  Emits extends EmitsOptions = {},
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
  ResolvedProps extends Record<string, any> = ToResolvedProps<
    Props & VaporPublicProps,
    Emits
  >,
> = new () => VaporComponentInstance<
  ResolvePropsWithSlots<ResolvedProps, Slots, NodeChild>,
  Emits,
  Slots,
  Exposed,
  TypeBlock
>

// overload 1: direct setup function
// (uses user defined props interface)
export function defineVaporComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
>(
  setup: (
    this: void,
    props: Props,
    ctx: {
      emit: EmitFn<Emits>
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed: Exposed) => void
    },
  ) => VaporRenderResult<TypeBlock> | void,
  extraOptions?: VaporComponentOptions<
    (keyof Props)[],
    Emits,
    RuntimeEmitsKeys,
    Slots,
    Exposed
  > &
    ThisType<void>,
): DefineVaporSetupFnComponent<Props, Emits, Slots, Exposed, TypeBlock>
export function defineVaporComponent<
  Props extends Record<string, any>,
  Emits extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  TypeBlock extends Block = Block,
>(
  this: void,
  setup: (
    props: Props,
    ctx: {
      emit: EmitFn<Emits>
      slots: Slots
      attrs: Record<string, any>
      expose: (exposed: Exposed) => void
    },
  ) => VaporRenderResult<TypeBlock> | void,
  extraOptions?: VaporComponentOptions<
    ComponentObjectPropsOptions<Props>,
    Emits,
    RuntimeEmitsKeys,
    Slots,
    Exposed
  > &
    ThisType<void>,
): DefineVaporSetupFnComponent<Props, Emits, Slots, Exposed, TypeBlock>

// overload 2: defineVaporComponent with options object, infer props from options
export function defineVaporComponent<
  // props
  TypeProps,
  RuntimePropsOptions extends
    ComponentObjectPropsOptions = ComponentObjectPropsOptions,
  RuntimePropsKeys extends string = string,
  // emits
  TypeEmits extends ComponentTypeEmits = {},
  RuntimeEmitsOptions extends EmitsOptions = {},
  RuntimeEmitsKeys extends string = string,
  Slots extends StaticSlots = StaticSlots,
  Exposed extends Record<string, any> = Record<string, any>,
  // resolved types
  ResolvedEmits extends EmitsOptions = {} extends RuntimeEmitsOptions
    ? TypeEmitsToOptions<TypeEmits>
    : RuntimeEmitsOptions,
  InferredProps = IsKeyValues<TypeProps> extends true
    ? TypeProps
    : string extends RuntimePropsKeys
      ? ComponentObjectPropsOptions extends RuntimePropsOptions
        ? {}
        : ExtractPropTypes<RuntimePropsOptions>
      : { [key in RuntimePropsKeys]?: any },
  TypeRefs extends Record<string, unknown> = {},
  TypeBlock extends Block = Block,
>(
  options: VaporComponentOptions<
    RuntimePropsOptions | RuntimePropsKeys[],
    ResolvedEmits,
    RuntimeEmitsKeys,
    Slots,
    Exposed,
    TypeBlock,
    InferredProps
  > & {
    // allow any custom options
    [key: string]: any
    /**
     * @private
     */
    __typeProps?: TypeProps
    /**
     * @private
     */
    __typeEmits?: TypeEmits
    /**
     * @private
     */
    __typeRefs?: TypeRefs
    /**
     * @private
     */
    __typeEl?: TypeBlock
  } & ThisType<void>,
): DefineVaporComponent<
  RuntimePropsOptions,
  RuntimePropsKeys,
  InferredProps,
  ResolvedEmits,
  RuntimeEmitsKeys,
  Slots,
  Exposed extends Block ? Record<string, any> : Exposed,
  TypeBlock,
  TypeRefs,
  // MakeDefaultsOptional - if TypeProps is provided, set to false to use
  // user props types verbatim
  unknown extends TypeProps ? true : false
>

/*@__NO_SIDE_EFFECTS__*/
export function defineVaporComponent(comp: any, extraOptions?: any) {
  if (typeof comp === 'function') {
    return Object.assign({ name: comp.name }, extraOptions, {
      setup: comp,
      __vapor: true,
    })
  }
  comp.__vapor = true
  return comp
}
