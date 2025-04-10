import defineStyle from './define-style'
import jsxSortProps from './jsx-sort-props'
import type { DefineStyleRuleOptions } from './define-style/types'
import type { JsxSortPropsRuleOptions } from './jsx-sort-props/types'
import type { Linter } from 'eslint'

const ruleOptions = {
  'jsx-sort-props': jsxSortProps,
  'define-style': defineStyle,
}

export interface RuleOptions {
  'vue-jsx-vapor/jsx-sort-props': JsxSortPropsRuleOptions
  'vue-jsx-vapor/define-style': DefineStyleRuleOptions
}

export type Rules = Partial<{
  [K in keyof RuleOptions]:
    | Linter.RuleSeverity
    | [Linter.RuleSeverity, ...RuleOptions[K]]
}>

export default ruleOptions
