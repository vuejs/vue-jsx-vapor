import {
  advancePositionWithClone,
  isStaticProperty,
  NewlineType,
  TS_NODE_TYPES,
  type SimpleExpressionNode,
  type SourceLocation,
} from '@vue/compiler-dom'
import { isString } from '@vue/shared'
import { walkIdentifiers } from 'ast-kit'
import { isConstantExpression } from '../utils'
import type { CodegenContext } from '../generate'
import { buildCodeFragment, type CodeFragment } from './utils'
import type { Identifier, Node } from '@babel/types'

export function genExpression(
  node: SimpleExpressionNode,
  context: CodegenContext,
  assignment?: string,
): CodeFragment[] {
  const { content, ast, isStatic, loc } = node

  if (isStatic) {
    return [[JSON.stringify(content), NewlineType.None, loc]]
  }

  if (
    !node.content.trim() ||
    // there was a parsing error
    ast === false ||
    isConstantExpression(node)
  ) {
    return [[content, NewlineType.None, loc], assignment && ` = ${assignment}`]
  }

  // the expression is a simple identifier
  if (ast === null) {
    return genIdentifier(content, context, loc, assignment)
  }

  const ids: Identifier[] = []
  const parentStackMap = new Map<Identifier, Node[]>()
  const parentStack: Node[] = []
  walkIdentifiers(
    ast!,
    (id) => {
      ids.push(id)
      parentStackMap.set(id, parentStack.slice())
    },
    false,
    parentStack,
  )

  let hasMemberExpression = false
  if (ids.length) {
    const [frag, push] = buildCodeFragment()
    const isTSNode = ast && TS_NODE_TYPES.includes(ast.type)
    const offset =
      (ast?.start ? ast.start - 1 : 0) - ((ast as any)._offset || 0)
    ids
      .sort((a, b) => a.start! - b.start!)
      .forEach((id, i) => {
        // range is offset by -1 due to the wrapping parens when parsed
        const start = id.start! - 1 - offset
        const end = id.end! - 1 - offset
        const last = ids[i - 1]

        if (!isTSNode || i !== 0) {
          const leadingText = content.slice(
            last ? last.end! - 1 - offset : 0,
            start,
          )
          if (leadingText.length) push([leadingText, NewlineType.Unknown])
        }

        const source = content.slice(start, end)
        const parentStack = parentStackMap.get(id)!
        const parent = parentStack.at(-1)

        hasMemberExpression ||=
          !!parent &&
          (parent.type === 'MemberExpression' ||
            parent.type === 'OptionalMemberExpression')

        push(
          ...genIdentifier(
            source,
            context,
            {
              start: advancePositionWithClone(node.loc.start, source, start),
              end: advancePositionWithClone(node.loc.start, source, end),
              source,
            },
            hasMemberExpression ? undefined : assignment,
            parent,
          ),
        )

        if (i === ids.length - 1 && end < content.length && !isTSNode) {
          push([content.slice(end), NewlineType.Unknown])
        }
      })

    if (assignment && hasMemberExpression) {
      push(` = ${assignment}`)
    }
    return frag
  } else {
    return [[content, NewlineType.Unknown, loc]]
  }
}

function genIdentifier(
  raw: string,
  context: CodegenContext,
  loc?: SourceLocation,
  assignment?: string,
  parent?: Node,
): CodeFragment[] {
  const { identifiers } = context
  const name: string | undefined = raw

  const idMap = identifiers[raw]
  if (idMap && idMap.length) {
    const replacement = idMap[0]
    if (isString(replacement)) {
      if (parent && parent.type === 'ObjectProperty' && parent.shorthand) {
        return [[`${name}: ${replacement}`, NewlineType.None, loc]]
      } else {
        return [[replacement, NewlineType.None, loc]]
      }
    } else {
      // replacement is an expression - process it again
      return genExpression(replacement, context, assignment)
    }
  }

  let prefix: string | undefined
  if (isStaticProperty(parent) && parent.shorthand) {
    // property shorthand like { foo }, we need to add the key since
    // we rewrite the value
    prefix = `${raw}: `
  }

  raw = withAssignment(raw)
  return [prefix, [raw, NewlineType.None, loc, name]]

  function withAssignment(s: string) {
    return assignment ? `${s} = ${assignment}` : s
  }
}
