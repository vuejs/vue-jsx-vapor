import { ref, defineComponent } from 'vue'

export default defineComponent(() => {
  const count = ref(0)
  let dynamic = 0
  let stable = 0
  return () => {
    const offset = 1 // render-local: referencing it makes a slot dynamic
    return (
      <main class="demo">
        <button onClick={() => count.value++}>rerender × {count.value}</button>
        <Output label="dynamic">{() => (dynamic += offset)}</Output>
        <Output label="stable">{() => (stable += 1)}</Output>
      </main>
    )
  }
})

function Output ({ label }: { label: string }) { 
  return (
    <div class="output">
      {label}: <slot />
    </div>
  )
}
