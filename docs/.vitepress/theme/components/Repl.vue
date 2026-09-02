<script setup lang="ts">
import { Repl, serialize } from 'jsx-repl'
import { ref, watch, type PropType } from 'vue'

const props = defineProps({
  files: {
    type: Object,
    required: true,
  },
  layout: {
    type: String as PropType<'horizontal' | 'vertical'>,
    default: 'horizontal',
  },
  editorOptions: { type: Object },
})

const src = ref(serialize(props.files))
watch(
  () => props.files,
  () => {
    src.value = serialize(props.files)
  },
  { deep: true },
)
</script>

<template>
  <Repl
    v-model="src"
    auto-save
    slim
    preview-theme
    :layout
    :editor-options="props.editorOptions"
  >
    <template #previewActions>
      <slot name="previewActions" />
    </template>
  </Repl>
</template>

<style>
.jsx-repl {
  margin-left: auto;
  border: 1px solid var(--border);
  width: 100%;
  height: 100%;
}
</style>
