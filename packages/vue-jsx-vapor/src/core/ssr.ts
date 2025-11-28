import type { ComponentOptions } from 'vue'

export const ssrRegisterHelperId = '/__vue-jsx-ssr-register-helper'
export const ssrRegisterHelperCode =
  `import { useSSRContext } from "vue"\n` +
  // the const here is just to work around the Bun bug where
  // Function.toString() isn't working as intended
  // https://github.com/oven-sh/bun/issues/9543
  `export const ssrRegisterHelper = ${ssrRegisterHelper.toString()}`

/**
 * This function is serialized with toString() and evaluated as a virtual
 * module during SSR
 */
function ssrRegisterHelper(comp: ComponentOptions, filename: string) {
  if (typeof comp === 'function') {
    // @ts-ignore
    comp.__setup = () => {
      // @ts-ignore
      const ssrContext = useSSRContext()
      ;(ssrContext.modules || (ssrContext.modules = new Set())).add(filename)
    }
  } else {
    const setup = comp.setup
    comp.setup = (props, ctx) => {
      // @ts-ignore
      const ssrContext = useSSRContext()
      ;(ssrContext.modules || (ssrContext.modules = new Set())).add(filename)
      if (setup) {
        return setup(props, ctx)
      }
    }
  }
}
