import {
  createCompilerError,
  createSimpleExpression,
  ErrorCodes,
  resolveModifiers,
} from '@vue/compiler-dom'
import { extend, makeMap } from '@vue/shared'
import { IRNodeTypes, type KeyOverride, type SetEventIRNode } from '../ir'
import {
  isJSXComponent,
  resolveExpression,
  resolveLocation,
  resolveSimpleExpression,
} from '../utils'
import type { DirectiveTransform } from '../transform'
import { EMPTY_EXPRESSION } from './utils'

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
      createCompilerError(
        ErrorCodes.X_V_ON_NO_EXPRESSION,
        resolveLocation(loc, context),
      ),
    )
  }

  let arg = resolveSimpleExpression(nameString, true, dir.name.loc)
  const exp = resolveExpression(dir.value, context)

  const { keyModifiers, nonKeyModifiers, eventOptionModifiers } =
    resolveModifiers(
      arg.isStatic ? `on${nameString}` : arg,
      modifiers.map((modifier) => createSimpleExpression(modifier)),
      null,
      resolveLocation(loc, context),
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
