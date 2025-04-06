import {
  // detectVueVersion,
  REGEX_NODE_MODULES,
  REGEX_SETUP_SFC,
  REGEX_SRC_FILE,
  type BaseOptions,
  type MarkRequired,
} from '@vue-macros/common'

export type Options = BaseOptions & {
  defineComponent?: { alias: string[]; autoReturnFunction?: boolean }
  defineModel?: { alias: string[] }
  defineExpose?: { alias: string[] }
  defineSlots?: { alias: string[] }
  defineStyle?: { alias: string[] }
}
export type OptionsResolved = MarkRequired<
  Options,
  | 'include'
  | 'version'
  | 'defineComponent'
  | 'defineModel'
  | 'defineExpose'
  | 'defineSlots'
  | 'defineStyle'
>

export function resolveOptions(options: Options): OptionsResolved {
  // waiting for vue@3.6 release
  // const version = options.version || detectVueVersion()
  const version = options.version || 3.6
  return {
    include: [REGEX_SRC_FILE],
    exclude: [REGEX_SETUP_SFC, REGEX_NODE_MODULES],
    ...options,
    version,
    defineComponent: {
      alias: options?.defineComponent?.alias ?? [
        'defineComponent',
        'defineVaporComponent',
      ],
    },
    defineModel: { alias: options?.defineModel?.alias ?? ['defineModel'] },
    defineSlots: { alias: options?.defineSlots?.alias ?? ['defineSlots'] },
    defineExpose: { alias: options?.defineExpose?.alias ?? ['defineExpose'] },
    defineStyle: { alias: options?.defineStyle?.alias ?? ['defineStyle'] },
  }
}
