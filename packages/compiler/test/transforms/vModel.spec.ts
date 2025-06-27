import { DOMErrorCodes, ErrorCodes } from '@vue/compiler-dom'
import { describe, expect, test, vi } from 'vitest'
import {
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformVModel,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithVModel = makeCompile({
  nodeTransforms: [transformElement, transformChildren],
  directiveTransforms: {
    model: transformVModel,
  },
})

describe('compiler: vModel transform', () => {
  test('should support simple expression', () => {
    const { code, helpers } = compileWithVModel('<input v-model={model} />')
    expect(code).toMatchSnapshot()
    expect(helpers).toContain('applyTextModel')
  })

  describe('modifiers', () => {
    test('.number', () => {
      const { code } = compileWithVModel('<input v-model_number={model} />')

      expect(code).toMatchSnapshot()
    })

    test('.trim', () => {
      const { code } = compileWithVModel('<input v-model_trim={model} />')

      expect(code).toMatchSnapshot()
    })

    test('.lazy', () => {
      const { code } = compileWithVModel('<input v-model_lazy={model} />')

      expect(code).toMatchSnapshot()
    })
  })

  test('should support input (text)', () => {
    const { code, helpers } = compileWithVModel(
      '<input type="text" v-model={model} />',
    )
    expect(code).toMatchSnapshot()
    expect(helpers).toContain('applyTextModel')
  })

  test('should support input (radio)', () => {
    const { code, helpers } = compileWithVModel(
      '<input type="radio" v-model={model} />',
    )
    expect(code).toMatchSnapshot()
    expect(helpers).toContain('applyRadioModel')
  })

  test('should support input (checkbox)', () => {
    const { code, helpers } = compileWithVModel(
      '<input type="checkbox" v-model={model} />',
    )
    expect(code).toMatchSnapshot()
    expect(helpers).toContain('applyCheckboxModel')
  })

  test('should support select', () => {
    const { code, helpers } = compileWithVModel('<select v-model={model} />')
    expect(code).toMatchSnapshot()
    expect(helpers).toContain('applySelectModel')
  })

  test('should support textarea', () => {
    const { code, helpers } = compileWithVModel('<textarea v-model={model} />')
    expect(code).toMatchSnapshot()
    expect(helpers).toContain('applyTextModel')
  })

  test('should support input (dynamic type)', () => {
    const { code, helpers } = compileWithVModel(
      '<input type={foo} v-model={model} />',
    )
    expect(code).toMatchSnapshot()
    expect(helpers).toContain('applyDynamicModel')
  })

  test('should support w/ dynamic v-bind', () => {
    const root = compileWithVModel('<input {...obj} v-model={model} />')
    expect(root.code).toMatchSnapshot()
    expect(root.helpers).toContain('applyDynamicModel')
  })

  describe('errors', () => {
    test('invalid element', () => {
      const onError = vi.fn()
      compileWithVModel('<span v-model={model} />', { onError })

      expect(onError).toHaveBeenCalledTimes(1)
      expect(onError).toHaveBeenCalledWith(
        expect.objectContaining({
          code: DOMErrorCodes.X_V_MODEL_ON_INVALID_ELEMENT,
        }),
      )
    })

    test('plain elements with argument', () => {
      const onError = vi.fn()
      compileWithVModel('<input v-model:value={model} />', { onError })

      expect(onError).toHaveBeenCalledTimes(1)
      expect(onError).toHaveBeenCalledWith(
        expect.objectContaining({
          code: DOMErrorCodes.X_V_MODEL_ARG_ON_ELEMENT,
        }),
      )
    })

    // TODO: component
    test.fails('should allow usage on custom element', () => {
      const onError = vi.fn()
      const root = compileWithVModel('<my-input v-model={model} />', {
        onError,
        isCustomElement: (tag) => tag.startsWith('my-'),
      })
      expect(root.helpers).toContain('vModelText')
      expect(onError).not.toHaveBeenCalled()
    })

    test('should raise error if used file input element', () => {
      const onError = vi.fn()
      compileWithVModel(`<input type="file" v-model={test} />`, {
        onError,
      })
      expect(onError).toHaveBeenCalledWith(
        expect.objectContaining({
          code: DOMErrorCodes.X_V_MODEL_ON_FILE_INPUT_ELEMENT,
        }),
      )
    })

    test('should error on dynamic value binding alongside v-model', () => {
      const onError = vi.fn()
      compileWithVModel(`<input v-model={test} value={test} />`, {
        onError,
      })
      expect(onError).toHaveBeenCalledWith(
        expect.objectContaining({
          code: DOMErrorCodes.X_V_MODEL_UNNECESSARY_VALUE,
        }),
      )
    })

    // #3596
    test('should NOT error on static value binding alongside v-model', () => {
      const onError = vi.fn()
      compileWithVModel(`<input v-model={test} value="test" />`, {
        onError,
      })
      expect(onError).not.toHaveBeenCalled()
    })

    test('empty expression', () => {
      const onError = vi.fn()
      compileWithVModel('<span v-model="" />', { onError })

      expect(onError).toHaveBeenCalledTimes(1)
      expect(onError).toHaveBeenCalledWith(
        expect.objectContaining({
          code: ErrorCodes.X_V_MODEL_MALFORMED_EXPRESSION,
        }),
      )
    })

    test('mal-formed expression', () => {
      const onError = vi.fn()
      compileWithVModel('<span v-model={a + b} />', { onError })

      expect(onError).toHaveBeenCalledTimes(1)
      expect(onError).toHaveBeenCalledWith(
        expect.objectContaining({
          code: ErrorCodes.X_V_MODEL_MALFORMED_EXPRESSION,
        }),
      )
    })
  })

  test('should support member expression', () => {
    const { code } = compileWithVModel(
      '<><input v-model={setupRef.child} /><input v-model={setupLet.child} /><input v-model={setupMaybeRef.child} /></>',
    )

    expect(code).toMatchSnapshot()
  })

  test('should support member expression w/ inline', () => {
    const { code } = compileWithVModel(
      '<><input v-model={setupRef.child} /><input v-model={setupLet.child} /><input v-model={setupMaybeRef.child} /></>',
    )

    expect(code).toMatchSnapshot()
  })

  describe('component', () => {
    test('v-model for component should work', () => {
      const { code, ir } = compileWithVModel('<Comp v-model={foo} />')
      expect(code).toMatchSnapshot()
      expect(code).contains(`modelValue: () => (foo),`)
      expect(code).contains(
        `"onUpdate:modelValue": () => _value => (foo = _value)`,
      )
      expect(ir.block.dynamic.children[0].operation).toMatchObject({
        type: IRNodeTypes.CREATE_COMPONENT_NODE,
        tag: 'Comp',
        props: [
          [
            {
              key: { content: 'modelValue', isStatic: true },
              model: true,
              modelModifiers: [],
              values: [{ content: 'foo', isStatic: false }],
            },
          ],
        ],
      })
    })

    test('v-model with arguments for component should work', () => {
      const { code, ir } = compileWithVModel('<Comp v-model:bar={foo} />')
      expect(code).toMatchSnapshot()
      expect(code).contains(`bar: () => (foo),`)
      expect(code).contains(`"onUpdate:bar": () => _value => (foo = _value)`)
      expect(ir.block.dynamic.children[0].operation).toMatchObject({
        type: IRNodeTypes.CREATE_COMPONENT_NODE,
        tag: 'Comp',
        props: [
          [
            {
              key: { content: 'bar', isStatic: true },
              model: true,
              modelModifiers: [],
              values: [{ content: 'foo', isStatic: false }],
            },
          ],
        ],
      })
    })

    test('v-model with dynamic arguments for component should work', () => {
      const { code, ir } = compileWithVModel('<Comp v-model:$arg$={foo} />')
      expect(code).toMatchSnapshot()
      expect(code).contains(`[arg]: foo,`)
      expect(code).contains(
        `["onUpdate:" + arg]: () => _value => (foo = _value)`,
      )
      expect(ir.block.dynamic.children[0].operation).toMatchObject({
        type: IRNodeTypes.CREATE_COMPONENT_NODE,
        tag: 'Comp',
        props: [
          {
            key: { content: 'arg', isStatic: false },
            values: [{ content: 'foo', isStatic: false }],
            model: true,
            modelModifiers: [],
          },
        ],
      })
    })

    test('v-model for component should generate modelValueModifiers', () => {
      const { code, ir } = compileWithVModel(
        '<Comp v-model_trim_bar-baz={foo} />',
      )
      expect(code).toMatchSnapshot()
      expect(code).contain(
        `modelValueModifiers: () => ({ trim: true, "bar-baz": true })`,
      )
      expect(ir.block.dynamic.children[0].operation).toMatchObject({
        type: IRNodeTypes.CREATE_COMPONENT_NODE,
        tag: 'Comp',
        props: [
          [
            {
              key: { content: 'modelValue', isStatic: true },
              values: [{ content: 'foo', isStatic: false }],
              model: true,
              modelModifiers: ['trim', 'bar-baz'],
            },
          ],
        ],
      })
    })

    test('v-model with arguments for component should generate modelModifiers', () => {
      const { code, ir } = compileWithVModel(
        '<Comp v-model:foo_trim={foo} v-model:bar_number={bar} />',
      )
      expect(code).toMatchSnapshot()
      expect(code).contain(`fooModifiers: () => ({ trim: true })`)
      expect(code).contain(`barModifiers: () => ({ number: true })`)
      expect(ir.block.dynamic.children[0].operation).toMatchObject({
        type: IRNodeTypes.CREATE_COMPONENT_NODE,
        tag: 'Comp',
        props: [
          [
            {
              key: { content: 'foo', isStatic: true },
              values: [{ content: 'foo', isStatic: false }],
              model: true,
              modelModifiers: ['trim'],
            },
            {
              key: { content: 'bar', isStatic: true },
              values: [{ content: 'bar', isStatic: false }],
              model: true,
              modelModifiers: ['number'],
            },
          ],
        ],
      })
    })

    test('v-model with dynamic arguments for component should generate modelModifiers ', () => {
      const { code, ir } = compileWithVModel(
        '<Comp v-model:$foo$_trim={foo} v-model:$bar_value$_number={bar} />',
      )
      expect(code).toMatchSnapshot()
      expect(code).contain(`[foo + "Modifiers"]: () => ({ trim: true })`)
      expect(code).contain(
        `[bar.value + "Modifiers"]: () => ({ number: true })`,
      )
      expect(ir.block.dynamic.children[0].operation).toMatchObject({
        type: IRNodeTypes.CREATE_COMPONENT_NODE,
        tag: 'Comp',
        props: [
          {
            key: { content: 'foo', isStatic: false },
            values: [{ content: 'foo', isStatic: false }],
            model: true,
            modelModifiers: ['trim'],
          },
          {
            key: { content: 'bar.value', isStatic: false },
            values: [{ content: 'bar', isStatic: false }],
            model: true,
            modelModifiers: ['number'],
          },
        ],
      })
    })
  })
})
