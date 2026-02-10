import './main.css'

export default () => {
  const titleClass = 'title-bg'
  function onClick() {
    alert('clicked')
  }
  return (
    <h1 class={titleClass} onClick={onClick}>
      Make me red
    </h1>
  )
}
