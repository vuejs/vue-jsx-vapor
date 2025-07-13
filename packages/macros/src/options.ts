import {
  REGEX_NODE_MODULES,
  REGEX_SETUP_SFC,
  REGEX_SRC_FILE,
  type BaseOptions,
  type MarkRequired,
} from '@vue-macros/common'

type DefineComponentOptions = { alias?: string[]; autoReturnFunction?: boolean }
type DefineModelOptions = { alias?: string[] }
type DefineExposeOptions = { alias?: string[] }
type DefineSlotsOptions = { alias?: string[] }
type DefineStyleOptions = { alias?: string[] }
export type Options = BaseOptions & {
  defineComponent?: DefineComponentOptions
  defineModel?: DefineModelOptions
  defineExpose?: DefineExposeOptions
  defineSlots?: DefineSlotsOptions
  defineStyle?: DefineStyleOptions
}
export type OptionsResolved = MarkRequired<Options, 'include' | 'version'> & {
  defineComponent: MarkRequired<DefineComponentOptions, 'alias'>
  defineModel: MarkRequired<DefineModelOptions, 'alias'>
  defineExpose: MarkRequired<DefineExposeOptions, 'alias'>
  defineSlots: MarkRequired<DefineSlotsOptions, 'alias'>
  defineStyle: MarkRequired<DefineStyleOptions, 'alias'>
}

export function resolveOptions(options: Options): OptionsResolved {
  const version = options.version || 3.6
  return {
    include: [REGEX_SRC_FILE],
    exclude: [REGEX_SETUP_SFC, REGEX_NODE_MODULES],
    ...options,
    version,
    defineComponent: {
      ...options.defineComponent,
      alias: options.defineComponent?.alias ?? [
        'defineComponent',
        'defineVaporComponent',
      ],
    },
    defineModel: { alias: options.defineModel?.alias ?? ['defineModel'] },
    defineSlots: { alias: options.defineSlots?.alias ?? ['defineSlots'] },
    defineExpose: { alias: options.defineExpose?.alias ?? ['defineExpose'] },
    defineStyle: { alias: options.defineStyle?.alias ?? ['defineStyle'] },
  }
}
