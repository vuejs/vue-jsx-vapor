import { HELPER_PREFIX } from '@vue-macros/common'
import type { RootMap, TransformOptions } from '.'

export function getGlobalTypes(
  rootMap: RootMap,
  options: TransformOptions,
): string {
  let defineStyle = ''
  if (options.defineSlots.alias) {
    defineStyle = options.defineStyle.alias
      .map(
        (alias) =>
          `declare const ${alias}: { <T>(...args: ${HELPER_PREFIX}StyleArgs): T; scss: <T>(...args: ${HELPER_PREFIX}StyleArgs)=> T; sass: <T>(...args: ${HELPER_PREFIX}StyleArgs)=> T; stylus: <T>(...args: ${HELPER_PREFIX}StyleArgs)=> T; less: <T>(...args: ${HELPER_PREFIX}StyleArgs)=> T; postcss: <T>(...args: ${HELPER_PREFIX}StyleArgs)=> T };`,
      )
      .join('\n')
    defineStyle += `\ntype ${HELPER_PREFIX}StyleArgs = [style: string, options?: { scoped?: boolean }];`
  }
  if (!rootMap.size) {
    return `\n${defineStyle}`
  }

  const defineSlots = options.defineSlots.alias
    .flatMap((alias) => [
      `declare function ${alias}<T extends Record<string, any>>(): Partial<T>;`,
      `declare function ${alias}<T extends Record<string, any>>(slots: T): T;`,
    ])
    .join('\n')
  const defineExpose = options.defineExpose.alias
    .map(
      (alias) =>
        `declare function ${alias}<Exposed extends Record<string, any> = Record<string, any>>(exposed?: Exposed): Exposed;`,
    )
    .join('\n')
  const defineModel = options.defineModel.alias.map((alias) =>
    alias === 'defineModel' ? 'defineModel' : `defineModel: ${alias}`,
  )
  const defineComponent = options.defineComponent.alias.map((alias) =>
    ['defineComponent', 'defineVaporComponent'].includes(alias)
      ? ''
      : `defineComponent: ${alias}`,
  )
  const VueMacros = [...defineModel, ...defineComponent]
    .filter(Boolean)
    .join(',')
  return `
${VueMacros ? `declare const { ${VueMacros} }: typeof import('vue');` : ''}
${defineSlots}
${defineExpose}
${defineStyle}
// @ts-ignore
type __VLS_IsAny<T> = 0 extends 1 & T ? true : false; type __VLS_PickNotAny<A, B> = __VLS_IsAny<A> extends true ? B : A;
`
}
