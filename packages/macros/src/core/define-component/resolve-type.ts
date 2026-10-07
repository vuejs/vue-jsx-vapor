import { walkAST } from 'ast-kit'
import { getRequire, type FunctionalNode } from '../utils'
import type { CallExpression, Expression, Node, Statement } from '@babel/types'
import type { SimpleTypeResolveContext } from 'vue/compiler-sfc'

type CompilerSfc = typeof import('vue/compiler-sfc')

export interface TypeContext {
  filename: string
  source: string
  body: Statement[]
  isProduction?: boolean
  /** imports a helper from `vue` and returns its local name */
  helper: (name: string) => string
}

export interface ResolvedTypes {
  /** runtime props object literal, as `<script setup>` generates it from a type */
  props?: string
  propKeys: Set<string>
  emits?: string[]
  /** from `const Name = defineComponent(...)` */
  name?: string
}

let sfc: CompilerSfc | undefined | null
function getCompilerSfc(): CompilerSfc | undefined {
  if (sfc !== undefined) return sfc ?? undefined
  const require = getRequire()
  try {
    sfc = require!('vue/compiler-sfc') as CompilerSfc
    // lets imports from packages and tsconfig paths resolve; relative imports work without it
    try {
      const ts = require!('typescript')
      sfc.registerTS(() => ts)
    } catch {}
  } catch {
    sfc = null
  }
  return sfc ?? undefined
}

function getTypeArguments(node: Node): Node[] {
  const args = (node as any).typeArguments ?? (node as any).typeParameters
  return args?.params ?? []
}

/** `E` from `SetupContext<E>` or `{ emit: EmitFn<E> }` on the setup's second parameter. */
function getEmitsType(root: FunctionalNode): Node | undefined {
  const annotation = (root.params[1] as any)?.typeAnnotation?.typeAnnotation as Node | undefined
  if (!annotation) return
  if (
    annotation.type === 'TSTypeReference' &&
    annotation.typeName.type === 'Identifier' &&
    annotation.typeName.name === 'SetupContext'
  ) {
    return getTypeArguments(annotation)[0]
  }
  if (annotation.type === 'TSTypeLiteral') {
    for (const member of annotation.members) {
      if (
        member.type === 'TSPropertySignature' &&
        member.key.type === 'Identifier' &&
        member.key.name === 'emit'
      ) {
        const type = member.typeAnnotation?.typeAnnotation
        if (
          type?.type === 'TSTypeReference' &&
          type.typeName.type === 'Identifier' &&
          type.typeName.name === 'EmitFn'
        ) {
          return getTypeArguments(type)[0]
        }
      }
    }
  }
}

function inferName(call: CallExpression, body: Statement[]): string | undefined {
  // options passed as a spread can't be extended
  if (call.arguments[1]?.type === 'SpreadElement') return
  let name: string | undefined
  walkAST<Node>({ type: 'Program', body } as Node, {
    enter(node) {
      if (node.type === 'VariableDeclarator' && node.init === call) {
        if (node.id.type === 'Identifier') name = node.id.name
        this.skip()
      }
    },
  })
  return name
}

/**
 * Runtime props and emits from the setup function's types, using the same resolver as
 * `defineProps<T>()` / `defineEmits<T>()` in `<script setup>`. As in
 * `@vue/babel-plugin-resolve-type`, explicit type arguments win over the parameter
 * annotations. Destructured defaults are passed along so they come out the way props
 * destructure generates them.
 */
export function resolveTypes(
  root: FunctionalNode,
  call: CallExpression,
  context: TypeContext,
): ResolvedTypes {
  const resolved: ResolvedTypes = { propKeys: new Set(), name: inferName(call, context.body) }
  const compiler = getCompilerSfc()
  if (!compiler) return resolved

  const [propsGeneric, emitsGeneric] = getTypeArguments(call)
  const propsParam = root.params[0] as any
  // `(props: Props = defaults) => ...`
  const annotated = propsParam?.type === 'AssignmentPattern' ? propsParam.left : propsParam
  const propsTypeDecl =
    propsGeneric ?? (annotated?.typeAnnotation?.typeAnnotation as Node | undefined)
  const emitsTypeDecl = emitsGeneric ?? getEmitsType(root)
  if (!propsTypeDecl && !emitsTypeDecl) return resolved

  const propsDestructuredBindings: SimpleTypeResolveContext['propsDestructuredBindings'] = {}
  if (propsParam?.type === 'ObjectPattern') {
    for (const prop of propsParam.properties) {
      if (prop.type !== 'ObjectProperty' || prop.key.type !== 'Identifier') continue
      const value = prop.value
      propsDestructuredBindings[prop.key.name] =
        value.type === 'AssignmentPattern'
          ? { local: value.left.name, default: value.right as Expression }
          : { local: value.name }
    }
  }

  const require = getRequire()
  const fs = require?.('node:fs')
  const ctx: SimpleTypeResolveContext & { ast: Statement[] } = {
    filename: context.filename,
    source: context.source,
    ast: context.body,
    options: {
      isProd: context.isProduction,
      fs: fs && {
        fileExists: (file: string) => fs.existsSync(file),
        readFile: (file: string) =>
          fs.existsSync(file) ? fs.readFileSync(file, 'utf8') : undefined,
        realpath: (file: string) => fs.realpathSync(file),
      },
    },
    isCE: false,
    error(message, node) {
      throw new Error(`[vue-jsx] ${message}\n${context.filename}:${node.loc?.start.line ?? '?'}`)
    },
    warn(message) {
      console.warn(`[vue-jsx] ${message}`)
    },
    helper: context.helper,
    getString: (node) => context.source.slice(node.start!, node.end!),
    propsTypeDecl: propsTypeDecl as any,
    propsRuntimeDefaults:
      propsParam?.type === 'AssignmentPattern' ? (propsParam.right as Expression) : undefined,
    propsDestructuredBindings,
    emitsTypeDecl: emitsTypeDecl as any,
  }

  if (propsTypeDecl) {
    resolved.props = compiler.extractRuntimeProps(ctx)
    for (const key of Object.keys(compiler.resolveTypeElements(ctx, propsTypeDecl as any).props))
      resolved.propKeys.add(key)
  }
  if (emitsTypeDecl) {
    resolved.emits = [...compiler.extractRuntimeEmits(ctx)]
  }
  return resolved
}
