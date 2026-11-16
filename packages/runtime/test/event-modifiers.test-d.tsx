// Typecheck-only regression test for camelCase event modifier aliases
// (`Capture` / `Once` / `Passive`) in the JSX transform path and the
// vapor `h()` helper.
// Exercised by `pnpm typecheck` (tsmc --noEmit); `@ts-expect-error` lines
// must keep erroring.

import { vaporH as h } from '../src/h'

// ---------------------------------------------------------------------------
// JSX-side type assertions
// ---------------------------------------------------------------------------

// camelCase aliases resolve to the base event's payload type.
const handlers = (
  <>
    <div onClick={(e) => e.preventDefault()} />
    <div onClickCapture={(e) => e.clientX} />
    <div onClickOnce={(e) => e.clientX} />
    <div onTouchstartPassive={(e) => e.touches} />
    <div onWheelCapture={(e) => e.deltaY} />
  </>
)

// Inline handler parameters keep contextual typing — these only error when
// `e` is typed as the event, not any.
const typed = (
  <div
    onClickCapture={(e) => {
      // @ts-expect-error - clientX is a number
      const s: string = e.clientX
      return s
    }}
  />
)

// @ts-expect-error - unknown props are still rejected
const bogus = <div totallyBogusProp={1} />

// @ts-expect-error - only event option modifiers get a camelCase alias
const stopAlias = <div onClickStop={(e) => e.preventDefault()} />

// @ts-expect-error - key names have no camelCase alias
const keyAlias = <div onKeyupEnter={(e) => e.preventDefault()} />

// @ts-expect-error - unsupported camelCase aliases are rejected
const unsupported = <div onClickBanana={(e) => e.preventDefault()} />

// @ts-expect-error - passive only applies to scroll-blocking events
const passiveOnClick = <div onClickPassive={(e) => e.preventDefault()} />

// @ts-expect-error - `scroll` is not cancelable, so passive does nothing
const passiveOnScroll = <div onScrollPassive={(e) => e.preventDefault()} />

// @ts-expect-error - touchend cancelation suppresses the click, not the scroll
const passiveOnTouchend = <div onTouchendPassive={(e) => e.preventDefault()} />

// ---------------------------------------------------------------------------
// h() type assertions
// ---------------------------------------------------------------------------

// The aliases resolve to the base event's payload type.
h('div', {
  onClickCapture: (e) => e.clientX,
  onClickOnce: (e) => e.clientX,
  onWheelPassive: (e) => e.deltaY,
  onTouchstartPassive: (e) => e.touches.length,
  onTouchmovePassive: (e) => e.touches.length,
})

// The base event keeps its own payload type.
h('div', { onClick: (e) => e.clientX })

// Inline handler parameters keep contextual typing — this only errors when
// `e` is typed as the event, not any.
h('div', {
  onClickCapture: (e) => {
    // @ts-expect-error - clientX is a number
    const s: string = e.clientX
    return s
  },
})

// @ts-expect-error - the click payload is not a wheel event
h('div', { onClickCapture: (e) => e.deltaY })

// @ts-expect-error - the wheel payload has no `touches`
h('div', { onWheelPassive: (e) => e.touches })

// `passive` is only offered for scroll-blocking events, so `onClickPassive`
// has no payload type and the handler parameter falls back to an implicit
// `any` — which `noImplicitAny` reports.
//
// @ts-expect-error - `onClickPassive` is not a known alias
h('div', { onClickPassive: (e) => e.clientX })

export {
  bogus,
  handlers,
  keyAlias,
  passiveOnClick,
  passiveOnScroll,
  passiveOnTouchend,
  stopAlias,
  typed,
  unsupported,
}
