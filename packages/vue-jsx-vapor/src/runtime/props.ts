import { proxyRefs, toRefs, useAttrs } from 'vue'
import * as Vue from 'vue'

export function getCurrentInstance(): // @ts-expect-error Suppors vue3.6 and below versions
import('vue').GenericComponentInstance | null {
  // @ts-ignore
  return Vue.currentInstance || Vue.getCurrentInstance()
}

/**
 * Returns the props of the current component instance.
 *
 * @example
 * ```tsx
 * import { useProps } from 'vue-jsx-vapor'
 *
 * defineComponent(({ foo = '' })=>{
 *   const props = useProps() // { foo: '' }
 * })
 * ```
 */
export function useProps() {
  const i = getCurrentInstance()
  return i!.props
}

/**
 * Returns the merged props and attrs of the current component.\
 * Equivalent to `useProps()` + `useAttrs()`.
 *
 * @example
 * ```tsx
 * import { useFullProps } from 'vue-jsx-vapor'
 *
 * defineComponent((props) => {
 *   const fullProps = useFullProps() // = useAttrs() + useProps()
 * })
 */
export function useFullProps() {
  return proxyRefs({
    ...toRefs(useProps()),
    ...toRefs(useAttrs()),
  })
}
