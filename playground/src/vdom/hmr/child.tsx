import { defineComponent } from 'vue'

export const Child = defineComponent((props: { n: number }) => {
  return () => <div id="child">vchild-v1:{props.n}</div>
})
