import { readFile, writeFile } from 'node:fs/promises'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const currentDir = dirname(fileURLToPath(import.meta.url))
// The in-package artifact is what bundlers resolve through the `browser` field
// (and what pkg-pr-new previews ship); the npm/ one is what `napi artifacts`
// stages for the published wasm32-wasi subpackage. Both carry the same line.
const targetFiles = [
  resolve(currentDir, '../compiler.wasi-browser.js'),
  resolve(currentDir, '../npm/wasm32-wasi/compiler.wasi-browser.js'),
]
// Remove it to fix bugs for REPL.
const targetLine = 'reuseWorker: { size: __asyncWorkPoolSize + __workerPoolSize },'

for (const targetFile of targetFiles) {
  let source

  try {
    source = await readFile(targetFile, 'utf8')
  } catch (error) {
    if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT') {
      continue
    }
    throw error
  }

  if (!source.includes(targetLine)) {
    continue
  }

  await writeFile(targetFile, source.replace(targetLine, ''))
}
