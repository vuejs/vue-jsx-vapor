import _defineStyle from './define-style/index.ts'
import jsxSortProps from './jsx-sort-props/index.ts'
import type { DefineStyleRuleOptions } from './define-style/types'
import type { JsxSortPropsRuleOptions } from './jsx-sort-props/types'
import type { Linter } from '@typescript-eslint/utils/ts-eslint'

const ruleOptions = {
  'jsx-sort-props': jsxSortProps,
  'define-style': _defineStyle,
}

export interface RuleOptions {
  'vue-jsx-vapor/jsx-sort-props': JsxSortPropsRuleOptions
  'vue-jsx-vapor/define-style': DefineStyleRuleOptions
}

export type Rules = Partial<{
  [K in keyof RuleOptions]: Linter.Severity | [Linter.Severity, ...RuleOptions[K]]
}>

export default ruleOptions
