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
