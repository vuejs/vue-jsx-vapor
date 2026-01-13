import { createApp, createVaporApp, vaporInteropPlugin } from 'vue'

const modules = import.meta.glob<any>('./src/**/*.tsx')
const mod = (
  modules[`./src${location.pathname}.tsx`] || modules['./src/App.tsx']
)()

mod.then(({ default: mod }) => {
  if (mod.setup && !mod.__vapor) {
    const app = createApp(mod)
    app.mount('#app')
    // @ts-expect-error
    globalThis.unmount = () => {
      app.unmount()
    }
  } else {
    const app = createVaporApp(mod)
    if (mod.name === 'interop') {
      app.use(vaporInteropPlugin)
    }
    app.mount('#app')
    // @ts-expect-error
    globalThis.unmount = () => {
      app.unmount()
    }
  }
})
