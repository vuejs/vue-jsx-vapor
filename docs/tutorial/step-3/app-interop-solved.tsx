import { defineComponent } from 'vue'
import './main.css'

export default defineComponent(() => {
  const titleClass = 'title'
  return () => <h1 class={titleClass}>Make me red</h1>
})
