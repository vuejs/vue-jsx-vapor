import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'
import { execa } from 'execa'

import type { Overrides, RepoOptions, SuiteOptions } from './types.ts'

const isGitHubActions = !!process.env.GITHUB_ACTIONS

export const VUE_JSX_VAPOR_PACKAGES = ['vue-jsx-vapor']

export const root = path.dirname(fileURLToPath(import.meta.url))
export const workspace = path.resolve(root, 'workspace')

let cwd = process.cwd()

export function cd(dir: string) {
  cwd = path.resolve(cwd, dir)
}

// ─── shell helpers ────────────────────────────────────────────────────────────

export async function $(cmd: string, args: string[] = [], env = {}) {
  if (isGitHubActions) {
    console.log(
      `\u001B[1;34m::group::${cwd} $> ${cmd} ${args.join(' ')}\u001B[0m`,
    )
  } else {
    console.log(`\u001B[1;34m${cwd} $> ${cmd} ${args.join(' ')}\u001B[0m`)
  }

  await execa(cmd, args, { cwd, env, stdio: 'inherit' })

  if (isGitHubActions) {
    console.log('::endgroup::')
  }
}

export async function $output(cmd: string, args: string[] = [], env = {}) {
  const { stdout } = await execa(cmd, args, { cwd, env, stdio: 'pipe' })
  return stdout.trim()
}

// ─── git helpers ──────────────────────────────────────────────────────────────

export async function setupRepo(options: RepoOptions) {
  const { repo, tag, commit, shallow = true } = options
  const branch = options.branch ?? 'main'
  const dir = path.resolve(workspace, options.dir ?? repo.split('/').at(-1)!)

  const repoUrl = repo.includes(':') ? repo : `https://github.com/${repo}.git`

  let needClone = true
  if (fs.existsSync(dir)) {
    const _cwd = cwd
    cd(dir)
    try {
      const remoteUrl = await $output(
        'git',
        ['ls-remote', '--get-url'],
        options.env,
      )
      if (remoteUrl === repoUrl) needClone = false
      else fs.rmSync(dir, { recursive: true, force: true })
    } catch {
      fs.rmSync(dir, { recursive: true, force: true })
    }
    cd(_cwd)
  }

  if (needClone) {
    await $(
      'git',
      [
        '-c',
        'advice.detachedHead=false',
        'clone',
        ...(shallow ? ['--depth=1', '--no-tags'] : []),
        '--branch',
        tag ?? branch,
        repoUrl,
        dir,
      ],
      options.env,
    )
  }

  cd(dir)
  await $('git', ['clean', '-fdxq'], options.env)
  await $(
    'git',
    [
      'fetch',
      ...(shallow ? ['--depth=1', '--no-tags'] : ['--tags']),
      'origin',
      tag ? `tag ${tag}` : (commit ?? branch),
    ],
    options.env,
  )
  await $(
    'git',
    [
      '-c',
      'advice.detachedHead=false',
      'checkout',
      tag ? `tags/${tag}` : (commit ?? branch),
    ],
    options.env,
  )
}

// ─── override helpers ─────────────────────────────────────────────────────────

export function buildOverrides(release?: string): Overrides {
  const overrides: Overrides = {}

  for (const pkg of VUE_JSX_VAPOR_PACKAGES) {
    if (release) {
      overrides[pkg] = release.startsWith('@')
        ? `https://pkg.pr.new/${pkg}${release}`
        : release
    } else {
      const dirName =
        pkg === 'vue-jsx-vapor'
          ? 'vue-jsx-vapor'
          : pkg.replace('@vue-jsx-vapor/', '')
      const pkgPath = path.resolve(root, '..', 'packages', dirName)
      if (fs.existsSync(pkgPath)) {
        overrides[pkg] = `link:${pkgPath}`
      }
    }
  }

  return overrides
}

export function applyOverrides(dir: string, overrides: Overrides) {
  const pkgPath = path.join(dir, 'package.json')
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'))

  pkg.pnpm ??= {}
  pkg.pnpm.overrides = { ...pkg.pnpm.overrides, ...overrides }

  const workspacePath = path.join(dir, 'pnpm-workspace.yaml')
  if (fs.existsSync(workspacePath)) {
    const content = fs.readFileSync(workspacePath, 'utf8')
    const suffix = content.includes('dangerouslyAllowAllBuilds')
      ? ''
      : '\ndangerouslyAllowAllBuilds: true\n'
    fs.writeFileSync(workspacePath, content + suffix)
  }

  fs.writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`)
  console.log('Applied overrides:', overrides)
}

// ─── run helpers ─────────────────────────────────────────────────────────────

async function runTasks(
  tasks: string | string[],
  scripts: Record<string, string>,
) {
  const list = Array.isArray(tasks) ? tasks : [tasks]
  for (const task of list) {
    if (scripts[task] == null) {
      const [cmd, ...args] = task.split(' ')
      await $(cmd, args)
    } else {
      await $('pnpm', ['run', task])
    }
  }
}

export async function runInRepo(options: SuiteOptions) {
  const {
    repo,
    branch,
    tag,
    commit,
    skipGit = false,
    build,
    test,
    release,
    env,
    install,
    overrides: extraOverrides = {},
  } = options

  const dir = path.resolve(workspace, options.dir ?? repo.split('/').at(-1)!)

  if (skipGit) {
    cd(dir)
  } else {
    await setupRepo({
      repo,
      branch,
      tag,
      commit,
      env,
      dir: path.relative(workspace, dir),
    })
  }

  const pkgPath = path.join(dir, 'package.json')
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'))

  await $('git', ['clean', '-fdxq'], env)
  const overrides = { ...buildOverrides(release), ...extraOverrides }
  await applyOverrides(dir, overrides)

  if (install) {
    const [cmd, ...args] = install.split(' ')
    await $(cmd, args, env)
  } else {
    await $('pnpm', ['install'], env)
  }

  if (build) await runTasks(build, pkg.scripts ?? {})
  if (test) await runTasks(test, pkg.scripts ?? {})
}
