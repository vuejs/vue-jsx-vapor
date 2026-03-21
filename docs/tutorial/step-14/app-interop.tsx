import { defineComponent, Fragment, ref, h } from 'vue'

export default defineComponent(() => {
  const count = ref(1)

  return () => h(
    Fragment,
    null,
    [
      h('input', {
        type: 'number',
        value: count.value,
        onInput: (e) => count.value = +e.target.value
      }),
      h(`h${count.value}`, null, [`h${count.value}`]),
    ],
  )
})
