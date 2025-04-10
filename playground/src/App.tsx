import { defineVaporComponent, ref, type Ref } from 'vue'
import { useRef } from 'vue-jsx-vapor'
import Count2 from './count'
import For from './for'
import Html from './html'
import If from './if'
import Model from './model'
import Once from './once'
import Show from './show'
import Slot from './slot'

export default defineVaporComponent(() => {
  const count = ref('1')

  const Count = (props: { value: string }) => {
    return <div>{props.value}</div>
  }

  const Count1 = ({ value }: { value: Ref<string> }) => {
    return <div>{value.value}</div>
  }

  const compRef = useRef()

  return (
    <>
      <fieldset>
        <legend>Component</legend>
        <input
          value_prop={count.value}
          onInput={(e) => (count.value = e.currentTarget.value)}
        />

        <Count value={count.value} />
        <Count1 value={count} />
        <Count2 ref={compRef} value={count.value} />
        {compRef.value?.double}
      </fieldset>

      <fieldset>
        <legend>v-if</legend>
        <If />
      </fieldset>

      <fieldset>
        <legend>v-for</legend>
        <For />
      </fieldset>

      <fieldset>
        <legend>v-slot</legend>
        <Slot />
      </fieldset>

      <fieldset>
        <legend>v-model</legend>
        <Model />
      </fieldset>

      <fieldset>
        <legend>v-show</legend>
        <Show />
      </fieldset>

      <fieldset>
        <legend>v-html</legend>
        <Html />
      </fieldset>

      <fieldset>
        <legend>v-once</legend>
        <Once />
      </fieldset>
    </>
  )
})
