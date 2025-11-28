import {
  instantiateNapiModuleSync,
  MessageHandler,
  WASI,
} from '@napi-rs/wasm-runtime'

const handler = new MessageHandler({
  onLoad({ wasmModule, wasmMemory }) {
    const wasi = new WASI({
      print() {
        // eslint-disable-next-line prefer-spread, prefer-rest-params, no-console
        console.log.apply(console, arguments)
      },
      printErr() {
        // eslint-disable-next-line prefer-spread, prefer-rest-params
        console.error.apply(console, arguments)
      },
    })
    return instantiateNapiModuleSync(wasmModule, {
      childThread: true,
      wasi,
      overwriteImports(importObject) {
        importObject.env = {
          ...importObject.env,
          ...importObject.napi,
          ...importObject.emnapi,
          memory: wasmMemory,
        }
      },
    })
  },
})

// eslint-disable-next-line unicorn/prefer-add-event-listener
globalThis.onmessage = function (e) {
  handler.handle(e)
}
