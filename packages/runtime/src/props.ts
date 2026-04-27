import { computed, useAttrs } from 'vue'
import * as Vue from 'vue'

/*@__NO_SIDE_EFFECTS__*/
export function getCurrentInstance():
  | import('vue').GenericComponentInstance
  | null {
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
/*@__NO_SIDE_EFFECTS__*/
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
/*@__NO_SIDE_EFFECTS__*/
export function useFullProps() {
  const attrs = useAttrs()
  const i = getCurrentInstance()!
  // @ts-ignore
  if (!i.type.props) {
    return attrs
  }
  const props = useProps()
  const fullProps = computed(() => ({ ...props, ...attrs }))
  return new Proxy(
    {},
    {
      get(_, p, receiver) {
        return Reflect.get(fullProps.value, p, receiver)
      },
      set(_, p, v, r) {
        return Reflect.set(fullProps.value, p, v, r)
      },
      deleteProperty(_, p) {
        return Reflect.deleteProperty(fullProps.value, p)
      },
      has(_, p) {
        return Reflect.has(fullProps.value, p)
      },
      ownKeys() {
        return Object.keys(fullProps.value)
      },
      getOwnPropertyDescriptor() {
        return {
          enumerable: true,
          configurable: true,
        }
      },
    },
  )
}
