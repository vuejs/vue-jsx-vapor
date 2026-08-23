import { readFile, writeFile } from 'node:fs/promises'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const currentDir = dirname(fileURLToPath(import.meta.url))
const targetFiles = [
  resolve(currentDir, '../npm/wasm32-wasi/compiler-rs.wasi-browser.js'),
]
// Remove it to fix bugs for REPL.
const targetLine =
  'reuseWorker: { size: __asyncWorkPoolSize + __workerPoolSize },'

for (const targetFile of targetFiles) {
  let source

  try {
    source = await readFile(targetFile, 'utf8')
  } catch (error) {
    if (
      error &&
      typeof error === 'object' &&
      'code' in error &&
      error.code === 'ENOENT'
    ) {
      continue
    }
    throw error
  }

  if (!source.includes(targetLine)) {
    continue
  }

  await writeFile(targetFile, source.replace(targetLine, ''))
}
