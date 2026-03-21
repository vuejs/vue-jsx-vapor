import { Fragment, ref } from 'vue'
import { h } from 'vue-jsx-vapor'

export default () => {
  const count = ref(1)

  return h(
    Fragment,
    null,
    [
      h('input', {
        type: 'number',
        value: () => count.value,
        onInput: (e) => count.value = +e.target.value
      }),
      () => h(`h${count.value}`, null, [() => `h${count.value}`]),
    ],
  )
}
