import { defineComponent, ref } from 'vue'
export default defineComponent(() => {
  const isDone = ref(false)
  const text = ref('learn Vue JSX')
  return () => (
    <section class="demo">
      <p class={{ done: isDone.value }}>{text.value}</p>
      <button onClick={() => (isDone.value = !isDone.value)}>toggle</button>
      <footer>static</footer>
    </section>
  )
})
