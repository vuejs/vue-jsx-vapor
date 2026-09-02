import { defineVaporComponent, ref, type Ref } from 'vue'
import { useRef } from 'vue-jsx'
import Count from './count'
import Expose from './expose'
import For from './for'
import Html from './html'
import If from './if'
import Model from './model'
import Once from './once'
import Show from './show'
import Slot from './slot'

export default defineVaporComponent(() => {
  const count = ref('1')
  const countRef = useRef()

  const Value = (props: { value: string }) => <div>{props.value}</div>
  const RefValue = ({ value }: { value: Ref<string> }) => (
    <div>{value.value}</div>
  )

  return (
    <>
      <fieldset>
        <input
          value={count.value}
          onInput={(event) => (count.value = event.currentTarget.value)}
        />
        <Value value={count.value} />
        <RefValue value={count} />
        <Count ref={countRef} value={count.value} />
        {countRef.value?.double}
        <Expose />
      </fieldset>

      <fieldset>
        <legend>conditional rendering</legend>
        <If />
      </fieldset>
      <fieldset>
        <legend>list rendering</legend>
        <For />
      </fieldset>
      <fieldset>
        <legend>slots</legend>
        <Slot />
      </fieldset>
      <fieldset>
        <legend>model</legend>
        <Model />
      </fieldset>
      <fieldset>
        <legend>show</legend>
        <Show />
      </fieldset>
      <fieldset>
        <legend>HTML</legend>
        <Html />
      </fieldset>
      <fieldset>
        <legend>once</legend>
        <Once />
      </fieldset>
    </>
  )
})
