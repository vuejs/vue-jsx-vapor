import {
  createCompilerError,
  ErrorCodes,
  type SimpleExpressionNode,
} from '@vue/compiler-dom'
import {
  DynamicFlag,
  IRSlotType,
  type IRFor,
  type IRSlotDynamic,
  type IRSlotDynamicBasic,
  type IRSlotDynamicConditional,
  type IRSlots,
  type IRSlotsStatic,
  type SlotBlockIRNode,
  type VaporDirectiveNode,
} from '../ir'
import {
  findProp,
  isEmptyText,
  isJSXComponent,
  isTemplate,
  resolveDirective,
  resolveLocation,
  resolveSimpleExpressionNode,
} from '../utils'
import type { NodeTransform, TransformContext } from '../transform'
import { newBlock } from './utils'
import { getForParseResult } from './vFor'
import type { JSXElement } from '@babel/types'

export const transformVSlot: NodeTransform = (node, context) => {
  if (node.type !== 'JSXElement') return

  const { children } = node

  const dir = findProp(node, 'v-slot')
  const resolvedDirective = dir
    ? resolveDirective(dir, context, true)
    : undefined
  const { parent } = context

  const isComponent = isJSXComponent(node)
  const isSlotTemplate =
    isTemplate(node) &&
    parent &&
    parent.node.type === 'JSXElement' &&
    isJSXComponent(parent.node)

  if (isComponent && children.length) {
    return transformComponentSlot(node, resolvedDirective, context)
  } else if (isSlotTemplate && resolvedDirective) {
    return transformTemplateSlot(node, resolvedDirective, context)
  } else if (!isComponent && dir) {
    context.options.onError(
      createCompilerError(
        ErrorCodes.X_V_SLOT_MISPLACED,
        resolveLocation(dir.loc, context),
      ),
    )
  }
}

// <Foo v-slot:default>
function transformComponentSlot(
  node: JSXElement,
  dir: VaporDirectiveNode | undefined,
  context: TransformContext,
) {
  const { children } = node
  const arg = dir && dir.arg
  const nonSlotTemplateChildren = children.filter(
    (n) =>
      !isEmptyText(n) && (n.type !== 'JSXElement' || !findProp(n, 'v-slot')),
  )

  const [block, onExit] = createSlotBlock(node, dir, context)

  const { slots } = context

  return () => {
    onExit()

    const hasOtherSlots = !!slots.length
    if (dir && hasOtherSlots) {
      // already has on-component slot - this is incorrect usage.
      context.options.onError(
        createCompilerError(ErrorCodes.X_V_SLOT_MIXED_SLOT_USAGE, dir.loc),
      )
      return
    }

    if (nonSlotTemplateChildren.length) {
      if (hasStaticSlot(slots, 'default')) {
        context.options.onError(
          createCompilerError(
            ErrorCodes.X_V_SLOT_EXTRANEOUS_DEFAULT_SLOT_CHILDREN,
            resolveLocation(nonSlotTemplateChildren[0].loc, context),
          ),
        )
      } else {
        registerSlot(slots, arg, block)
        context.slots = slots
      }
    } else if (hasOtherSlots) {
      context.slots = slots
    }
  }
}

const elseIfRE = /^v-else(-if)?$/
// <template v-slot:foo>
function transformTemplateSlot(
  node: JSXElement,
  dir: VaporDirectiveNode,
  context: TransformContext,
) {
  context.dynamic.flags |= DynamicFlag.NON_TEMPLATE

  const arg = dir.arg && resolveSimpleExpressionNode(dir.arg)
  const vFor = findProp(node, 'v-for')
  const vIf = findProp(node, 'v-if')
  const vElse = findProp(node, elseIfRE)
  const { slots } = context
  const [block, onExit] = createSlotBlock(node, dir, context)

  if (!vFor && !vIf && !vElse) {
    const slotName = arg ? arg.isStatic && arg.content : 'default'
    if (slotName && hasStaticSlot(slots, slotName)) {
      context.options.onError(
        createCompilerError(ErrorCodes.X_V_SLOT_DUPLICATE_SLOT_NAMES, dir.loc),
      )
    } else {
      registerSlot(slots, arg, block)
    }
  } else if (vIf) {
    const vIfDir = resolveDirective(vIf, context)
    registerDynamicSlot(slots, {
      slotType: IRSlotType.CONDITIONAL,
      condition: vIfDir.exp!,
      positive: {
        slotType: IRSlotType.DYNAMIC,
        name: arg!,
        fn: block,
      },
    })
  } else if (vElse) {
    const vElseDir = resolveDirective(vElse, context)
    const vIfSlot = slots.at(-1) as IRSlotDynamic
    if (vIfSlot.slotType === IRSlotType.CONDITIONAL) {
      let ifNode = vIfSlot
      while (
        ifNode.negative &&
        ifNode.negative.slotType === IRSlotType.CONDITIONAL
      )
        ifNode = ifNode.negative
      const negative: IRSlotDynamicBasic | IRSlotDynamicConditional =
        vElseDir.exp
          ? {
              slotType: IRSlotType.CONDITIONAL,
              condition: vElseDir.exp,
              positive: {
                slotType: IRSlotType.DYNAMIC,
                name: arg!,
                fn: block,
              },
            }
          : {
              slotType: IRSlotType.DYNAMIC,
              name: arg!,
              fn: block,
            }
      ifNode.negative = negative
    } else {
      context.options.onError(
        createCompilerError(ErrorCodes.X_V_ELSE_NO_ADJACENT_IF, vElseDir.loc),
      )
    }
  } else if (vFor) {
    const forParseResult = getForParseResult(vFor, context)
    if (forParseResult.source) {
      registerDynamicSlot(slots, {
        slotType: IRSlotType.LOOP,
        name: arg!,
        fn: block,
        loop: forParseResult as IRFor,
      })
    }
  }

  return onExit
}

export function ensureStaticSlots(slots: IRSlots[]): IRSlotsStatic['slots'] {
  let lastSlots = slots.at(-1)!
  if (!slots.length || lastSlots.slotType !== IRSlotType.STATIC) {
    slots.push(
      (lastSlots = {
        slotType: IRSlotType.STATIC,
        slots: {},
      }),
    )
  }
  return lastSlots.slots
}

function registerSlot(
  slots: IRSlots[],
  name: SimpleExpressionNode | undefined,
  block: SlotBlockIRNode,
) {
  const isStatic = !name || name.isStatic
  if (isStatic) {
    const staticSlots = ensureStaticSlots(slots)
    staticSlots[name ? name.content : 'default'] = block
  } else {
    slots.push({
      slotType: IRSlotType.DYNAMIC,
      name: name!,
      fn: block,
    })
  }
}

function registerDynamicSlot(allSlots: IRSlots[], dynamic: IRSlotDynamic) {
  allSlots.push(dynamic)
}

function hasStaticSlot(slots: IRSlots[], name: string) {
  return slots.some((slot) =>
    slot.slotType === IRSlotType.STATIC ? !!slot.slots[name] : false,
  )
}

function createSlotBlock(
  slotNode: JSXElement,
  dir: VaporDirectiveNode | undefined,
  context: TransformContext,
): [SlotBlockIRNode, () => void] {
  const block: SlotBlockIRNode = newBlock(slotNode)
  block.props = dir && dir.exp
  const exitBlock = context.enterBlock(block)
  return [block, exitBlock]
}
