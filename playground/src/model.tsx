import { ref } from 'vue'

type ModelProps = {
  modelValue: string
  'onUpdate:modelValue'?: (value: string) => void
}

const Comp = (props: ModelProps) => (
  <input
    value={props.modelValue}
    onInput={(event) =>
      props['onUpdate:modelValue']?.(event.currentTarget.value)
    }
  />
)

export default () => {
  const model = ref('model')
  return (
    <>
      <Comp
        modelValue={model.value}
        onUpdate:modelValue={(value) => (model.value = value)}
      />
      {model.value}
    </>
  )
}
