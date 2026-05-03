import { runInRepo } from '../utils.ts'
import type { RunOptions } from '../types.ts'

export default async (options: RunOptions) => {
  await runInRepo({
    ...options,
    repo: 'zhiyuanzmj/naive-ui',
    branch: 'vue-jsx-compiler',
    install: 'pnpm install --ignore-workspace',
    env: {
      TZ: 'Asia/Shanghai',
    },
    test: 'test',
  })
}
