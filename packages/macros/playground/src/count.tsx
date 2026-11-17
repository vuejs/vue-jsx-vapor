import { computed, defineVaporComponent } from 'vue'

export default defineVaporComponent(({ value = '' }) => {
  defineExpose({
    double: computed(() => +value * 2),
  })
  return <div>{value}</div>
})
