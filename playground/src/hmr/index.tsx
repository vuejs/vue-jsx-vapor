import { ref } from 'vue'
import { Child } from './child'

export const InlineChild = (props: { n: number }) => {
  return <div id="child">inline-child-v1:{props.n}</div>
}

export default () => {
  const typed = ref('')
  const show = ref(true)

  return (
    <div>
      <h3>vapor: function component HMR</h3>
      <ol>
        <li>edit child-v1 to child-v2 in child.tsx and save - it should update in place</li>
        <li>click the button to unmount/remount - does it stay child-v2 or revert to child-v1?</li>
      </ol>
      <input v-model={typed.value} id="typed" placeholder="parent state" />
      <button id="toggle" onClick={() => (show.value = !show.value)}>
        {show.value ? 'unmount' : 'mount'} Child
      </button>
      {show.value && <Child n={5} />}
      <InlineChild n={5}></InlineChild>
    </div>
  )
}
