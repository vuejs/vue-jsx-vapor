import { customRef, watchSyncEffect, type ModelRef } from 'vue'

type DefineModelOptions<T = Record<string, any>> = {
  default?: any
  get?: (v: T) => any
  set?: (v: T) => any
}

const EMPTY_OBJ = {}

export function useModel<
  M extends PropertyKey,
  T extends Record<string, any>,
  K extends keyof T,
>(props: T, name: K, options?: DefineModelOptions<T[K]>): ModelRef<T[K], M>
export function useModel(
  props: Record<string, any>,
  name: string,
  options: DefineModelOptions = {},
): any {
  const res = customRef((track, trigger) => {
    let localValue: any = options && options.default
    let prevSetValue = EMPTY_OBJ

    watchSyncEffect(() => {
      let propValue = props[name]
      if (propValue === undefined) {
        propValue = options && options.default
      }
      if (!Object.is(localValue, propValue)) {
        localValue = propValue
        trigger()
      }
    })

    return {
      get() {
        track()
        return options.get ? options.get(localValue) : localValue
      },

      set(value) {
        const emittedValue = options.set ? options.set(value) : value
        if (
          Object.is(emittedValue, localValue) &&
          (prevSetValue === EMPTY_OBJ || Object.is(value, prevSetValue))
        )
          return
        localValue = emittedValue
        trigger()
        for (const emit of [props[`onUpdate:${name}`]].flat()) {
          if (typeof emit === 'function') emit(emittedValue)
        }
        prevSetValue = value
      },
    }
  })

  // @ts-expect-error
  res[Symbol.iterator] = () => {
    let i = 0
    return {
      next() {
        if (i < 2) {
          return {
            value: i++ ? props[`${name}Modifiers`] || {} : res,
            done: false,
          }
        } else {
          return { done: true }
        }
      },
    }
  }
  return res
}
