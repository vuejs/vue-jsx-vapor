import { describe, expect, test, vi } from 'vitest'
import {
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
  transformVOn,
} from '../../src'
import { ErrorCodes } from '../../src/utils'
import { makeCompile } from './_utils'

const compileWithVOn = makeCompile({
  nodeTransforms: [transformElement, transformText, transformChildren],
  directiveTransforms: {
    on: transformVOn,
    bind: transformVBind,
  },
})

describe('v-on', () => {
  test('simple expression', () => {
    const { code, ir, helpers } = compileWithVOn(
      `<div onClick={handleClick}></div>`,
    )

    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = t0()
        n0.$evtclick = e => handleClick(e)
        return n0
      "
    `)
    expect(helpers).not.contains('delegate') // optimized as direct attachment
    expect(ir.block.effect).toEqual([])
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        element: 0,
        key: {
          content: 'click',
          isStatic: true,
        },
        value: {
          content: 'handleClick',
          isStatic: false,
        },
        modifiers: { keys: [], nonKeys: [], options: [] },
        keyOverride: undefined,
        delegate: true,
      },
    ])
  })

  test('event modifier', () => {
    const { code } = compileWithVOn(
      `<>
        <a onClick_stop={handleEvent}></a>
        <form onSubmit_prevent={handleEvent}></form>
        <a onClick_stop_prevent={handleEvent}></a>
        <div onClick_self={handleEvent}></div>
        <div onClick_capture={handleEvent}></div>
        <a onClick_once={handleEvent}></a>
        <div onScroll_passive={handleEvent}></div>
        <input onClick_right={handleEvent} />
        <input onClick_left={handleEvent} />
        <input onClick_middle={handleEvent} />
        <input onClick_enter_right={handleEvent} />
        <input onKeyup_enter={handleEvent} />
        <input onKeyup_tab={handleEvent} />
        <input onKeyup_delete={handleEvent} />
        <input onKeyup_esc={handleEvent} />
        <input onKeyup_space={handleEvent} />
        <input onKeyup_up={handleEvent} />
        <input onKeyup_down={handleEvent} />
        <input onKeyup_left={handleEvent} />
        <input onKeyup_middle={submit} />
        <input onKeyup_middle_self={submit} />
        <input onKeyup_self_enter={handleEvent} />
      </>`,
    )
    expect(code).matchSnapshot()
  })

  test('should error if no expression AND no modifier', () => {
    const onError = vi.fn()
    compileWithVOn(`<div onClick />`, { onError })
    expect(onError.mock.calls[0][0]).toMatchObject({
      code: ErrorCodes.X_V_ON_NO_EXPRESSION,
      loc: {
        start: {
          line: 1,
          column: 5,
        },
        end: {
          line: 1,
          column: 12,
        },
      },
    })
  })

  test('should NOT error if no expression but has modifier', () => {
    const onError = vi.fn()
    compileWithVOn(`<div onClick_prevent />`, { onError })
    expect(onError).not.toHaveBeenCalled()
  })

  test('should support multiple modifiers and event options', () => {
    const { code, ir, helpers } = compileWithVOn(
      `<div onClick_stop_prevent_capture_once={test}/>`,
    )

    expect(code).toMatchSnapshot()
    expect(helpers).contains('on')
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        value: {
          content: 'test',
          isStatic: false,
        },
        modifiers: {
          keys: [],
          nonKeys: ['stop', 'prevent'],
          options: ['capture', 'once'],
        },
        keyOverride: undefined,
        delegate: false,
      },
    ])
    expect(code).contains(
      `_on(n0, "click", _withModifiers(e => test(e), ["stop","prevent"]), {
    capture: true, 
    once: true
  })`,
    )
  })

  test('should support multiple events and modifiers options', () => {
    const { code, ir } = compileWithVOn(
      `<div onClick_stop={test} onKeyup_enter={test} />`,
    )

    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        key: {
          content: 'click',
          isStatic: true,
        },
        value: {
          content: 'test',
          isStatic: false,
        },
        modifiers: {
          keys: [],
          nonKeys: ['stop'],
          options: [],
        },
      },
      {
        type: IRNodeTypes.SET_EVENT,
        key: {
          content: 'keyup',
          isStatic: true,
        },
        value: {
          content: 'test',
          isStatic: false,
        },
        modifiers: {
          keys: ['enter'],
          nonKeys: [],
          options: [],
        },
      },
    ])

    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = t0()
        n0.$evtclick = _withModifiers(e => test(e), ["stop"])
        n0.$evtkeyup = _withKeys(e => test(e), ["enter"])
        return n0
      "
    `)
    expect(code).contains(
      `n0.$evtclick = _withModifiers(e => test(e), ["stop"])`,
    )
    expect(code).contains(`n0.$evtkeyup = _withKeys(e => test(e), ["enter"])`)
  })

  test('should wrap keys guard for keyboard events or dynamic events', () => {
    const { code, ir } = compileWithVOn(
      `<div onKeydown_stop_capture_ctrl_a={test}/>`,
    )

    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        element: 0,
        key: {
          content: 'keydown',
          isStatic: true,
        },
        value: {
          content: 'test',
          isStatic: false,
        },
        modifiers: {
          keys: ['a'],
          nonKeys: ['stop', 'ctrl'],
          options: ['capture'],
        },
      },
    ])

    expect(code).matchSnapshot()
  })

  test('should not wrap keys guard if no key modifier is present', () => {
    const { code, ir } = compileWithVOn(`<div onKeyup_exact={test}/>`)
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        modifiers: { nonKeys: ['exact'] },
      },
    ])

    expect(code).matchSnapshot()
  })

  test('should wrap keys guard for static key event w/ left/right modifiers', () => {
    const { code, ir } = compileWithVOn(`<div onKeyup_left={test}/>`)

    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        modifiers: {
          keys: ['left'],
          nonKeys: [],
          options: [],
        },
      },
    ])

    expect(code).matchSnapshot()
  })

  test('should transform click.right', () => {
    const { code, ir, delegates } = compileWithVOn(
      `<div onClick_right={test}/>`,
    )
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        key: {
          content: 'contextmenu',
          isStatic: true,
        },
        modifiers: { nonKeys: ['right'] },
        keyOverride: undefined,
      },
    ])

    expect(code).toMatchSnapshot()
    expect(delegates).includes('contextmenu')
  })

  test('should transform click.middle', () => {
    const { code, ir, delegates } = compileWithVOn(
      `<div onClick_middle={test}/>`,
    )
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        key: {
          content: 'mouseup',
          isStatic: true,
        },
        modifiers: { nonKeys: ['middle'] },
        keyOverride: undefined,
      },
    ])

    expect(code).matchSnapshot()
    expect(delegates).includes('mouseup')
  })

  test('should delegate event', () => {
    const { code, ir, helpers, delegates } = compileWithVOn(
      `<div onClick={test}/>`,
    )

    expect(code).matchSnapshot()
    expect(delegates).contains('click')
    expect(helpers).contains('delegateEvents')
    expect(ir.block.operation).toMatchObject([
      {
        type: IRNodeTypes.SET_EVENT,
        delegate: true,
      },
    ])
  })

  test('should use delegate helper when have multiple events of same name', () => {
    const { code, helpers } = compileWithVOn(
      `<div onClick={test} onClick_stop={test} />`,
    )
    expect(helpers).contains('delegate')
    expect(code).toMatchSnapshot()
    expect(code).contains('_delegate(n0, "click", e => test(e))')
    expect(code).contains(
      '_delegate(n0, "click", _withModifiers(e => test(e), ["stop"]))',
    )
  })

  test('namespace event with Component', () => {
    const { code } = compileWithVOn(`<Comp onUpdate:modelValue={() => {}} />`)
    expect(code).toMatchSnapshot()
    expect(code).contains(
      '_createComponent(Comp, { "onUpdate:modelValue": () => () => {} }, null, true)',
    )
  })

  test('expression with type', () => {
    const { code } = compileWithVOn(`<div onClick={handleClick as any} />`)
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = t0()
        n0.$evtclick = e => handleClick(e)
        return n0
      "
    `)
    expect(code).contains('n0.$evtclick = e => handleClick(e)')
  })
})
