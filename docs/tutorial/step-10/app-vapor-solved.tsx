const Comp = (props, { slots }) => {
  return <slots.default foo="from child" />
}

export default () => {
  return <Comp>{(slotProps) => <div>{slotProps.foo}</div>}</Comp>
}
