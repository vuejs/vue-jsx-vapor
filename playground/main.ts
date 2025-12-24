import { createVaporApp, vaporInteropPlugin } from 'vue'

const modules = import.meta.glob<any>('./src/*.tsx')
const mod = (
  modules[`./src${location.pathname}.tsx`] || modules['./src/App.tsx']
)()

mod.then(({ default: mod }) => {
  const app = createVaporApp(mod)
  if (mod.name === 'interop') {
    app.use(vaporInteropPlugin)
  }
  app.mount('#app')
  // @ts-expect-error
  globalThis.unmount = () => {
    app.unmount()
  }
})
