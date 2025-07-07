export { compile, type CompilerOptions, type TransformPreset } from './compile'
export * from './transform'
export {
  CodegenContext,
  generate,
  type CodegenOptions,
  type VaporCodegenResult,
} from './generate'

export * from './ir'

export { transformText } from './transforms/transformText'
export { transformElement } from './transforms/transformElement'
export { transformChildren } from './transforms/transformChildren'
export { transformTemplateRef } from './transforms/transformTemplateRef'
export { transformVBind } from './transforms/vBind'
export { transformVOn } from './transforms/vOn'
export { transformVSlot } from './transforms/vSlot'
export { transformVSlots } from './transforms/vSlots'
export { transformVModel } from './transforms/vModel'
export { transformVShow } from './transforms/vShow'
export { transformVHtml } from './transforms/vHtml'
export { transformVFor } from './transforms/vFor'
export { transformVIf } from './transforms/vIf'
export { transformVOnce } from './transforms/vOnce'
export { transformVText } from './transforms/vText'
