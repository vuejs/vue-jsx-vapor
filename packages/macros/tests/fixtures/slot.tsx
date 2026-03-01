import { defineComponent, defineVaporComponent } from 'vue';

const Comp = defineComponent((props: {text?: string}) => {
  const computedText = () => {
   return props.text
  }
  return () => <div><slot foo="" /></div>
})
;<Comp>
  {{ default: (props) => [props.foo === ''] }}
</Comp>

const Comp1 = defineVaporComponent((props: {text?: string}) => {
  const computedText = () => {
   return props.text
  }
  return <div><slot foo="" /></div>
}) 
;<Comp1>
  {{ default: (props) => [props.foo === ''] }}
</Comp1>

const Comp2 = (props: {text?: string}) => {
  const computedText = () => {
   return props.text
  }
  return <div><slot foo="" /></div>
}
;<Comp2>
  {{ default: (props) => [props.foo === ''] }}
</Comp2>
