export const helperPrefix = '/vue-jsx-vapor/macros' as const

export const useModelHelperId = `${helperPrefix}/use-model`
export { default as useModelHelperCode } from './use-model?raw'

export const withDefaultsHelperId = `${helperPrefix}/with-defaults` as const
export { default as useSlotsHelperCode } from './use-slots?raw'

export const useSlotsHelperId = `${helperPrefix}/use-slots` as const
export { default as withDefaultsHelperCode } from './with-defaults?raw'
