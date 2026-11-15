import { describe, expect, test } from 'vitest'
import { transformJsxMacros } from '../src/core'

const options = {
  defineModel: { alias: ['defineModel'] },
  defineSlots: { alias: ['defineSlots'] },
  defineStyle: { alias: ['defineStyle'] },
  defineExpose: { alias: ['defineExpose'] },
  defineComponent: { alias: ['defineComponent', 'defineVaporComponent'] },
}

describe('fixtures', () => {
  for (const [id, code] of Object.entries(
    import.meta.glob('./fixtures/**/*.tsx', {
      eager: true,
      as: 'raw',
    }),
  )) {
    test(id, async () => {
      expect(
        (
          await transformJsxMacros(code, id, new Map(), {
            include: ['*.tsx'],
            version: 3.6,
            ...options,
          })
        )?.code,
      ).toMatchSnapshot()
    })
  }
})

describe('defineComponent', () => {
  test('rejects a computed props key', async () => {
    await expect(
      transformJsxMacros(
        `const key = 'foo'\ndefineVaporComponent(({ [key]: foo }) => <div>{foo}</div>)`,
        'computed-key.tsx',
        new Map(),
        { include: ['*.tsx'], version: 3.6, ...options },
      ),
    ).rejects.toThrow('Destructured props cannot use a computed key: [key]')
  })

  test('declares a props member only by a literal key', async () => {
    const code = (await transformJsxMacros(
      `const key = 'foo'\ndefineVaporComponent((props) => <div>{props[key]}{props[0]}{props.bar}</div>)`,
      'computed-member.tsx',
      new Map(),
      { include: ['*.tsx'], version: 3.6, ...options },
    ))!.code
    expect(code).toContain(`'0': null`)
    expect(code).toContain(`'bar': null`)
    expect(code).not.toContain(`'key': null`)
  })
})
