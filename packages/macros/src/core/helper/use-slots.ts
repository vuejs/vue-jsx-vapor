import { useSlots as _useSlots } from 'vue'

export function useSlots(defaultSlots: Record<string, any> = {}) {
  const slots = _useSlots()
  return new Proxy(defaultSlots, {
    get(target, key: string) {
      return key in slots ? slots[key] : target[key]
    },
  })
}
