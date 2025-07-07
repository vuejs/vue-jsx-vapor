import {
  advancePositionWithMutation,
  locStub,
  NewlineType,
  type CodegenSourceMapGenerator,
  type Position,
  type SourceLocation,
} from '@vue/compiler-dom'
import { isArray, isString } from '@vue/shared'
import { SourceMapGenerator } from 'source-map-js'
import type { CodegenContext } from '../generate'

export { genExpression } from './expression'

export const NEWLINE: unique symbol = Symbol(`newline`)
export const INDENT_START: unique symbol = Symbol(`indent start`)
export const INDENT_END: unique symbol = Symbol(`indent end`)

type FalsyValue = false | null | undefined
export type CodeFragment =
  | typeof NEWLINE
  | typeof INDENT_START
  | typeof INDENT_END
  | string
  | [code: string, newlineIndex?: number, loc?: SourceLocation, name?: string]
  | FalsyValue
export type CodeFragments = Exclude<CodeFragment, any[]> | CodeFragment[]

export function buildCodeFragment(
  ...frag: CodeFragment[]
): [
  CodeFragment[],
  (...items: CodeFragment[]) => number,
  (...items: CodeFragment[]) => number,
] {
  const push = frag.push.bind(frag)
  const unshift = frag.unshift.bind(frag)
  return [frag, push, unshift]
}

export type CodeFragmentDelimiters = [
  left: CodeFragments,
  right: CodeFragments,
  delimiter: CodeFragments,
  placeholder?: CodeFragments,
]

export function genMulti(
  [left, right, seg, placeholder]: CodeFragmentDelimiters,
  ...frags: CodeFragments[]
): CodeFragment[] {
  if (placeholder) {
    while (frags.length > 0 && !frags.at(-1)) {
      frags.pop()
    }
    frags = frags.map((frag) => frag || placeholder)
  } else {
    frags = frags.filter(Boolean)
  }

  const frag: CodeFragment[] = []
  push(left)
  for (const [i, fn] of (
    frags as Array<Exclude<CodeFragments, FalsyValue>>
  ).entries()) {
    push(fn)
    if (i < frags.length - 1) push(seg)
  }
  push(right)
  return frag

  function push(fn: CodeFragments) {
    if (!isArray(fn)) fn = [fn]
    frag.push(...fn)
  }
}
export const DELIMITERS_ARRAY: CodeFragmentDelimiters = ['[', ']', ', ']
export const DELIMITERS_ARRAY_NEWLINE: CodeFragmentDelimiters = [
  ['[', INDENT_START, NEWLINE],
  [INDENT_END, NEWLINE, ']'],
  [', ', NEWLINE],
]
export const DELIMITERS_OBJECT: CodeFragmentDelimiters = ['{ ', ' }', ', ']
export const DELIMITERS_OBJECT_NEWLINE: CodeFragmentDelimiters = [
  ['{', INDENT_START, NEWLINE],
  [INDENT_END, NEWLINE, '}'],
  [', ', NEWLINE],
]

export function genCall(
  name: string | [name: string, placeholder?: CodeFragments],
  ...frags: CodeFragments[]
): CodeFragment[] {
  const hasPlaceholder = isArray(name)
  const fnName = hasPlaceholder ? name[0] : name
  const placeholder = hasPlaceholder ? name[1] : 'null'
  return [fnName, ...genMulti(['(', ')', ', ', placeholder], ...frags)]
}

export function codeFragmentToString(
  code: CodeFragment[],
  context: CodegenContext,
): [code: string, map: CodegenSourceMapGenerator | undefined] {
  const {
    options: { filename, sourceMap },
  } = context

  let map: CodegenSourceMapGenerator | undefined
  if (sourceMap) {
    // lazy require source-map implementation, only in non-browser builds
    map = new SourceMapGenerator() as unknown as CodegenSourceMapGenerator
    map.setSourceContent(filename, context.ir.source)
    map._sources.add(filename)
  }

  let codegen = ''
  const pos = { line: 1, column: 1, offset: 0 }
  let indentLevel = 0

  for (let frag of code) {
    if (!frag) continue

    if (frag === NEWLINE) {
      frag = [`\n${`  `.repeat(indentLevel)}`, NewlineType.Start]
    } else if (frag === INDENT_START) {
      indentLevel++
      continue
    } else if (frag === INDENT_END) {
      indentLevel--
      continue
    }

    if (isString(frag)) frag = [frag]

    let [code, newlineIndex = NewlineType.None, loc, name] = frag
    codegen += code

    if (map) {
      if (loc) addMapping(loc.start, name)
      if (newlineIndex === NewlineType.Unknown) {
        // multiple newlines, full iteration
        advancePositionWithMutation(pos, code)
      } else {
        // fast paths
        pos.offset += code.length
        if (newlineIndex === NewlineType.None) {
          pos.column += code.length
        } else {
          // single newline at known index
          if (newlineIndex === NewlineType.End) {
            newlineIndex = code.length - 1
          }
          pos.line++
          pos.column = code.length - newlineIndex
        }
      }
      if (loc && loc !== locStub) {
        addMapping(loc.end)
      }
    }
  }

  return [codegen, map]

  function addMapping(loc: Position, name: string | null = null) {
    // we use the private property to directly add the mapping
    // because the addMapping() implementation in source-map-js has a bunch of
    // unnecessary arg and validation checks that are pure overhead in our case.
    const { _names, _mappings } = map!
    if (name !== null && !_names.has(name)) _names.add(name)
    _mappings.add({
      originalLine: loc.line,
      originalColumn: loc.column - 1, // source-map column is 0 based
      generatedLine: pos.line,
      generatedColumn: pos.column - 1,
      source: filename,
      name,
    })
  }
}
