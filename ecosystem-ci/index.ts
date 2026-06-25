import fs from 'node:fs'
import path from 'node:path'
import { cwd, exit } from 'node:process'
import { cac } from 'cac'
import { execa } from 'execa'
import { root, workspace } from './utils.ts'
import type { RunOptions } from './types.ts'

const cli = cac()

cli
  .command('[...suites]', 'run selected suites against vue-jsx-vapor')
  .option('--branch <branch>', 'vue-jsx-vapor branch to use', {
    default: 'main',
  })
  .option('--tag <tag>', 'vue-jsx-vapor tag to use')
  .option('--commit <commit>', 'vue-jsx-vapor commit sha to use')
  .action(async (suites: string[], options: RunOptions) => {
    fs.mkdirSync(workspace, { recursive: true })

    const suitesToRun = getSuitesToRun(suites)
    const runOptions = {
      ...options,
      root,
      workspace,
    }

    await execa(
      'pnpm',
      ['-C', './packages/vue-jsx-vapor', 'pack', '--out', '%s.tgz'],
      {
        cwd: path.dirname(cwd()),
        stdio: 'inherit',
      },
    )

    for (const suite of suitesToRun) {
      console.log(`\n${'─'.repeat(60)}`)
      console.log(`▶  Running suite: ${suite}`)
      console.log(`${'─'.repeat(60)}\n`)
      const { default: test } = await import(`./tests/${suite}.ts`)
      await test({
        ...runOptions,
        workspace: path.resolve(workspace, suite),
      })
    }

    console.log('\n✅  All suites passed!')
  })

cli.help()
cli.parse()

// ─────────────────────────────────────────────────────────────────────────────

function getSuitesToRun(suites: string[]): string[] {
  const available = fs
    .readdirSync(path.join(root, 'tests'))
    .filter((f) => !f.startsWith('_') && f.endsWith('.ts'))
    .map((f) => f.slice(0, -3))
    .sort()

  if (suites.length === 0) return available

  const invalid = suites.filter((s) => !available.includes(s))
  if (invalid.length) {
    console.error(`Invalid suite(s): ${invalid.join(', ')}`)
    console.error(`Available: ${available.join(', ')}`)
    exit(1)
  }
  return suites
}
