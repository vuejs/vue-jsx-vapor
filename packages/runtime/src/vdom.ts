import { getCurrentInstance } from 'vue'

const cacheMap = new WeakMap()

export function useVdomCache() {
  const i = getCurrentInstance()
  if (i) {
    !cacheMap.has(i) && cacheMap.set(i, [])
    const caches = cacheMap.get(i)
    return (caches[caches.length] = [])
  } else {
    return []
  }
}
