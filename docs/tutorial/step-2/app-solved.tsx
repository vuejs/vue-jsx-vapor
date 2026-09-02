import { defineComponent } from 'vue'

export default defineComponent(() => {
  const name = 'Vue JSX'
  return () => {
    const a = <a href="#">link</a>
    return (
      <>
        <div>Hello {name}!</div>
        {a}
      </>
    )
  }
})
