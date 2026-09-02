<script setup lang="ts">
import { VTSwitch } from '@vue/theme'
import { watch, type PropType } from 'vue'
// @ts-ignore
import vaporHtmlCode from '../../../tutorial/template/index-vapor.html?raw'
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
      solved?: object
      vapor?: object
      vaporSolved?: object
      macros?: object
      macrosSolved?: object
      vaporMacros?: object
      vaporMacrosSolved?: object
    }>,
    required: true,
  },
})

const solved = defineModel<boolean>('solved', { required: true })
const vapor = defineModel<boolean>('vapor', { required: true })
const macros = defineModel<boolean>('macros', { required: true })

watch(
  () => [vapor.value, macros.value, solved.value],
  ([vapor, macros]) => {
    props.files['vite.config.ts'] = props.files['vite.config.ts'].replace(
      /(?<=vapor: )(true|false)/,
      vapor.toString(),
    )
    props.files['vite.config.ts'] = props.files['vite.config.ts'].replace(
      /(?<=macros: )(true|false)/,
      macros.toString(),
    )

    props.files['ts-macro.config.ts'] = props.files[
      'ts-macro.config.ts'
    ].replace(/(?<=vapor: )(true|false)/, vapor.toString())
    props.files['ts-macro.config.ts'] = props.files[
      'ts-macro.config.ts'
    ].replace(/(?<=macros: )(true|false)/, macros.toString())

    props.files['src/index.html'] = vapor ? vaporHtmlCode : htmlCode
  },
  { immediate: true, deep: true },
)
</script>

<template>
  <div class="repl-options">
    <div class="repl-options-left">
      <template v-if="!!apps.vapor">
        <VTSwitch
          aria-label="Enable Vapor mode"
          :class="{ 'prefer-vapor': vapor }"
          :aria-checked="vapor"
          @click="vapor = !vapor"
        />
        <label
          style="cursor: pointer"
          :style="{ opacity: vapor === false ? '60%' : undefined }"
          @click="vapor = !vapor"
          >Vapor</label
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
    <div class="repl-options-right">
      <button
        class="repl-button"
        :style="{
          'background-color': solved
            ? 'var(--vp-c-green)'
            : 'var(--vp-c-brand)',
        }"
        @click="solved = !solved"
      >
        {{ solved ? 'Reset' : 'Solve' }}
      </button>
    </div>
  </div>
</template>

<style>
.repl-button {
  color: var(--vp-c-bg);
  padding: 4px 12px 3px;
  border-radius: 8px;
  font-weight: 600;
  font-size: 14px;
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

.repl-options-right {
  align-items: center;
  display: flex;
  gap: 10px;
  font-weight: 600;
}

.prefer-vapor .vt-switch-check,
.prefer-macros .vt-switch-check {
  transform: translateX(18px);
}
.prefer-vapor.vt-switch,
.prefer-macros.vt-switch {
  background-color: var(--vp-c-brand);
}
</style>
