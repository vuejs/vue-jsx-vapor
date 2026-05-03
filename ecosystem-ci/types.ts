export interface RepoOptions {
  repo: string
  branch?: string
  tag?: string
  commit?: string
  dir?: string
  shallow?: boolean
  env?: Record<string, string>
}

export interface RunOptions {
  root: string
  workspace: string
  release?: string
  skipGit?: boolean
}

export interface Overrides {
  [pkg: string]: string
}

export interface SuiteOptions extends RunOptions, RepoOptions {
  build?: string | string[]
  test?: string | string[]
  install?: string
  overrides?: Overrides
}
