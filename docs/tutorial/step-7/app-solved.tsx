import { defineComponent } from 'vue'
import Child from './Child'

export default defineComponent(() => {
  return () => (
    <div>
      Parent
      <Child />
    </div>
  )
})
