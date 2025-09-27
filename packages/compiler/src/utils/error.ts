import type { SourceLocation } from '@babel/types'

export enum ErrorCodes {
  X_V_IF_NO_EXPRESSION = 28,
  X_V_ELSE_NO_ADJACENT_IF = 30,
  X_V_FOR_NO_EXPRESSION = 31,
  X_V_FOR_MALFORMED_EXPRESSION = 32,
  X_V_ON_NO_EXPRESSION = 35,
  X_V_SLOT_MIXED_SLOT_USAGE = 37,
  X_V_SLOT_DUPLICATE_SLOT_NAMES = 38,
  X_V_SLOT_EXTRANEOUS_DEFAULT_SLOT_CHILDREN = 39,
  X_V_SLOT_MISPLACED = 40,
  X_V_MODEL_NO_EXPRESSION = 41,
  X_V_MODEL_MALFORMED_EXPRESSION = 42,
  X_V_HTML_NO_EXPRESSION = 53,
  X_V_HTML_WITH_CHILDREN = 54,
  X_V_TEXT_NO_EXPRESSION = 55,
  X_V_TEXT_WITH_CHILDREN = 56,
  X_V_MODEL_ON_INVALID_ELEMENT = 57,
  X_V_MODEL_ARG_ON_ELEMENT = 58,
  X_V_MODEL_ON_FILE_INPUT_ELEMENT = 59,
  X_V_MODEL_UNNECESSARY_VALUE = 60,
  X_V_SHOW_NO_EXPRESSION = 61,
}
export const errorMessages: Record<ErrorCodes, string> = {
  [ErrorCodes.X_V_IF_NO_EXPRESSION]: `v-if/v-else-if is missing expression.`,
  [ErrorCodes.X_V_ELSE_NO_ADJACENT_IF]: `v-else/v-else-if has no adjacent v-if or v-else-if.`,
  [ErrorCodes.X_V_FOR_NO_EXPRESSION]: `v-for is missing expression.`,
  [ErrorCodes.X_V_FOR_MALFORMED_EXPRESSION]: `v-for has invalid expression.`,
  [ErrorCodes.X_V_ON_NO_EXPRESSION]: `v-on is missing expression.`,
  [ErrorCodes.X_V_SLOT_MIXED_SLOT_USAGE]:
    `Mixed v-slot usage on both the component and nested <template>. ` +
    `When there are multiple named slots, all slots should use <template> ` +
    `syntax to avoid scope ambiguity.`,
  [ErrorCodes.X_V_SLOT_DUPLICATE_SLOT_NAMES]: `Duplicate slot names found. `,
  [ErrorCodes.X_V_SLOT_EXTRANEOUS_DEFAULT_SLOT_CHILDREN]:
    `Extraneous children found when component already has explicitly named ` +
    `default slot. These children will be ignored.`,
  [ErrorCodes.X_V_MODEL_NO_EXPRESSION]: `v-model is missing expression.`,
  [ErrorCodes.X_V_MODEL_MALFORMED_EXPRESSION]: `v-model value must be a valid JavaScript member expression.`,
  [ErrorCodes.X_V_SLOT_MISPLACED]: `v-slot can only be used on components or <template> tags.`,
  [ErrorCodes.X_V_HTML_NO_EXPRESSION]: `v-html is missing expression.`,
  [ErrorCodes.X_V_HTML_WITH_CHILDREN]: `v-html will override element children.`,
  [ErrorCodes.X_V_TEXT_NO_EXPRESSION]: `v-text is missing expression.`,
  [ErrorCodes.X_V_TEXT_WITH_CHILDREN]: `v-text will override element children.`,
  [ErrorCodes.X_V_MODEL_ARG_ON_ELEMENT]: `v-model argument is not supported on plain elements.`,
  [ErrorCodes.X_V_MODEL_ON_INVALID_ELEMENT]: `v-model can only be used on <input>, <textarea> and <select> elements.`,
  [ErrorCodes.X_V_MODEL_ON_FILE_INPUT_ELEMENT]: `v-model cannot be used on file inputs since they are read-only. Use a v-on:change listener instead.`,
  [ErrorCodes.X_V_MODEL_UNNECESSARY_VALUE]: `Unnecessary value binding used alongside v-model. It will interfere with v-model's behavior.`,
  [ErrorCodes.X_V_SHOW_NO_EXPRESSION]: `v-show is missing expression.`,
}
export interface CompilerError extends SyntaxError {
  code: number | string
  loc?: SourceLocation | null
}
export function createCompilerError(
  code: ErrorCodes,
  loc?: SourceLocation | null,
  messages?: { [code: number]: string },
  additionalMessage?: string,
) {
  const msg = (messages || errorMessages)[code] + (additionalMessage || ``)
  const error = new SyntaxError(String(msg)) as CompilerError
  error.code = code
  error.loc = loc
  return error
}
