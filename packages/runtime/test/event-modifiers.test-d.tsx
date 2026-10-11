// Typecheck-only regression test for event modifier props. Exercised by
// `pnpm typecheck` (tsmc --noEmit); @ts-expect-error lines must keep erroring.

// `_` modifiers and camelCase aliases resolve to the base event's payload
// type at any chain depth.
const handlers = (
  <>
    <div onClick={(e) => e.preventDefault()} />
    <div onClick_capture={(e) => e.preventDefault()} />
    <div onClickCapture={(e) => e.clientX} />
    <div onClickStop={(e) => e.stopPropagation()} />
    <div onClick_stop_prevent={(e) => e.clientX} />
    <div onClick_right_stop={(e) => e.clientX} />
    <div onKeyup_ctrl={(e) => e.key} />
    <div onKeyupShift={(e) => e.key} />
    <div onKeyupEnter={(e) => e.key} />
    <div onKeydownUp={(e) => e.key} />
    <div onDragexitStop={(e) => e.dataTransfer} />
    <div onTouchstartPassive={(e) => e.touches} />
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
    onKeyup_ctrl={(e) => {
      // @ts-expect-error - key is a string
      const n: number = e.key
      return n
    }}
  />
)

// @ts-expect-error - unknown props are still rejected
const bogus = <div totallyBogusProp={1} />

// @ts-expect-error - custom event names are not modifier aliases on elements
const custom = <div onMoveLeft={(e) => e.preventDefault()} />

// @ts-expect-error - unsupported camelCase aliases are rejected
const unsupported = <div onClickBanana={(e) => e.preventDefault()} />

// @ts-expect-error - key names only alias on keyboard events
const keyOnMouse = <div onDragEnter={(e) => e.preventDefault()} />

// @ts-expect-error - camel chains aren't modeled; use `onClick_right_stop`
const camelChain = <div onClickRightStop={(e) => e.preventDefault()} />

export { bogus, camelChain, custom, handlers, keyOnMouse, typed, unsupported }
