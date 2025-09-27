import { describe, expect, test } from 'vitest'
import {
  IRNodeTypes,
  transformChildren,
  transformElement,
  transformText,
  transformVBind,
  transformVFor,
  transformVOn,
  transformVText,
  type ForIRNode,
} from '../../src'
import { makeCompile } from './_utils'

const compileWithVFor = makeCompile({
  nodeTransforms: [
    transformVFor,
    transformElement,
    transformText,
    transformChildren,
  ],
  directiveTransforms: {
    bind: transformVBind,
    on: transformVOn,
    text: transformVText,
  },
})

describe('compiler: v-for', () => {
  test('basic v-for', () => {
    const { code, ir, helpers } = compileWithVFor(
      `<div v-for={item in items} key={item.id} onClick={() => remove(item)}>{item}</div>`,
    )

    expect(code).toMatchSnapshot()

    expect(helpers).contains('createFor')
    expect(ir.templates).toEqual(['<div> </div>'])
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      id: 0,
      source: {
        content: 'items',
      },
      value: {
        content: 'item',
      },
      key: undefined,
      index: undefined,
      render: {
        type: IRNodeTypes.BLOCK,
        dynamic: {
          children: [{ template: 0 }],
        },
      },
      keyProp: {
        content: 'item.id',
      },
    })
    expect(ir.block.returns).toEqual([0])
    expect(ir.block.dynamic).toMatchObject({
      children: [{ id: 0 }],
    })
    expect(ir.block.effect).toEqual([])
  })

  test('key only binding pattern', () => {
    expect(
      compileWithVFor(
        `
          <tr
            v-for={row in rows}
            key={row.id}
          >
            { row.id + row.id }
          </tr>
      `,
      ).code,
    ).matchSnapshot()
  })

  test('selector pattern', () => {
    expect(
      compileWithVFor(
        `
          <tr
            v-for={row in rows}
            key={row.id}
            v-text={selected === row.id ? 'danger' : ''}
          ></tr>
      `,
      ).code,
    ).matchSnapshot()

    expect(
      compileWithVFor(
        `
          <tr
            v-for={row in rows}
            key={row.id}
            class={selected === row.id ? 'danger' : ''}
          ></tr>
      `,
      ).code,
    ).matchSnapshot()

    // Should not be optimized because row.label is not from parent scope
    expect(
      compileWithVFor(
        `
          <tr
            v-for={row in rows}
            key={row.id}
            class={row.label === row.id ? 'danger' : ''}
          ></tr>
      `,
      ).code,
    ).matchSnapshot()

    expect(
      compileWithVFor(
        `
          <tr
            v-for={row in rows}
            key={row.id}
            class={{ danger: row.id === selected }}
          ></tr>
      `,
      ).code,
    ).matchSnapshot()
  })

  test('multi effect', () => {
    const { code } = compileWithVFor(
      `<div v-for={(item, index) in items} item={item} index={index} />`,
    )
    expect(code).matchSnapshot()
  })

  test('nested v-for', () => {
    const { code, ir } = compileWithVFor(
      `<div v-for={i in list}><span v-for={j in i}>{ j+i }</span></div>`,
    )
    expect(code).matchSnapshot()
    expect(code).contains(`_createFor(() => (list), (_for_item0) => {`)
    expect(code).contains(
      `_createFor(() => (_for_item0.value), (_for_item1) => {`,
    )
    expect(code).contains(`_for_item1.value+_for_item0.value`)
    expect(ir.templates).toEqual(['<span> </span>', '<div></div>'])
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      id: 0,
      source: { content: 'list' },
      value: { content: 'i' },
      render: {
        type: IRNodeTypes.BLOCK,
        dynamic: {
          children: [{ template: 1 }],
        },
      },
    })
    expect(
      (op as ForIRNode).render.dynamic.children[0].children[0].operation,
    ).toMatchObject({
      type: IRNodeTypes.FOR,
      id: 2,
      source: { content: 'i' },
      value: { content: 'j' },
      render: {
        type: IRNodeTypes.BLOCK,
        dynamic: {
          children: [{ template: 0 }],
        },
      },
    })
  })

  test('object value, key and index', () => {
    const { code, ir } = compileWithVFor(
      '<span v-for={(value, key, index) in items} key={id}>{ id }{ value }{ index }</span>',
    )
    expect(code).matchSnapshot()
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      source: {
        content: 'items',
      },
      value: {
        content: 'value',
      },
      key: {
        content: 'key',
      },
      index: {
        content: 'index',
      },
    })
  })

  test('object de-structured value', () => {
    const { code, ir } = compileWithVFor(
      '<span v-for={({ id, value }) in items} key={id}>{ id }{ value }</span>',
    )
    expect(code).matchSnapshot()
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      source: {
        content: 'items',
      },
      value: {
        content: '{ id, value }',
        ast: {
          type: 'ArrowFunctionExpression',
          params: [
            {
              type: 'ObjectPattern',
            },
          ],
        },
      },
      key: undefined,
      index: undefined,
    })
  })

  test('object de-structured value (with rest)', () => {
    const { code, ir } = compileWithVFor(
      `<div v-for={(  { id, ...other }, index) in list} key={id}>{ id + other + index }</div>`,
    )
    expect(code).matchSnapshot()
    expect(code).toContain('_getRestElement(_for_item0.value, ["id"])')
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      source: {
        content: 'list',
      },
      value: {
        content: '{ id, ...other }',
        ast: {
          type: 'ArrowFunctionExpression',
          params: [
            {
              type: 'ObjectPattern',
            },
          ],
        },
      },
      key: {
        content: 'index',
      },
      index: undefined,
    })
  })

  test('array de-structured value', () => {
    const { code, ir } = compileWithVFor(
      `<div v-for={([id, other], index) in list} key={id}>{ id + other + index }</div>`,
    )
    expect(code).matchSnapshot()
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      source: {
        content: 'list',
      },
      value: {
        content: '[id, other]',
        ast: {
          type: 'ArrowFunctionExpression',
          params: [
            {
              type: 'ArrayPattern',
            },
          ],
        },
      },
      key: {
        content: 'index',
      },
      index: undefined,
    })
  })

  test('array de-structured value (with rest)', () => {
    const { code, ir } = compileWithVFor(
      `<div v-for={([id, ...other], index) in list} key={id}>{ id + other + index }</div>`,
    )
    expect(code).matchSnapshot()
    expect(code).toContain('_for_item0.value.slice(1)')
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      source: {
        content: 'list',
      },
      value: {
        content: '[id, ...other]',
        ast: {
          type: 'ArrowFunctionExpression',
          params: [
            {
              type: 'ArrayPattern',
            },
          ],
        },
      },
      key: {
        content: 'index',
      },
      index: undefined,
    })
  })

  test('v-for aliases w/ complex expressions', () => {
    const { code, ir } = compileWithVFor(
      `<div v-for={({ foo, baz: [qux] }) in list}>
        { foo + bar + baz + qux + quux }
      </div>`,
    )
    expect(code).matchSnapshot()
    const op = ir.block.dynamic.children[0].operation
    expect(op).toMatchObject({
      type: IRNodeTypes.FOR,
      source: {
        content: 'list',
      },
      value: {
        content: '{ foo, baz: [qux] }',
        ast: {
          type: 'ArrowFunctionExpression',
          params: [
            {
              type: 'ObjectPattern',
            },
          ],
        },
      },
      key: undefined,
      index: undefined,
    })
  })
  test('fast-remove flag', () => {
    const { code, ir } = compileWithVFor(
      `<div>
        <span v-for={j in i}>{ j+i }</span>
      </div>`,
    )

    const op = ir.block.dynamic.children[0].children[1].operation
    expect(op).toMatchObject({
      onlyChild: true,
    })
    expect(code).toMatchSnapshot()
  })

  test('v-for on component', () => {
    const { code, ir } = compileWithVFor(
      `<Comp v-for={item in list}>{item}</Comp>`,
    )
    expect(code).matchSnapshot()
    expect(
      (ir.block.dynamic.children[0].operation as ForIRNode).component,
    ).toBe(true)
  })

  test('v-for on template with single component child', () => {
    const { code, ir } = compileWithVFor(
      `<template v-for={item in list}><Comp>{item}</Comp></template>`,
    )
    expect(code).matchSnapshot()
    expect(
      (ir.block.dynamic.children[0].operation as ForIRNode).component,
    ).toBe(true)
  })
})
