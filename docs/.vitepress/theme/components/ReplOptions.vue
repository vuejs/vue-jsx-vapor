<script setup lang="ts">
import { VTSwitch } from '@vue/theme'
import { watch, type PropType } from 'vue'
// @ts-ignore
import interopHtmlCode from '../../../tutorial/template/index-interop.html?raw'
// @ts-ignore
import htmlCode from '../../../tutorial/template/index.html?raw'

const props = defineProps({
  files: {
    type: Object,
    required: true,
  },
  apps: {
    type: Object as PropType<{
      app: object
      solved: object
      interop?: object
      interopSolved?: object
      macros?: object
      macrosSolved?: object
      interopMacros?: object
      interopMacrosSolved?: object
    }>,
    required: true,
  },
})

const solved = defineModel<boolean>('solved', { required: true })
const interop = defineModel<boolean>('interop', { required: true })
const macros = defineModel<boolean>('macros', { required: true })

watch(
  () => [interop.value, macros.value, solved.value],
  ([interop, macros]) => {
    props.files['vite.config.ts'] = props.files['vite.config.ts'].replace(
      /(?<=interop: )(true|false)/,
      interop.toString(),
    )
    props.files['vite.config.ts'] = props.files['vite.config.ts'].replace(
      /(?<=macros: )(true|false)/,
      macros.toString(),
    )

    props.files['ts-macro.config.ts'] = props.files[
      'ts-macro.config.ts'
    ].replace(/(?<=interop: )(true|false)/, interop.toString())
    props.files['ts-macro.config.ts'] = props.files[
      'ts-macro.config.ts'
    ].replace(/(?<=macros: )(true|false)/, macros.toString())

    props.files['src/index.html'] = interop ? interopHtmlCode : htmlCode

    const tsConfig = JSON.parse(props.files['tsconfig.json'])
    tsConfig.compilerOptions.jsxImportSource = interop ? 'vue' : 'vue-jsx-vapor'
    if (interop) {
      tsConfig.compilerOptions.types.push('vue/jsx')
    }
    props.files['tsconfig.json'] = JSON.stringify(tsConfig, null, 2)
  },
  { immediate: true, deep: true },
)
</script>

<template>
  <div class="repl-options">
    <div class="repl-options-left">
      <template v-if="!!apps.interop">
        <VTSwitch
          aria-label="prefer interop option"
          :class="{ 'prefer-interop': interop }"
          :aria-checked="interop"
          @click="interop = !interop"
        />
        <label
          style="cursor: pointer"
          :style="{ opacity: interop === false ? '60%' : undefined }"
          @click="interop = !interop"
          >Interop</label
        >
      </template>

      <template v-if="!!apps.macros">
        <VTSwitch
          aria-label="prefer macros option"
          :class="{ 'prefer-macros': macros }"
          :aria-checked="macros"
          @click="macros = !macros"
        />
        <label
          style="cursor: pointer"
          :style="{ opacity: macros === false ? '60%' : undefined }"
          @click="macros = !macros"
          >Macros</label
        >
      </template>
    </div>
    <button class="repl-button" @click="solved = !solved">
      {{ solved ? 'Reset' : 'Solve' }}
    </button>
  </div>
</template>

<style>
.repl-button {
  background-color: var(--vp-c-brand);
  color: var(--vp-c-bg);
  padding: 4px 12px 3px;
  border-radius: 8px;
  font-weight: 600;
  font-size: 14px;
  margin-left: auto;
}

.repl-options {
  padding: 12px 18px;
  border-radius: 8px;
  background-color: var(--vp-c-gray-soft);
  display: flex;
  align-items: center;
}

.repl-options-left {
  display: flex;
  gap: 8px;
  font-size: 14px;
  font-weight: 600;
  margin-right: auto;
}

.prefer-interop .vt-switch-check,
.prefer-macros .vt-switch-check {
  transform: translateX(18px);
}
.prefer-interop.vt-switch,
.prefer-macros.vt-switch {
  background-color: var(--vp-c-brand);
}
</style>
