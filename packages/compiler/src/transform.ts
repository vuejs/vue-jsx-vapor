import { extend, isArray, isString, NOOP } from '@vue/shared'
import {
  DynamicFlag,
  IRNodeTypes,
  type BlockIRNode,
  type IRDynamicInfo,
  type IRSlots,
  type OperationNode,
  type RootIRNode,
  type RootNode,
  type SetEventIRNode,
} from './ir'
import {
  findProp,
  getText,
  isConstantExpression,
  isConstantNode,
  isTemplate,
  newBlock,
  newDynamic,
  type CompilerError,
  type SimpleExpressionNode,
} from './utils'
import type { CodegenOptions } from './generate'
import type { JSXAttribute, JSXElement, JSXFragment } from '@babel/types'

export type NodeTransform = (
  node: BlockIRNode['node'],
  context: TransformContext<BlockIRNode['node']>,
) => void | (() => void) | (() => void)[]

export type DirectiveTransform = (
  dir: JSXAttribute,
  node: JSXElement,
  context: TransformContext<JSXElement>,
) => DirectiveTransformResult | void

export interface DirectiveTransformResult {
  key: SimpleExpressionNode
  value: SimpleExpressionNode
  modifier?: '.' | '^'
  runtimeCamelize?: boolean
  handler?: boolean
  handlerModifiers?: SetEventIRNode['modifiers']
  model?: boolean
  modelModifiers?: string[]
}

// A structural directive transform is technically also a NodeTransform;
// Only v-if and v-for fall into this category.
export type StructuralDirectiveTransform = (
  node: JSXElement,
  dir: JSXAttribute,
  context: TransformContext,
) => void | (() => void)

export type TransformOptions = CodegenOptions & {
  source?: string
  /**
   * Whether to compile components to createComponentWithFallback.
   * @default false
   */
  withFallback?: boolean
  /**
   * Indicates that transforms and codegen should try to output valid TS code
   */
  isTS?: boolean
  /**
   * An array of node transforms to be applied to every AST node.
   */
  nodeTransforms?: NodeTransform[]
  /**
   * An object of { name: transform } to be applied to every directive attribute
   * node found on element nodes.
   */
  directiveTransforms?: Record<string, DirectiveTransform | undefined>
  /**
   * Separate option for end users to extend the native elements list
   */
  isCustomElement?: (tag: string) => boolean | void
  onError?: (error: CompilerError) => void
}
const defaultOptions: Required<TransformOptions> = {
  source: '',
  sourceMap: false,
  filename: 'index.jsx',
  nodeTransforms: [],
  directiveTransforms: {},
  templates: [],
  isCustomElement: NOOP,
  expressionPlugins: [],
  isTS: false,
  withFallback: false,
  onError: (error: CompilerError) => {
    throw error
  },
}

export class TransformContext<
  T extends BlockIRNode['node'] = BlockIRNode['node'],
> {
  parent: TransformContext<RootNode | JSXElement | JSXFragment> | null = null
  root: TransformContext<RootNode>
  index: number = 0

  block: BlockIRNode
  options: Required<TransformOptions>

  template: string = ''
  childrenTemplate: (string | null)[] = []
  dynamic: IRDynamicInfo

  inVOnce: boolean = false
  inVFor: number = 0

  component: Set<string>
  directive: Set<string>

  slots: IRSlots[] = []

  private globalId = 0

  constructor(
    public ir: RootIRNode,
    public node: T,
    options: TransformOptions = {},
  ) {
    this.options = extend({}, defaultOptions, options)
    this.block = this.ir.block
    this.dynamic = this.ir.block.dynamic
    this.component = this.ir.component
    this.directive = this.ir.directive
    this.root = this as TransformContext<RootNode>
  }

  enterBlock(ir: BlockIRNode, isVFor: boolean = false): () => void {
    const { block, template, dynamic, childrenTemplate, slots } = this
    this.block = ir
    this.dynamic = ir.dynamic
    this.template = ''
    this.childrenTemplate = []
    this.slots = []
    isVFor && this.inVFor++
    return () => {
      // exit
      this.registerTemplate()
      this.block = block
      this.template = template
      this.dynamic = dynamic
      this.childrenTemplate = childrenTemplate
      this.slots = slots
      isVFor && this.inVFor--
    }
  }

  increaseId = () => this.globalId++
  reference() {
    if (this.dynamic.id !== undefined) return this.dynamic.id
    this.dynamic.flags |= DynamicFlag.REFERENCED
    return (this.dynamic.id = this.increaseId())
  }

  pushTemplate(content: string) {
    const existing = this.ir.templates.indexOf(content)
    if (existing !== -1) return existing
    this.ir.templates.push(content)
    return this.ir.templates.length - 1
  }

  registerTemplate() {
    if (!this.template) return -1
    const id = this.pushTemplate(this.template)
    return (this.dynamic.template = id)
  }

  registerEffect(
    expressions: SimpleExpressionNode[],
    operation: OperationNode | OperationNode[],
    getEffectIndex = (): number => this.block.effect.length,
    getOperationIndex = (): number => this.block.operation.length,
  ) {
    const operations = [operation].flat()
    expressions = expressions.filter((exp) => !isConstantExpression(exp))
    if (
      this.inVOnce ||
      expressions.length === 0 ||
      expressions.every((e) => e.ast && isConstantNode(e.ast))
    ) {
      return this.registerOperation(operations, getOperationIndex)
    }

    this.block.effect.splice(getEffectIndex(), 0, {
      expressions,
      operations,
    })
  }

  registerOperation(
    operation: OperationNode | OperationNode[],
    getOperationIndex = (): number => this.block.operation.length,
  ) {
    this.block.operation.splice(getOperationIndex(), 0, ...[operation].flat())
  }

  create<E extends T>(node: E, index: number): TransformContext<E> {
    return Object.assign(Object.create(TransformContext.prototype), this, {
      node,
      parent: this as any,
      index,

      template: '',
      childrenTemplate: [],
      dynamic: newDynamic(),
    } satisfies Partial<TransformContext<T>>)
  }
}

// AST -> IR
export function transform(
  node: RootNode,
  options: TransformOptions = {},
): RootIRNode {
  const ir: RootIRNode = {
    type: IRNodeTypes.ROOT,
    node,
    source: node.source,
    templates: options.templates || [],
    component: new Set(),
    directive: new Set(),
    block: newBlock(node),
    hasTemplateRef: false,
  }

  const context = new TransformContext(ir, node, options)

  transformNode(context)

  return ir
}

export function transformNode(context: TransformContext<BlockIRNode['node']>) {
  let { node } = context

  // apply transform plugins
  const { nodeTransforms } = context.options
  const exitFns = []
  for (const nodeTransform of nodeTransforms) {
    const onExit = nodeTransform(node, context)
    if (onExit) {
      if (isArray(onExit)) {
        exitFns.push(...onExit)
      } else {
        exitFns.push(onExit)
      }
    }
    if (!context.node) {
      // node was removed
      return
    } else {
      // node may have been replaced
      node = context.node
    }
  }

  // exit transforms
  context.node = node
  let i = exitFns.length
  while (i--) {
    exitFns[i]()
  }

  if (context.node.type === IRNodeTypes.ROOT) {
    context.registerTemplate()
  }
}

export function createStructuralDirectiveTransform(
  name: string | string[],
  fn: StructuralDirectiveTransform,
): NodeTransform {
  const matches = (n: string) =>
    isString(name) ? n === name : name.includes(n)

  return (node, context) => {
    if (node.type === 'JSXElement') {
      const {
        openingElement: { attributes },
      } = node
      // structural directive transforms are not concerned with slots
      // as they are handled separately in vSlot.ts
      if (isTemplate(node) && findProp(node, 'v-slot')) {
        return
      }
      const exitFns = []
      for (const prop of attributes) {
        if (prop.type !== 'JSXAttribute') continue
        const propName = getText(prop.name, context)
        if (propName.startsWith('v-') && matches(propName.slice(2))) {
          attributes.splice(attributes.indexOf(prop), 1)
          const onExit = fn(node, prop, context as TransformContext)
          if (onExit) exitFns.push(onExit)
          break
        }
      }
      return exitFns
    }
  }
}
