import { defineComponent } from 'vue'
import Child from './Child'

export default defineComponent(() => {
  return () => (
    <>
      Parent
      <Child />
    </>
  )
})
