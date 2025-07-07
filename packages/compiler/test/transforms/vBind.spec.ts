import { NodeTypes } from '@vue/compiler-dom'
import { describe, expect, test } from 'vitest'
import {
  DynamicFlag,
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformVBind,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithVBind = makeCompile({
  nodeTransforms: [transformElement, transformChildren],
  directiveTransforms: {
    bind: transformVBind,
  },
})

describe('compiler v-bind', () => {
  test('basic', () => {
    const { ir, code } = compileWithVBind(`<div id={id}/>`)
    expect(code).toMatchInlineSnapshot(`
      "
        const n0 = t0()
        _renderEffect(() => _setProp(n0, "id", id))
        return n0
      "
    `)

    expect(ir.block.dynamic.children[0]).toMatchObject({
      id: 0,
      flags: DynamicFlag.REFERENCED,
    })
    expect(ir.templates).toEqual(['<div></div>'])
    expect(ir.block.effect).lengthOf(1)
    expect(ir.block.effect[0].expressions).lengthOf(1)
    expect(ir.block.effect[0].operations).lengthOf(1)
    expect(ir.block.effect[0]).toMatchObject({
      expressions: [
        {
          type: NodeTypes.SIMPLE_EXPRESSION,
          content: 'id',
          isStatic: false,
        },
      ],
      operations: [
        {
          type: IRNodeTypes.SET_PROP,
          element: 0,
          prop: {
            key: {
              type: NodeTypes.SIMPLE_EXPRESSION,
              content: 'id',
              isStatic: true,
              loc: {
                start: { line: 1, column: 6, offset: 5 },
                end: { line: 1, column: 8, offset: 7 },
                source: 'id',
              },
            },
            values: [
              {
                type: NodeTypes.SIMPLE_EXPRESSION,
                content: 'id',
                isStatic: false,
                loc: {
                  source: 'id',
                  start: { line: 1, column: 10, offset: 9 },
                  end: { line: 1, column: 12, offset: 11 },
                },
              },
            ],
            loc: {
              start: { column: 5, line: 1, index: 5 },
              end: { column: 12, line: 1, index: 12 },
            },
            runtimeCamelize: false,
          },
        },
      ],
    })

    expect(code).contains('_setProp(n0, "id", id)')
  })

  test('no expression', () => {
    const { ir, code } = compileWithVBind(`<div id />`)

    expect(code).matchSnapshot()
    expect(ir.block.operation[0]).toMatchObject({
      type: IRNodeTypes.SET_PROP,
      prop: {
        key: {
          content: `id`,
          isStatic: true,
          loc: {
            start: { line: 1, column: 6, offset: 5 },
            end: { line: 1, column: 8, offset: 7 },
          },
        },
        values: [
          {
            content: `true`,
            isStatic: false,
            loc: {
              start: { line: 1, column: 1, offset: 0 },
              end: { line: 1, column: 1, offset: 0 },
            },
          },
        ],
      },
    })
    expect(code).contains('_setProp(n0, "id", true)')
  })

  /*
  test('no expression (shorthand)', () => {
    const { ir, code } = compileWithVBind(`<div :camel-case />`)

    expect(code).matchSnapshot()
    expect(ir.block.effect[0].operations[0]).toMatchObject({
      type: IRNodeTypes.SET_PROP,
      prop: {
        key: {
          content: `camel-case`,
          isStatic: true,
        },
        values: [
          {
            content: `camelCase`,
            isStatic: false,
          },
        ],
      },
    })
    expect(code).contains('_setDynamicProp(n0, "camel-case", _ctx.camelCase)')
  })

  test('dynamic arg', () => {
    const { ir, code } = compileWithVBind(
      `<div v-bind:[id]="id" v-bind:[title]="title" />`,
    )
    expect(code).matchSnapshot()
    expect(ir.block.effect[0].operations[0]).toMatchObject({
      type: IRNodeTypes.SET_DYNAMIC_PROPS,
      element: 0,
      props: [
        [
          {
            key: {
              type: NodeTypes.SIMPLE_EXPRESSION,
              content: 'id',
              isStatic: false,
            },
            values: [
              {
                type: NodeTypes.SIMPLE_EXPRESSION,
                content: 'id',
                isStatic: false,
              },
            ],
          },
          {
            key: {
              type: NodeTypes.SIMPLE_EXPRESSION,
              content: 'title',
              isStatic: false,
            },
            values: [
              {
                type: NodeTypes.SIMPLE_EXPRESSION,
                content: 'title',
                isStatic: false,
              },
            ],
          },
        ],
      ],
    })
    expect(code).contains(
      '_setDynamicProps(n0, { [_ctx.id]: _ctx.id, [_ctx.title]: _ctx.title })',
    )
  })

  test('dynamic arg w/ static attribute', () => {
    const { ir, code } = compileWithVBind(
      `<div v-bind:[id]="id" foo="bar" checked />`,
    )
    expect(code).matchSnapshot()
    expect(ir.block.effect[0].operations[0]).toMatchObject({
      type: IRNodeTypes.SET_DYNAMIC_PROPS,
      element: 0,
      props: [
        [
          {
            key: {
              type: NodeTypes.SIMPLE_EXPRESSION,
              content: 'id',
              isStatic: false,
            },
            values: [
              {
                type: NodeTypes.SIMPLE_EXPRESSION,
                content: 'id',
                isStatic: false,
              },
            ],
          },
          {
            key: {
              type: NodeTypes.SIMPLE_EXPRESSION,
              content: 'foo',
              isStatic: true,
            },
            values: [
              {
                type: NodeTypes.SIMPLE_EXPRESSION,
                content: 'bar',
                isStatic: true,
              },
            ],
          },
          {
            key: {
              type: NodeTypes.SIMPLE_EXPRESSION,
              content: 'checked',
              isStatic: true,
            },
          },
        ],
      ],
    })
    expect(code).contains(
      '_setDynamicProps(n0, { [_ctx.id]: _ctx.id, foo: "bar", checked: "" })',
    )
  })

  test('should error if empty expression', () => {
    const onError = vi.fn()
    const { ir, code } = compileWithVBind(`<div v-bind:arg="" />`, {
      onError,
    })

    expect(onError.mock.calls[0][0]).toMatchObject({
      code: ErrorCodes.X_V_BIND_NO_EXPRESSION,
      loc: {
        start: { line: 1, column: 6 },
        end: { line: 1, column: 19 },
      },
    })
    expect(ir.templates).toEqual(['<div arg></div>'])

    expect(code).matchSnapshot()
    expect(code).contains(JSON.stringify('<div arg></div>'))
  })

  test('error on invalid argument for same-name shorthand', () => {
    const onError = vi.fn()
    compileWithVBind(`<div v-bind:[arg] />`, { onError })
    expect(onError.mock.calls[0][0]).toMatchObject({
      code: ErrorCodes.X_V_BIND_INVALID_SAME_NAME_ARGUMENT,
      loc: {
        start: {
          line: 1,
          column: 13,
        },
        end: {
          line: 1,
          column: 18,
        },
      },
    })
  })
  */

  test('.camel modifier', () => {
    const { ir, code } = compileWithVBind(`<div foo-bar_camel={id}/>`)

    expect(ir.block.effect[0].operations[0]).toMatchObject({
      prop: {
        key: {
          content: `fooBar`,
          isStatic: true,
        },
        values: [
          {
            content: `id`,
            isStatic: false,
          },
        ],
        runtimeCamelize: false,
        modifier: undefined,
      },
    })

    expect(code).toMatchSnapshot()
    expect(code).contains('_setProp(n0, "fooBar", id)')
  })

  test.todo('.camel modifier w/ no expression', () => {
    const { ir, code } = compileWithVBind(`<div foo-bar_camel />`)

    expect(code).matchSnapshot()
    expect(ir.block.operation[0]).toMatchObject({
      prop: {
        key: {
          content: `fooBar`,
          isStatic: true,
        },
        values: [
          {
            content: `true`,
            isStatic: false,
          },
        ],
        runtimeCamelize: false,
        modifier: undefined,
      },
    })
    expect(code).contains('renderEffect')
    expect(code).contains('_setProp(n0, "fooBar", true)')
  })

  // test('.camel modifier w/ dynamic arg', () => {
  //   const { ir, code } = compileWithVBind(`<div v-bind:[foo].camel="id"/>`)

  //   expect(ir.block.effect[0].operations[0]).toMatchObject({
  //     type: IRNodeTypes.SET_DYNAMIC_PROPS,
  //     props: [
  //       [
  //         {
  //           key: {
  //             content: `foo`,
  //             isStatic: false,
  //           },
  //           values: [
  //             {
  //               content: `id`,
  //               isStatic: false,
  //             },
  //           ],
  //           runtimeCamelize: true,
  //           modifier: undefined,
  //         },
  //       ],
  //     ],
  //   })

  //   expect(code).matchSnapshot()
  //   expect(code).contains('renderEffect')
  //   expect(code).contains(
  //     `_setDynamicProps(n0, { [_camelize(_ctx.foo)]: _ctx.id })`,
  //   )
  // })

  // test.todo('.camel modifier w/ dynamic arg + prefixIdentifiers')

  test('.prop modifier', () => {
    const { ir, code } = compileWithVBind(`<div fooBar_prop={id}/>`)

    expect(code).matchSnapshot()
    expect(ir.block.effect[0].operations[0]).toMatchObject({
      prop: {
        key: {
          content: `fooBar`,
          isStatic: true,
        },
        values: [
          {
            content: `id`,
            isStatic: false,
          },
        ],
        runtimeCamelize: false,
        modifier: '.',
      },
    })
    expect(code).contains('renderEffect')
    expect(code).contains('_setDOMProp(n0, "fooBar", id)')
  })

  test.todo('.prop modifier w/ no expression', () => {
    const { ir, code } = compileWithVBind(`<div fooBar_prop />`)

    expect(code).matchSnapshot()
    expect(ir.block.effect[0].operations[0]).toMatchObject({
      prop: {
        key: {
          content: `fooBar`,
          isStatic: true,
        },
        values: [
          {
            content: `fooBar`,
            isStatic: false,
          },
        ],
        runtimeCamelize: false,
        modifier: '.',
      },
    })
    expect(code).contains('renderEffect')
    expect(code).contains('_setDOMProp(n0, "fooBar", fooBar)')
  })

  // test('.prop modifier w/ dynamic arg', () => {
  //   const { ir, code } = compileWithVBind(`<div v-bind:[fooBar].prop="id"/>`)

  //   expect(code).matchSnapshot()
  //   expect(ir.block.effect[0].operations[0]).toMatchObject({
  //     type: IRNodeTypes.SET_DYNAMIC_PROPS,
  //     props: [
  //       [
  //         {
  //           key: {
  //             content: `fooBar`,
  //             isStatic: false,
  //           },
  //           values: [
  //             {
  //               content: `id`,
  //               isStatic: false,
  //             },
  //           ],
  //           runtimeCamelize: false,
  //           modifier: '.',
  //         },
  //       ],
  //     ],
  //   })
  //   expect(code).contains('renderEffect')
  //   expect(code).contains(
  //     `_setDynamicProps(n0, { ["." + _ctx.fooBar]: _ctx.id })`,
  //   )
  // })

  // test.todo('.prop modifier w/ dynamic arg + prefixIdentifiers')

  // test('.prop modifier (shorthand)', () => {
  //   const { ir, code } = compileWithVBind(`<div .fooBar="id"/>`)

  //   expect(code).matchSnapshot()
  //   expect(ir.block.effect[0].operations[0]).toMatchObject({
  //     prop: {
  //       key: {
  //         content: `fooBar`,
  //         isStatic: true,
  //       },
  //       values: [
  //         {
  //           content: `id`,
  //           isStatic: false,
  //         },
  //       ],
  //       runtimeCamelize: false,
  //       modifier: '.',
  //     },
  //   })
  //   expect(code).contains('renderEffect')
  //   expect(code).contains('_setDOMProp(n0, "fooBar", _ctx.id)')
  // })

  // test('.prop modifier (shortband) w/ no expression', () => {
  //   const { ir, code } = compileWithVBind(`<div .fooBar />`)

  //   expect(code).matchSnapshot()
  //   expect(ir.block.effect[0].operations[0]).toMatchObject({
  //     prop: {
  //       key: {
  //         content: `fooBar`,
  //         isStatic: true,
  //       },
  //       values: [
  //         {
  //           content: `fooBar`,
  //           isStatic: false,
  //         },
  //       ],
  //       runtimeCamelize: false,
  //       modifier: '.',
  //     },
  //   })
  //   expect(code).contains('renderEffect')
  //   expect(code).contains('_setDOMProp(n0, "fooBar", _ctx.fooBar)')
  // })

  test('.attr modifier', () => {
    const { ir, code } = compileWithVBind(`<div foo-bar_attr={id}/>`)

    expect(code).matchSnapshot()
    expect(ir.block.effect[0].operations[0]).toMatchObject({
      prop: {
        key: {
          content: `foo-bar`,
          isStatic: true,
        },
        values: [
          {
            content: `id`,
            isStatic: false,
          },
        ],
        runtimeCamelize: false,
        modifier: '^',
      },
    })
    expect(code).contains('renderEffect')
    expect(code).contains('_setAttr(n0, "foo-bar", id)')
  })

  test.todo('.attr modifier w/ no expression', () => {
    const { ir, code } = compileWithVBind(`<div foo-bar_attr />`)

    expect(code).matchSnapshot()
    expect(ir.block.effect[0].operations[0]).toMatchObject({
      prop: {
        key: {
          content: `foo-bar`,
          isStatic: true,
        },
        values: [
          {
            content: `fooBar`,
            isStatic: false,
          },
        ],
        runtimeCamelize: false,
        modifier: '^',
      },
    })

    expect(code).contains('renderEffect')
    expect(code).contains('_setAttr(n0, "foo-bar", fooBar)')
  })

  test('with constant value', () => {
    const { code } = compileWithVBind(
      `
        <div
          a={void 0}
          b={1 > 2} 
          c={1 + 2}
          d={1 ? 2 : 3}
          e={(2)}
          f={\`foo${1}\`}
          g={1}
          h={'1'}
          i={true}
          j={null}
          l={{ foo: 1 }}
          n={{ ...{ foo: 1 } }}
          o={[1, , 3]}
          p={[1, ...[2, 3]]}
          q={[1, 2]}
          r={/\\s+/}
        />`,
    )
    expect(code).matchSnapshot()
  })
})
