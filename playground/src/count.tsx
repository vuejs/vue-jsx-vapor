import { computed, type Ref } from 'vue'

export default (
  props: { value: string },
  { expose }: { expose: (value: { double: Ref<number> }) => void },
) => {
  expose({ double: computed(() => +props.value * 2) })
  return <div>{props.value}</div>
}
