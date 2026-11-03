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
})
