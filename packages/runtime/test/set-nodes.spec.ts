// @vitest-environment happy-dom
import { expect, test } from 'vite-plus/test'
import { setNodes } from '../src/vapor'

// The compiler pairs every expression container with a blank text node in the template
// (`_template("<div> ", 1)`) and hands it to `setNodes` as the insertion anchor.
// `normalizeBlock` reuses it as its *write target* when the value is text; any other value
// kind is inserted before it, which used to leave the placeholder behind as a stray space.

const anchorIn = (parent: HTMLElement) => {
  const anchor = document.createTextNode(' ')
  parent.append(anchor)
  return anchor
}

test('setNodes drops the placeholder when it inserts a node', () => {
  const parent = document.createElement('div')
  const anchor = anchorIn(parent)

  setNodes(anchor, document.createElement('span'))

  expect(parent.innerHTML).toBe('<span></span>')
})

test('setNodes drops the placeholder for an array value', () => {
  const parent = document.createElement('div')

  setNodes(anchorIn(parent), [document.createElement('span'), document.createElement('b')])

  expect(parent.innerHTML).toBe('<span></span><b></b>')
})

// The anchor must survive: it is the insertion point later updates reuse, so the fix may
// clear its text but never remove it.
test('setNodes keeps the anchor in place after consuming the placeholder', () => {
  const parent = document.createElement('div')
  const anchor = anchorIn(parent)

  setNodes(anchor, document.createElement('span'))

  expect(parent.lastChild).toBe(anchor)
  expect(anchor.textContent).toBe('')
})

// The other half of the contract: a text value *is* written into the anchor, so its
// content must be left alone — this is why the clear is conditional.
test('setNodes writes into the anchor when the last value is text', () => {
  const parent = document.createElement('div')
  const anchor = anchorIn(parent)

  setNodes(anchor, 'abc')

  expect(parent.innerHTML).toBe('abc')
  expect(parent.firstChild).toBe(anchor)
})

test('setNodes writes into the anchor for a number and for nullish values', () => {
  const numberParent = document.createElement('div')
  const numberAnchor = anchorIn(numberParent)
  setNodes(numberAnchor, 123)
  expect(numberParent.innerHTML).toBe('123')

  const nullParent = document.createElement('div')
  const nullAnchor = anchorIn(nullParent)
  setNodes(nullAnchor, null)
  expect(nullParent.innerHTML).toBe('')
  expect(nullParent.firstChild).toBe(nullAnchor)
})

// `<div>a{...}b</div>` compiles to `_setNodes(anchor, "a", node, "b")`: only the *last*
// value receives the anchor, so the trailing text lands in it and the placeholder must be
// preserved. This test also pins that invariant — `setNodes` reads the anchor back out of
// the last result rather than searching, so handing it to every value fails here.
test('setNodes keeps the anchor when a trailing text value writes into it', () => {
  const parent = document.createElement('div')
  const anchor = anchorIn(parent)

  setNodes(anchor, 'a', document.createElement('b'), 'b')

  expect(parent.innerHTML).toBe('a<b></b>b')
  expect(parent.lastChild).toBe(anchor)
  expect(anchor.textContent).toBe('b')
})
