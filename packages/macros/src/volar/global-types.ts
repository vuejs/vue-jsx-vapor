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
          `declare const ${alias}: { <T>(...args: __StyleArgs): T; scss: <T>(...args: __StyleArgs)=> T; sass: <T>(...args: __StyleArgs)=> T; stylus: <T>(...args: __StyleArgs)=> T; less: <T>(...args: __StyleArgs)=> T; postcss: <T>(...args: __StyleArgs)=> T };`,
      )
      .join('\n')
    defineStyle += `\ntype __StyleArgs = [style: string, options?: { scoped?: boolean }];`
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
  return `
${defineModel.length ? `declare const { ${defineModel.join(',')} }: typeof import('vue');` : ''}
${defineSlots}
${defineExpose}
${defineStyle}
type ResolveSlots<T> = {
  [K in keyof T]?: T[K] extends Record<string, any>
    ? (props: T[K]) => any
    : T[K]
};
`
}
