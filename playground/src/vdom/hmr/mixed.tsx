import { defineComponent, ref } from 'vue'

export const ObjComp = defineComponent(() => {
  const count = ref(0)
  return () => (
    <div id="obj-child">
      <button onClick={() => count.value++}>obj count: {count.value}</button>
    </div>
  )
})

export const FnChild = (props: { n: number }) => {
  return <div id="fn-child">fn-v1:{props.n}</div>
}
