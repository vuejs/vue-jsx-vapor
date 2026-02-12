const Comp = (props, { slots }) => {
  return <slots.default foo="from child" />
}

export default () => {
  return <Comp>{() => <div>{/* ... */}</div>}</Comp>
}
