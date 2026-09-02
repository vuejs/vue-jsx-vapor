import { defineComponent } from 'vue'

export default defineComponent(() => {
  function onClick() {
    alert('clicked')
  }
  return () => <h1 onClick={onClick}>Click me!</h1>
})
