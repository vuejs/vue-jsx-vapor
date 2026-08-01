export function Comp() {
  defineExpose({
    foo: 1,
  })
  return <div />
}

export const Comp1 = function (props: any) {
  defineExpose({
    foo: props.foo,
  })
  return <div />
}

export const Comp2 = ({ foo }: any) => {
  defineExpose({
    foo,
  })
  return <div />
}

export const Comp3 = () => {
  defineExpose()
  return <div />
}

export const Comp4 = () => {
  defineExpose<{ foo: number }>({ foo: 1 })
  return <div />
}

export const Comp5 = () => {
  defineExpose<{ foo?: number }>()
  return <div />
}
