import { defineComponent } from 'vue'

export default defineComponent(() => {
  const name = 'Vue JSX'
  return () => {
    const a = undefined // Replace me with an a element
    return (
      <>
        <div>Hello {/* Use the name variable here */}!</div>
        {a}
      </>
    )
  }
})
