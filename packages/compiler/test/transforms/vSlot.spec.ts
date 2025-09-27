import { describe, expect, test, vi } from 'vitest'
import {
  IRNodeTypes,
  IRSlotType,
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
  transformVFor,
  transformVIf,
  transformVOn,
  transformVSlot,
} from '../../src'
import { ErrorCodes } from '../../src/utils'
import { makeCompile } from './_utils'

const compileWithSlots = makeCompile({
  nodeTransforms: [
    transformVIf,
    transformVFor,
    transformElement,
    transformText,
    transformVSlot,
    transformChildren,
  ],
  directiveTransforms: {
    bind: transformVBind,
    on: transformVOn,
  },
})

describe('compiler: transform slot', () => {
  test('implicit default slot', () => {
    const { ir, code } = compileWithSlots(`<Comp><div/></Comp>`)
    expect(code).toMatchSnapshot()

    expect(ir.templates).toEqual(['<div></div>'])
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      id: 1,
      tag: 'Comp',
      props: [[]],
      slots: [
        {
          slotType: IRSlotType.STATIC,
          slots: {
            default: {
              type: IRNodeTypes.BLOCK,
              dynamic: {
                children: [{ template: 0 }],
              },
            },
          },
        },
      ],
    })
    expect(ir.block.returns).toEqual([1])
    expect(ir.block.dynamic).toMatchObject({
      children: [{ id: 1 }],
    })
  })

  test('on-component default slot', () => {
    const { ir, code } = compileWithSlots(
      `<Comp v-slot={{ foo }}>{ foo + bar }</Comp>`,
    )
    expect(code).toMatchSnapshot()

    expect(code).contains(`"default": (_slotProps0) =>`)
    expect(code).contains(`_slotProps0["foo"] + bar`)

    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Comp',
      props: [[]],
      slots: [
        {
          slotType: IRSlotType.STATIC,
          slots: {
            default: {
              type: IRNodeTypes.BLOCK,
              props: {
                content: '{ foo }',
                ast: {
                  type: 'ArrowFunctionExpression',
                  params: [{ type: 'ObjectPattern' }],
                },
              },
            },
          },
        },
      ],
    })
  })

  test('on component named slot', () => {
    const { ir, code } = compileWithSlots(
      `<Comp v-slot:named={{ foo }}>{ foo + bar }</Comp>`,
    )
    expect(code).toMatchSnapshot()

    expect(code).contains(`"named": (_slotProps0) =>`)
    expect(code).contains(`_slotProps0["foo"] + bar`)

    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Comp',
      slots: [
        {
          slotType: IRSlotType.STATIC,
          slots: {
            named: {
              type: IRNodeTypes.BLOCK,
              props: {
                content: '{ foo }',
              },
            },
          },
        },
      ],
    })
  })

  test('on component dynamically named slot', () => {
    const { ir, code } = compileWithSlots(
      `<Comp v-slot:$named$={{ foo }}>{ foo + bar }</Comp>`,
    )
    expect(code).toMatchSnapshot()

    expect(code).contains(`fn: (_slotProps0) =>`)
    expect(code).contains(`_slotProps0["foo"] + bar`)

    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Comp',
      slots: [
        {
          name: {
            content: 'named',
            isStatic: false,
          },
          fn: {
            type: IRNodeTypes.BLOCK,
            props: {
              content: '{ foo }',
            },
          },
        },
      ],
    })
  })

  test('named slots w/ implicit default slot', () => {
    const { ir, code } = compileWithSlots(
      `<Comp>
        <template v-slot:one>foo</template>bar<span/>
      </Comp>`,
    )
    expect(code).toMatchSnapshot()
    expect(code).toMatchInlineSnapshot(`
      "
        const n6 = _createComponent(Comp, null, {
          "one": () => {
            const n1 = t0()
            return n1
          }, 
          "default": () => {
            const n3 = t1()
            const n4 = t2()
            return [n3, n4]
          }
        }, true)
        return n6
      "
    `)
    expect(ir.templates).toEqual(['foo', 'bar', '<span></span>'])
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      id: 6,
      tag: 'Comp',
      props: [[]],
      slots: [
        {
          slotType: IRSlotType.STATIC,
          slots: {
            one: {
              type: IRNodeTypes.BLOCK,
              dynamic: {
                children: [{ template: 0 }],
              },
            },
            default: {
              type: IRNodeTypes.BLOCK,
              dynamic: {
                children: [{}, {}, { template: 1 }, { template: 2 }, {}],
              },
            },
          },
        },
      ],
    })
  })

  test('named slots w/ comment', () => {
    const { ir, code } = compileWithSlots(
      `<Comp>
        {/* foo */}
        <template v-slot:one>foo</template>
      </Comp>`,
    )
    expect(code).toMatchSnapshot()
    const op = ir.block.dynamic.children[0].operation
    expect(op.slots.length).toEqual(1)
  })

  test('nested slots scoping', () => {
    const { ir, code } = compileWithSlots(
      `<Comp>
        <template v-slot:default={{ foo }}>
          <Inner v-slot={{ bar }}>
            { foo + bar + baz }
          </Inner>
          { foo + bar + baz }
        </template>
      </Comp>`,
    )
    expect(code).toMatchSnapshot()

    expect(code).contains(`"default": (_slotProps0) =>`)
    expect(code).contains(`"default": (_slotProps1) =>`)
    expect(code).contains(`_slotProps0["foo"] + _slotProps1["bar"] + baz`)
    expect(code).contains(`_slotProps0["foo"] + bar + baz`)

    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Comp',
      props: [[]],
      slots: [
        {
          slotType: IRSlotType.STATIC,
          slots: {
            default: {
              type: IRNodeTypes.BLOCK,
              props: {
                content: '{ foo }',
              },
            },
          },
        },
      ],
    })

    expect(
      (op as any).slots[0].slots.default.dynamic.children[1].operation,
    ).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Inner',
      slots: [
        {
          slotType: IRSlotType.STATIC,
          slots: {
            default: {
              type: IRNodeTypes.BLOCK,
              props: {
                content: '{ bar }',
              },
            },
          },
        },
      ],
    })
  })

  test('dynamic slots name', () => {
    const { ir, code } = compileWithSlots(
      `<Comp>
        <template v-slot:$name$>{foo}</template>
      </Comp>`,
    )
    expect(code).toMatchSnapshot()
    const op = ir.block.dynamic.children[0].operation
    expect(op.type).toBe(IRNodeTypes.CREATE_COMPONENT_NODE)
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Comp',
      slots: [
        {
          name: {
            content: 'name',
            isStatic: false,
          },
          fn: { type: IRNodeTypes.BLOCK },
        },
      ],
    })
  })

  test('dynamic slots name w/ v-for', () => {
    const { ir, code } = compileWithSlots(
      `<Comp>
        <template v-for={item in list} v-slot:$item$={{ bar }}>{ bar }</template>
      </Comp>`,
    )
    expect(code).toMatchSnapshot()
    expect(code).toMatchInlineSnapshot(`
      "
        const n4 = _createComponent(Comp, null, {
          $: [
            () => (_createForSlots(list, (item) => ({
              name: item, 
              fn: (_slotProps0) => {
                const n1 = _createNodes(() => (_slotProps0["bar"]))
                return n1
              }
            })))
          ]
        }, true)
        return n4
      "
    `)

    expect(code).contains(`fn: (_slotProps0) =>`)
    expect(code).contains(`_createNodes(() => (_slotProps0["bar"]))`)
    const op = ir.block.dynamic.children[0].operation
    expect(op.type).toBe(IRNodeTypes.CREATE_COMPONENT_NODE)
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Comp',
      slots: [
        {
          name: {
            content: 'item',
            isStatic: false,
          },
          fn: { type: IRNodeTypes.BLOCK },
          loop: {
            source: { content: 'list' },
            value: { content: 'item' },
            index: undefined,
          },
        },
      ],
    })
  })

  test('dynamic slots name w/ v-if / v-else[-if]', () => {
    const { ir, code } = compileWithSlots(
      `<Comp>
        <template v-if={condition} v-slot:condition>condition slot</template>
        <template v-else-if={anotherCondition} v-slot:condition={{ foo, bar }}>another condition</template>
        <template v-else v-slot:condition>else condition</template>
      </Comp>`,
    )
    expect(code).toMatchSnapshot()

    expect(code).contains(`fn: (_slotProps0) =>`)

    const op = ir.block.dynamic.children[0].operation
    expect(op.type).toBe(IRNodeTypes.CREATE_COMPONENT_NODE)
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'Comp',
      slots: [
        {
          slotType: IRSlotType.CONDITIONAL,
          condition: { content: 'condition' },
          positive: {
            slotType: IRSlotType.DYNAMIC,
          },
          negative: {
            slotType: IRSlotType.CONDITIONAL,
            condition: { content: 'anotherCondition' },
            positive: {
              slotType: IRSlotType.DYNAMIC,
            },
            negative: { slotType: IRSlotType.DYNAMIC },
          },
        },
      ],
    })
  })

  test('quote slot name', () => {
    const { code } = compileWithSlots(
      `<Comp><template v-slot:nav-bar-title-before></template></Comp>`,
    )
    expect(code).toMatchSnapshot()
    expect(code).contains(`"nav-bar-title-before"`)
  })

  test('nested component slot', () => {
    const { ir, code } = compileWithSlots(`<A><B/></A>`)
    expect(code).toMatchSnapshot()
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.CREATE_COMPONENT_NODE,
      tag: 'A',
      slots: [
        {
          slotType: IRSlotType.STATIC,
          slots: {
            default: {
              type: IRNodeTypes.BLOCK,
            },
          },
        },
      ],
    })
  })

  describe('errors', () => {
    test('error on extraneous children w/ named default slot', () => {
      const onError = vi.fn()
      const source = `<Comp><template v-slot:default>foo</template>bar</Comp>`
      compileWithSlots(source, { onError })
      const index = source.indexOf('bar')
      expect(onError.mock.calls[0][0]).toMatchObject({
        code: ErrorCodes.X_V_SLOT_EXTRANEOUS_DEFAULT_SLOT_CHILDREN,
        loc: {
          start: {
            index,
            line: 1,
            column: index,
          },
          end: {
            index: index + 3,
            line: 1,
            column: index + 3,
          },
        },
      })
    })

    test('error on duplicated slot names', () => {
      const onError = vi.fn()
      const source = `<Comp><template v-slot:foo></template><template v-slot:foo></template></Comp>`
      compileWithSlots(source, { onError })
      const index = source.lastIndexOf('v-slot:foo')
      expect(onError.mock.calls[0][0]).toMatchObject({
        code: ErrorCodes.X_V_SLOT_DUPLICATE_SLOT_NAMES,
        loc: {
          start: {
            index,
            line: 1,
            column: index,
          },
          end: {
            index: index + 10,
            line: 1,
            column: index + 10,
          },
        },
      })
    })

    test('error on invalid mixed slot usage', () => {
      const onError = vi.fn()
      const source = `<Comp v-slot={foo}><template v-slot:foo></template></Comp>`
      compileWithSlots(source, { onError })
      const index = source.lastIndexOf('v-slot={foo}')
      expect(onError.mock.calls[0][0]).toMatchObject({
        code: ErrorCodes.X_V_SLOT_MIXED_SLOT_USAGE,
        loc: {
          start: {
            index,
            line: 1,
            column: index,
          },
          end: {
            index: index + 12,
            line: 1,
            column: index + 12,
          },
        },
      })
    })

    test('error on v-slot usage on plain elements', () => {
      const onError = vi.fn()
      const source = `<div v-slot/>`
      compileWithSlots(source, { onError })
      const index = source.indexOf('v-slot')
      expect(onError.mock.calls[0][0]).toMatchObject({
        code: ErrorCodes.X_V_SLOT_MISPLACED,
        loc: {
          start: {
            index,
            line: 1,
            column: index,
          },
          end: {
            index: index + 6,
            line: 1,
            column: index + 6,
          },
        },
      })
    })
  })
})
