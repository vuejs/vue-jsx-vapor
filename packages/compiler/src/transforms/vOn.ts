import { extend, isString, makeMap } from '@vue/shared'
import { IRNodeTypes, type KeyOverride, type SetEventIRNode } from '../ir'
import {
  createCompilerError,
  createSimpleExpression,
  EMPTY_EXPRESSION,
  ErrorCodes,
  isJSXComponent,
  resolveExpression,
  resolveSimpleExpression,
  type SimpleExpressionNode,
} from '../utils'
import type { DirectiveTransform } from '../transform'

const delegatedEvents = /*#__PURE__*/ makeMap(
  'beforeinput,click,dblclick,contextmenu,focusin,focusout,input,keydown,' +
    'keyup,mousedown,mousemove,mouseout,mouseover,mouseup,pointerdown,' +
    'pointermove,pointerout,pointerover,pointerup,touchend,touchmove,' +
    'touchstart',
)

export const transformVOn: DirectiveTransform = (dir, node, context) => {
  const { name, loc, value } = dir
  if (!name) return
  const isComponent = isJSXComponent(node)

  const [nameString, ...modifiers] = context.ir.source
    .slice(name.start!, name.end!)
    .replace(/^on([A-Z])/, (_, $1) => $1.toLowerCase())
    .split('_')

  if (!value && !modifiers.length) {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_ON_NO_EXPRESSION, loc),
    )
  }

  let arg = resolveSimpleExpression(nameString, true, dir.name.loc)
  const exp = resolveExpression(dir.value, context)

  const { keyModifiers, nonKeyModifiers, eventOptionModifiers } =
    resolveModifiers(
      arg.isStatic ? `on${nameString}` : arg,
      modifiers.map((modifier) => createSimpleExpression(modifier)),
    )

  let keyOverride: KeyOverride | undefined
  const isStaticClick = arg.isStatic && arg.content.toLowerCase() === 'click'

  // normalize click.right and click.middle since they don't actually fire
  if (nonKeyModifiers.includes('middle')) {
    if (keyOverride) {
      // TODO error here
    }
    if (isStaticClick) {
      arg = extend({}, arg, { content: 'mouseup' })
    } else if (!arg.isStatic) {
      keyOverride = ['click', 'mouseup']
    }
  }
  if (nonKeyModifiers.includes('right')) {
    if (isStaticClick) {
      arg = extend({}, arg, { content: 'contextmenu' })
    } else if (!arg.isStatic) {
      keyOverride = ['click', 'contextmenu']
    }
  }

  if (isComponent) {
    const handler = exp || EMPTY_EXPRESSION
    return {
      key: arg,
      value: handler,
      handler: true,
      handlerModifiers: {
        keys: keyModifiers,
        nonKeys: nonKeyModifiers,
        options: eventOptionModifiers,
      },
    }
  }

  // Only delegate if:
  // - no dynamic event name
  // - no event option modifiers (passive, capture, once)
  // - is a delegatable event
  const delegate =
    arg.isStatic && !eventOptionModifiers.length && delegatedEvents(arg.content)

  const operation: SetEventIRNode = {
    type: IRNodeTypes.SET_EVENT,
    element: context.reference(),
    key: arg,
    value: exp,
    modifiers: {
      keys: keyModifiers,
      nonKeys: nonKeyModifiers,
      options: eventOptionModifiers,
    },
    keyOverride,
    delegate,
    effect: !arg.isStatic,
  }

  context.registerEffect([arg], operation)
}

const isEventOptionModifier = /*@__PURE__*/ makeMap(`passive,once,capture`)
const isNonKeyModifier = /*@__PURE__*/ makeMap(
  // event propagation management
  `stop,prevent,self,` +
    // system modifiers + exact
    `ctrl,shift,alt,meta,exact,` +
    // mouse
    `middle`,
)
// left & right could be mouse or key modifiers based on event type
const maybeKeyModifier = /*@__PURE__*/ makeMap('left,right')
const isKeyboardEvent = /*@__PURE__*/ makeMap(`onkeyup,onkeydown,onkeypress`)

export const resolveModifiers = (
  key: SimpleExpressionNode | string,
  modifiers: SimpleExpressionNode[],
): {
  keyModifiers: string[]
  nonKeyModifiers: string[]
  eventOptionModifiers: string[]
} => {
  const keyModifiers = []
  const nonKeyModifiers = []
  const eventOptionModifiers = []

  for (const modifier_ of modifiers) {
    const modifier = modifier_.content

    if (isEventOptionModifier(modifier)) {
      // eventOptionModifiers: modifiers for addEventListener() options,
      // e.g. .passive & .capture
      eventOptionModifiers.push(modifier)
    } else {
      const keyString = isString(key) ? key : key.isStatic ? key.content : null

      // runtimeModifiers: modifiers that needs runtime guards
      if (maybeKeyModifier(modifier)) {
        if (keyString) {
          if (isKeyboardEvent(keyString.toLowerCase())) {
            keyModifiers.push(modifier)
          } else {
            nonKeyModifiers.push(modifier)
          }
        } else {
          keyModifiers.push(modifier)
          nonKeyModifiers.push(modifier)
        }
      } else if (isNonKeyModifier(modifier)) {
        nonKeyModifiers.push(modifier)
      } else {
        keyModifiers.push(modifier)
      }
    }
  }

  return {
    keyModifiers,
    nonKeyModifiers,
    eventOptionModifiers,
  }
}
