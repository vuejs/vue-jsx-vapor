import { computed, ref, type Ref } from 'vue'

export function useRouteQuery<T extends string | boolean>(
  name: string,
  defaultValue?: T,
  reload = false,
) {
  const searchParams = new URLSearchParams(location.search)
  const data = searchParams.get(name)
  const value = ref(data || localStorage.getItem(name) || defaultValue)
  return computed({
    get() {
      return value.value === 'true'
        ? true
        : value.value === 'false'
          ? false
          : value.value
    },
    set(v) {
      const searchParams = new URLSearchParams(location.search)
      if (v === defaultValue) {
        searchParams.delete(name)
      } else {
        searchParams.set(name.toString(), v as string)
      }
      const url = `${location.pathname}${searchParams.size ? '?' : ''}${searchParams.toString()}`
      if (reload) location.replace(url)
      else history.replaceState({}, '', url + location.hash)
      localStorage.setItem(name, v)
      value.value = v
    },
  }) as unknown as Ref<T>
}
