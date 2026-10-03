import { transform } from '@vue-jsx-vapor/compiler-rs'
import { expect, it, vi } from 'vitest'
import { transformVueJsxVapor } from '../src/core'
import plugin from '../src/raw'

it('reports parse errors with locations', () => {
  const { errors, warnings } = transformVueJsxVapor(
    'const _c = () => <div><b></div>',
    'test.jsx',
  )
  expect(errors.length).toBeGreaterThan(0)
  expect(errors[0].message).toMatchInlineSnapshot(
    `"Expected corresponding JSX closing tag for 'b'."`,
  )
  expect(errors[0].loc).toBeDefined()
  expect(warnings).toHaveLength(0)
})

it('reports compiler validation errors with code and loc', () => {
  const { errors } = transformVueJsxVapor(
    'const _c = () => <input v-model={a+b} />',
    'test.jsx',
  )
  expect(errors.length).toBeGreaterThan(0)
  expect(errors[0].code).toBe(42)
})

it('forwards errors to a user-provided onError', () => {
  const received: unknown[] = []
  const { errors } = transformVueJsxVapor(
    'const _c = () => <input v-model={a+b} />',
    'test.jsx',
    { compiler: { onError: (e) => received.push(e) } },
  )
  expect(received.length).toBe(errors.length)
  expect(received.length).toBeGreaterThan(0)
})

it('does not route parse errors to onWarn', () => {
  const warnings: unknown[] = []
  transform('const _c = () => <div><b></div>', {
    filename: 'test.jsx',
    onWarn: (warning) => warnings.push(warning),
  })
  expect(warnings).toHaveLength(0)
})

it('reports no errors for valid input', () => {
  const { code, errors, warnings } = transformVueJsxVapor(
    'const _c = () => <div>{foo}</div>',
    'test.jsx',
  )
  expect(code).not.toBe('')
  expect(errors).toHaveLength(0)
  expect(warnings).toHaveLength(0)
})

function transformHandler() {
  const [{ transform }] = plugin()
  return (transform as { handler: Function }).handler
}

it('fails the transform on parse errors', () => {
  const handler = transformHandler()
  const ctx = {
    warn: vi.fn(),
    error: vi.fn((e) => {
      throw e
    }),
  }
  expect(() =>
    handler.call(ctx, 'const _c = () => <div><b></div>', 'test.tsx'),
  ).toThrowError(
    expect.objectContaining({
      message: "Expected corresponding JSX closing tag for 'b'.",
      id: 'test.tsx',
      loc: { file: 'test.tsx', line: 1, column: 27 },
    }),
  )
  expect(ctx.warn).not.toHaveBeenCalled()
})

it('reports correct line and column for errors', () => {
  const handler = transformHandler()
  const ctx = {
    warn: vi.fn(),
    error: vi.fn((e) => {
      throw e
    }),
  }
  const code = 'const a = 1\nconst _c = () => <div><b></div>'
  expect(() => handler.call(ctx, code, 'test.tsx')).toThrowError(
    expect.objectContaining({
      loc: { file: 'test.tsx', line: 2, column: 27 },
    }),
  )
})

it('reports correct line and column after Unicode characters', () => {
  const handler = transformHandler()
  const ctx = {
    warn: vi.fn(),
    error: vi.fn((e) => {
      throw e
    }),
  }
  const code = 'const c = "😀"\nconst _c = () => <div><b></div>'
  expect(() => handler.call(ctx, code, 'test.tsx')).toThrowError(
    expect.objectContaining({
      loc: { file: 'test.tsx', line: 2, column: 27 },
    }),
  )
})

it('returns output for valid input without diagnostics', () => {
  const handler = transformHandler()
  const ctx = { warn: vi.fn(), error: vi.fn() }
  const result = handler.call(
    ctx,
    'const _c = () => <div>{foo}</div>',
    'test.tsx',
  )
  expect(result?.code).toContain('_setNodes')
  expect(ctx.error).not.toHaveBeenCalled()
  expect(ctx.warn).not.toHaveBeenCalled()
})
