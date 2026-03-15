// @ts-ignore
import JSConfetti from 'https://esm.sh/js-confetti'

export default () => {
  const confetti = new JSConfetti()

  function showConfetti() {
    confetti.addConfetti()
  }

  showConfetti()
  return (
    <h1 style="text-align: center; margin-top: 3em;" onClick={showConfetti}>
      🎉 Congratulations!
    </h1>
  )
}
