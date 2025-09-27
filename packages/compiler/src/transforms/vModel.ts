import { IRNodeTypes, type DirectiveIRNode } from '../ir'
import {
  createCompilerError,
  createSimpleExpression,
  ErrorCodes,
  findProp,
  getText,
  isJSXComponent,
  isMemberExpression,
  resolveDirective,
} from '../utils'
import type { DirectiveTransform } from '../transform'
import type { JSXElement } from '@babel/types'

export const transformVModel: DirectiveTransform = (_dir, node, context) => {
  const dir = resolveDirective(_dir, context)
  const { exp, arg } = dir
  if (!exp) {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_MODEL_NO_EXPRESSION, dir.loc),
    )
    return
  }

  const expString = exp.content
  if (!expString.trim() || !isMemberExpression(exp)) {
    context.options.onError(
      createCompilerError(
        ErrorCodes.X_V_MODEL_MALFORMED_EXPRESSION,
        exp.loc as any,
      ),
    )
    return
  }

  const isComponent = isJSXComponent(node)
  if (isComponent) {
    return {
      key: arg ? arg : createSimpleExpression('modelValue', true),
      value: exp,
      model: true,
      modelModifiers: dir.modifiers.map((m) => m.content),
    }
  }

  if (dir.arg)
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_MODEL_ARG_ON_ELEMENT, dir.arg.loc),
    )
  const tag = getText(node.openingElement.name, context)
  const isCustomElement = context.options.isCustomElement(tag)
  let modelType: DirectiveIRNode['modelType'] | undefined = 'text'
  // TODO let runtimeDirective: VaporHelper | undefined = 'vModelText'
  if (
    tag === 'input' ||
    tag === 'textarea' ||
    tag === 'select' ||
    isCustomElement
  ) {
    if (tag === 'input' || isCustomElement) {
      const type = findProp(node, 'type')
      if (type?.value) {
        if (type.value.type === 'JSXExpressionContainer') {
          // type={foo}
          modelType = 'dynamic'
        } else if (type.value.type === 'StringLiteral') {
          switch (type.value.value) {
            case 'radio':
              modelType = 'radio'
              break
            case 'checkbox':
              modelType = 'checkbox'
              break
            case 'file':
              modelType = undefined
              context.options.onError(
                createCompilerError(
                  ErrorCodes.X_V_MODEL_ON_FILE_INPUT_ELEMENT,
                  dir.loc,
                ),
              )
              break
            default:
              // text type
              checkDuplicatedValue()
              break
          }
        }
      } else if (hasDynamicKeyVBind(node)) {
        // element has bindings with dynamic keys, which can possibly contain
        // "type".
        modelType = 'dynamic'
      } else {
        // text type
        checkDuplicatedValue()
      }
    } else if (tag === 'select') {
      modelType = 'select'
    } else {
      // textarea
      checkDuplicatedValue()
    }
  } else {
    context.options.onError(
      createCompilerError(ErrorCodes.X_V_MODEL_ON_INVALID_ELEMENT, dir.loc),
    )
  }

  if (modelType)
    context.registerOperation({
      type: IRNodeTypes.DIRECTIVE,
      element: context.reference(),
      dir,
      name: 'model',
      modelType,
      builtin: true,
    })

  function checkDuplicatedValue() {
    const value = findProp(node, 'value')
    if (value && value.value?.type !== 'StringLiteral') {
      context.options.onError(
        createCompilerError(ErrorCodes.X_V_MODEL_UNNECESSARY_VALUE, value.loc),
      )
    }
  }
}

function hasDynamicKeyVBind(node: JSXElement): boolean {
  return node.openingElement.attributes.some(
    (p) =>
      p.type === 'JSXSpreadAttribute' ||
      (p.type === 'JSXAttribute' &&
        p.name.type === 'JSXNamespacedName' &&
        !p.name.namespace.name.startsWith('v-')),
  )
}
