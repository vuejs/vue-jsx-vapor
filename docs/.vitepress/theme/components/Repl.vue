<script setup lang="ts">
import { Repl, serialize } from 'jsx-repl'
import { ref, watch } from 'vue'

const props = defineProps({
  files: {
    type: Object,
    required: true,
  },
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
  <Repl v-model="src" auto-save layout="vertical" slim preview-theme />
</template>

<style>
.jsx-repl {
  margin-left: auto;
  border: 1px solid var(--border);
  width: 100%;
  height: 100%;
}
</style>
