export default () => {
  function onClick() {
    alert('clicked')
  }
  return <h1 onClick={onClick}>Click me!</h1>
}
