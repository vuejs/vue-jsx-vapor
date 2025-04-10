import { createRuleTester } from 'eslint-vitest-rule-tester'
import { describe, expect, it } from 'vitest'
import jsxSortProps from '../src/rules/jsx-sort-props'

describe('jsx-sort-props', () => {
  const { invalid } = createRuleTester({
    name: 'jsx-sort-props',
    rule: jsxSortProps,
    parserOptions: {
      sourceType: 'module',
      ecmaFeatures: {
        jsx: true,
      },
    },
  })

  it('basic', async () => {
    const { result } = await invalid({
      code: `
        <div b a />
      `,
      errors: ['sortPropsByAlpha'],
    })
    expect(result.output).toMatchSnapshot()
  })

  it('reservedFirst', async () => {
    const { result } = await invalid({
      code: '<App a v-for={i in 4} v-if={true} b />',
      errors: ['listReservedPropsFirst', 'listReservedPropsFirst'],
      options: [
        { reservedFirst: ['v-if', 'v-for'], noSortAlphabetically: true },
      ],
    })
    expect(result.output).toMatchSnapshot()
  })

  it('reservedLast', async () => {
    const { result } = await invalid({
      code: '<App v-slot={{ foo }} onClick={() => {}} />',
      errors: ['listReservedPropsLast'],
      options: [{ reservedLast: ['v-slot'], callbacksLast: true }],
    })
    expect(result.output).toMatchSnapshot()
  })

  it('reservedFirst and reservedLast', async () => {
    const { result } = await invalid({
      code: '<App a v-model:b={foo} b v-model={foo} c v-slot:b={{ foo }} v-slot:a={{ foo }} onClick={() => {}} />',
      errors: [
        'listReservedPropsFirst',
        'listReservedPropsFirst',
        'sortPropsByAlpha',
        'listReservedPropsLast',
      ],
      options: [{ reservedFirst: ['v-model'], reservedLast: ['v-slot'] }],
    })
    expect(result.output).toMatchSnapshot()
  })
})
