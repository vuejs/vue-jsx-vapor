import { runInRepo } from '../utils.ts'
import type { RunOptions } from '../types.ts'

export default async (options: RunOptions) => {
  // await runInRepo({
  //   ...options,
  //   repo: 'zhiyuanzmj/vuetify',
  //   branch: 'vue-jsx-compiler',
  //   install: 'pnpm install --no-frozen-lockfile',
  //   env: { CI: 'false' },
  //   test: 'vue-ecosystem-ci:test',
  // })
}
