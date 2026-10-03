import { defineComponent, ref } from 'vue'
import { Child } from './child'
import { FnChild, ObjComp } from './mixed'

export const InlineChild = defineComponent((props: { n: number }) => {
  return () => <div id="child">inline vchild-v1:{props.n}</div>
})

export default defineComponent(() => {
  const show = ref(true)

  return () => (
    <div>
      <h3>vdom / interop: function component HMR</h3>
      <p>edit vchild-v1 to vchild-v2 in child.tsx and save - does the text update at all?</p>
      <button id="toggle" onClick={() => (show.value = !show.value)}>
        toggle
      </button>
      <input></input>
      {show.value && <Child n={9} />}
      <InlineChild n={9}></InlineChild>
      <ObjComp></ObjComp>
      <FnChild n={9}></FnChild>
    </div>
  )
})
