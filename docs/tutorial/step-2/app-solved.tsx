export default () => {
  const name = 'Vue JSX'
  const a = <a href="#">link</a>
  return (
    <>
      <div>Hello {name}!</div>
      {a}
    </>
  )
}
