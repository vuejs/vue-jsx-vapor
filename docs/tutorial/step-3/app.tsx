import { defineComponent } from 'vue'

export default defineComponent(() => {
  const titleClass = 'title'
  return () => (
    <>
      <h1>Make me red</h1>

      <style>{`
        .title {
          color: red;
        }
      `}</style>
    </>
  )
})
