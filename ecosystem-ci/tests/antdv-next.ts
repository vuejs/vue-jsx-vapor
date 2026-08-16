import { runInRepo } from '../utils.ts'
import type { RunOptions } from '../types.ts'

export default async (options: RunOptions) => {
  await runInRepo({
    ...options,
    repo: 'zhiyuanzmj/antdv-next',
    branch: 'vue-jsx-compiler',
    install: 'pnpm install --no-frozen-lockfile',
    test: 'test',
  })
}
