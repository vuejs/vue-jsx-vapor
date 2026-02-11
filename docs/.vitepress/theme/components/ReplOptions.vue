<script setup lang="ts">
import { VTSwitch } from '@vue/theme'
import { watch } from 'vue'
// @ts-ignore
import interopHtmlCode from '../../../tutorial/template/index-interop.html?raw'
// @ts-ignore
import htmlCode from '../../../tutorial/template/index.html?raw'

const props = defineProps({
  files: {
    type: Object,
    required: true,
  },
})

const solved = defineModel<boolean>('solved', { required: true })
const interop = defineModel<boolean>('interop', { required: true })

watch(
  () => [interop.value, solved.value],
  ([interop]) => {
    props.files['vite.config.ts'] = props.files['vite.config.ts'].replace(
      /(?<=interop: )(true|false)/,
      interop.toString(),
    )

    props.files['ts-macro.config.ts'] = props.files['vite.config.ts'].replace(
      /(?<=interop: )(true|false)/,
      interop.toString(),
    )

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
      <VTSwitch
        aria-label="prefer interop api"
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
  gap: 6px;
  font-size: 14px;
  font-weight: 600;
  margin-right: auto;
}

.prefer-interop .vt-switch-check {
  transform: translateX(18px);
}
.prefer-interop.vt-switch {
  background-color: var(--vp-c-brand);
}
</style>
