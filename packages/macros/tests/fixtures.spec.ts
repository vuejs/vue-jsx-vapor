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

describe('defineComponent autoReturnFunction fixtures', () => {
  for (const [id, code] of Object.entries(
    import.meta.glob('./fixtures/**/define-component.tsx', {
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
            defineComponent: {
              alias: ['defineComponent', 'defineVaporComponent'],
              autoReturnFunction: true,
            },
          })
        )?.code,
      ).toMatchSnapshot()
    })
  }
})
