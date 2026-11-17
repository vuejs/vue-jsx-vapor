// @vitest-environment happy-dom
//
// Runtime regression test for the camelCase event modifier aliases
// (`Capture` / `Once` / `Passive`) supported by the vapor `h()` helper.
//
// `h()` hands the raw prop key to the vapor runtime, which strips the
// trailing `Once|Passive|Capture` and forwards it to `addEventListener` as
// native options. The runtime half of the camelCase aliases therefore needs
// no code here — what the aliases add is the type side (see
// `event-modifiers.test-d.tsx`). These tests pin the runtime contract those
// types rely on.

import { expect, test } from 'vite-plus/test'
import { createVaporApp } from 'vue'
import { vaporH as h } from '../src/h'

type ListenerCall = { type: string; options?: AddEventListenerOptions }

function mount(props: Record<string, any>): {
  el: HTMLElement
  listeners: ListenerCall[]
} {
  const calls: Array<ListenerCall & { target: Element; options?: AddEventListenerOptions }> = []
  const original = Element.prototype.addEventListener
  Element.prototype.addEventListener = function (
    this: Element,
    type: string,
    listener: any,
    options?: AddEventListenerOptions | boolean,
  ) {
    calls.push({
      target: this,
      type,
      options: options as AddEventListenerOptions | undefined,
    })
    return original.call(this, type, listener, options)
  }
  const root = document.createElement('div')
  try {
    createVaporApp({ setup: () => h('div', props) }).mount(root)
  } finally {
    Element.prototype.addEventListener = original
  }
  const el = root.firstElementChild as HTMLElement
  return {
    el,
    listeners: calls.filter((c) => c.target === el).map(({ type, options }) => ({ type, options })),
  }
}

test('h() attaches a plain listener for the base event', () => {
  const { listeners } = mount({ onClick: () => {} })
  expect(listeners).toEqual([{ type: 'click', options: undefined }])
})

test('h() attaches a capture-phase listener for onClickCapture', () => {
  const { listeners } = mount({ onClickCapture: () => {} })
  expect(listeners).toEqual([{ type: 'click', options: { capture: true } }])
})

test('h() attaches a once listener for onClickOnce', () => {
  const { listeners } = mount({ onClickOnce: () => {} })
  expect(listeners).toEqual([{ type: 'click', options: { once: true } }])
})

test('h() attaches a passive listener for onWheelPassive', () => {
  const { listeners } = mount({ onWheelPassive: () => {} })
  expect(listeners).toEqual([{ type: 'wheel', options: { passive: true } }])
})

// The raw prop key is what identifies the invoker, so the bubble and the
// capture listener are two separate entries and both stay attached.
test('h() keeps the bubble and the capture listener separate', () => {
  const { listeners } = mount({ onClick: () => {}, onClickCapture: () => {} })
  expect(listeners).toEqual([
    { type: 'click', options: undefined },
    { type: 'click', options: { capture: true } },
  ])
})

test('onClickOnce runs the handler only once', () => {
  let count = 0
  const { el } = mount({ onClickOnce: () => count++ })
  el.dispatchEvent(new Event('click'))
  el.dispatchEvent(new Event('click'))
  expect(count).toBe(1)
})

// A capture listener on an ancestor runs before a bubble listener on a
// descendant — the whole point of the `capture` option.
test('onClickCapture runs before a descendant bubble listener', () => {
  const order: string[] = []
  const { el } = mount({ onClickCapture: () => order.push('capture') })
  const child = document.createElement('span')
  child.addEventListener('click', () => order.push('bubble'))
  el.append(child)
  child.dispatchEvent(new Event('click', { bubbles: true }))
  expect(order).toEqual(['capture', 'bubble'])
})

test('onWheelPassive ignores preventDefault', () => {
  const { el } = mount({ onWheelPassive: (e: Event) => e.preventDefault() })
  const event = new Event('wheel', { cancelable: true })
  el.dispatchEvent(event)
  expect(event.defaultPrevented).toBe(false)
})

// The control for the test above: without `passive`, canceling still works.
test('a plain wheel listener can cancel the event', () => {
  const { el } = mount({ onWheel: (e: Event) => e.preventDefault() })
  const event = new Event('wheel', { cancelable: true })
  el.dispatchEvent(event)
  expect(event.defaultPrevented).toBe(true)
})

// ---------------------------------------------------------------------------
// Combined modifier suffixes — Vue runtime DOES support them.
//
// Vue's `parseEventName` uses a `while` loop (not a single `if`) with
// `optionsModifierRE = /(Once|Passive|Capture)$/`, so it strips multiple
// trailing modifiers and accumulates them into one options object:
//
//   `onClickCaptureOnce` → strip `Once` → strip `Capture` → `click` + `{once,capture}`
//
// This means the *runtime* handles combinations correctly. The *types*
// (in event-modifiers.test-d.tsx) do not generate combined aliases, so
// `onClickCaptureOnce` has no contextual typing — but it still works at
// runtime via RawProps' index signature.
// ---------------------------------------------------------------------------

test('onClickCaptureOnce splits into capture + once', () => {
  const { listeners } = mount({ onClickCaptureOnce: () => {} })
  expect(listeners).toEqual([{ type: 'click', options: { once: true, capture: true } }])
})

test('onWheelPassiveCapture splits into passive + capture', () => {
  const { listeners } = mount({ onWheelPassiveCapture: () => {} })
  expect(listeners).toEqual([{ type: 'wheel', options: { capture: true, passive: true } }])
})

test('onClickCaptureOnce fires once in capture phase', () => {
  let count = 0
  const { el } = mount({ onClickCaptureOnce: () => count++ })
  // Dispatch from a child to verify capture phase (ancestor fires first).
  const child = document.createElement('span')
  el.append(child)
  child.dispatchEvent(new Event('click', { bubbles: true }))
  child.dispatchEvent(new Event('click', { bubbles: true }))
  // `once` means only the first dispatch triggers.
  expect(count).toBe(1)
})
