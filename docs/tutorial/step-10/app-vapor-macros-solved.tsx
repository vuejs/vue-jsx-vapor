const Comp = () => {
  const slots = defineSlots({
    default: (props: { foo: string }) => <></>,
  })
  return <slots.default foo="from child" />
}

export default () => {
  return (
    <Comp v-slot={{ foo }}>
      <div>{foo}</div>
    </Comp>
  )
}
