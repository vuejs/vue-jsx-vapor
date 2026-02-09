import appCode from './app.tsx?raw'
import htmlCode from './index.html?raw'
import packageCode from './package.json?raw'
import tsMacroConfigCode from './ts-macro.config.ts?raw'
import TsconfigCode from './tsconfig.json?raw'
import confCode from './vite.config.ts?raw'

export default {
  'src/index.html': htmlCode,
  'src/App.tsx': appCode,
  'vite.config.ts': confCode,
  'ts-macro.config.ts': tsMacroConfigCode,
  'tsconfig.json': TsconfigCode,
  'package.json': packageCode,
}
