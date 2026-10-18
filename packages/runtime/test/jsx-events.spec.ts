import { expect, expectTypeOf, test } from 'vite-plus/test'
import type { InputEvent, JSX } from '../src/jsx'

// Issue #82: event props used to be typed with React's synthetic events
// (`BaseSyntheticEvent`, `nativeEvent`, `persist()`, ...) while the runtime
// hands handlers the native DOM event unchanged. Event payloads are now the
// native DOM events, keeping React's element-generic narrowing of
// `currentTarget` — the one useful thing the synthetic types added.

type InputProps = JSX.IntrinsicElements['input']
type ButtonProps = JSX.IntrinsicElements['button']

type OnClickPayload = Parameters<NonNullable<ButtonProps['onClick']>>[0]
type OnChangePayload = Parameters<NonNullable<InputProps['onChange']>>[0]
type OnInputPayload = Parameters<NonNullable<InputProps['onInput']>>[0]

test('event props accept natively-annotated DOM handlers', () => {
  const props: ButtonProps = {
    onClick: (e: MouseEvent) => e.preventDefault(),
  }
  expect(props.onClick).toBeTypeOf('function')
})

test('currentTarget is narrowed to the element type', () => {
  expectTypeOf<OnChangePayload['currentTarget']>().toEqualTypeOf<EventTarget & HTMLInputElement>()
  expectTypeOf<OnClickPayload['currentTarget']>().toEqualTypeOf<EventTarget & HTMLButtonElement>()
})

test('onChange/onInput give cast-free access to .value', () => {
  const onChange: InputProps['onChange'] = (e) => e.currentTarget.value
  const onInput: InputProps['onInput'] = (e) => e.target.value
  expect([onChange, onInput]).toBeDefined()
})

test('payloads carry the native DOM event members', () => {
  expectTypeOf<OnClickPayload['button']>().toEqualTypeOf<number>()
  expectTypeOf<OnClickPayload['clientX']>().toEqualTypeOf<number>()
  expectTypeOf<OnChangePayload['target']>().toEqualTypeOf<EventTarget & HTMLInputElement>()
  expectTypeOf<OnInputPayload>().toEqualTypeOf<InputEvent<HTMLInputElement>>()
})

test('bubbling events keep `target` un-narrowed', () => {
  // a click may originate from a child element, unlike change/input/focus
  expectTypeOf<OnClickPayload['target']>().toEqualTypeOf<EventTarget | null>()
})

test('React-only synthetic members are rejected', () => {
  // nothing at runtime ever constructs these
  // @ts-expect-error `nativeEvent` does not exist on native DOM events
  expectTypeOf<OnClickPayload['nativeEvent']>().toBeUnknown()
  // @ts-expect-error `persist()` does not exist on native DOM events
  expectTypeOf<OnClickPayload['persist']>().toBeUndefined()
})
