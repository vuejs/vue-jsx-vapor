# Volar 插件

指令和宏都在编译阶段完成转换，运行时不需要任何编辑器插件。`vue-jsx/volar` 插件是可选的——只有在需要让这些语法在编辑器中被识别、或在命令行类型检查中通过时，才需要安装它。

插件提供以下两类语法的类型支持：

- **指令**与 **`ref`**：插件覆盖哪些指令，参见[指令](../features/directives)中的支持表。
- **宏**：`defineModel`、`defineSlots`、`defineExpose`、`defineStyle`。默认关闭，需要通过 `macros` 选项开启，参见[宏](../features/macros)。

同一个模块对应两种装配方式，按你的 TSX 写在哪里来选择。

## TSX 文件：TS Macro

`.tsx` 文件请安装 [TS Macro](https://marketplace.visualstudio.com/items?itemName=zhiyuanzmj.vscode-ts-macro)，并创建 `ts-macro.config.ts`：

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx()],
}
```

如果在 Vite 插件中启用了宏，这里也要同步开启宏转换，并保持两边选项一致，参见[宏](../features/macros)：

```ts [ts-macro.config.ts]
import vueJsx from 'vue-jsx/volar'

export default {
  plugins: [vueJsx({ macros: true })],
}
```

### 命令行类型检查

安装 `@ts-macro/tsc`，并用 `tsmc` 替代 `tsc`：

```bash
pnpm add -D @ts-macro/tsc
```

```json [package.json]
{
  "scripts": {
    "typecheck": "tsmc --noEmit"
  }
}
```

`tsmc` 读取的就是同一份 `ts-macro.config.ts`，无需额外配置。

## SFC 文件：Vue Language Tools

`.vue` 文件由 Vue Language Tools（官方 Vue 扩展与 `vue-tsc`）负责，而不是 TS Macro。请在 `tsconfig.json` 中把同一个模块注册为 Vue Language Tools 插件：

```json [tsconfig.json]
{
  "compilerOptions": {
    "jsx": "preserve",
    "jsxImportSource": "vue-jsx"
  },
  "vueCompilerOptions": {
    "plugins": ["vue-jsx/volar"]
  }
}
```

插件会作用于所有使用 TSX 的 `<script>` 块，因此 `<script setup lang="tsx">` 和 `<script lang="tsx">` 都被覆盖。命令行类型检查依旧用你已有的 `vue-tsc`——编辑器与 `vue-tsc` 走的是同一份配置：

```json [package.json]
{
  "scripts": {
    "typecheck": "vue-tsc --noEmit"
  }
}
```

::: tip
用 `tsmc` 检查 SFC 项目是行不通的：TS Macro 不解析 `.vue` 文件。`.tsx` 用 `tsmc`，SFC 用 `vue-tsc`。
:::

## 选项

指令支持始终开启。`ref` 与 `macros` 是两个开关，两种装配方式接受的选项形状完全相同——在 `ts-macro.config.ts` 里作为工厂函数的入参，在 `tsconfig.json` 里写在 `vueCompilerOptions['vue-jsx']` 下（使用 `vue-jsx-vapor` 时键名为 `vue-jsx-vapor`）：

```json [tsconfig.json]
{
  "vueCompilerOptions": {
    "plugins": ["vue-jsx/volar"],
    "vue-jsx": {
      "macros": true
    }
  }
}
```

|     选项     |  默认值  | 说明                                                   |
| :----------: | :------: | :----------------------------------------------------- |
| `directives` | 始终启用 | 为 `v-if`、`v-for`、`v-slot`、`v-model` 提供类型支持。 |
|    `ref`     |  `true`  | 设为 `false` 关闭；传对象可配置别名。                  |
|   `macros`   | `false`  | 设为 `true` 或选项对象以启用宏语法。                   |

::: warning
在这里开启宏只影响类型检查。让宏在运行时真正生效的转换由 Vite 插件完成——两边请保持配置一致，参见[宏](../features/macros)。
:::

## 相关页面

- [指令](../features/directives)：各指令的编译产物，以及插件覆盖哪些指令。
- [宏](../features/macros)：在 Vite 插件中开启宏转换。
- [扩展 JSX 类型](./extending-jsx-types)：插件生成的代码最终也解析自 JSX namespace。
