import { useSSRContext, type ComponentOptions } from 'vue'

export function ssrRegisterHelper(comp: ComponentOptions, filename: string) {
  if (typeof comp === 'function') {
    // @ts-ignore
    comp.__setup = () => {
      const ssrContext = useSSRContext()!
      ;(ssrContext.modules || (ssrContext.modules = new Set())).add(filename)
    }
  } else {
    const setup = comp.setup
    comp.setup = (props, ctx) => {
      const ssrContext = useSSRContext()!
      ;(ssrContext.modules || (ssrContext.modules = new Set())).add(filename)
      if (setup) {
        return setup(props, ctx)
      }
    }
  }
}
