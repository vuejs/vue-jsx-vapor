import { ref } from 'vue'

export default () => {
  const count = ref(0)
  const ok = ref(true)

  return (
    <section class="demo">
      <p class={{ active: ok.value }}>count: {count.value}</p>
      {ok.value ? <p>on</p> : <p>off</p>}
      <button onClick={() => count.value++}>increment</button>
      <button onClick={() => (ok.value = !ok.value)}>toggle</button>
    </section>
  )
}
