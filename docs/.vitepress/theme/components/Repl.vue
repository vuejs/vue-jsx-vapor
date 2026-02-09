<script setup lang="ts">
import { Repl, serialize } from 'jsx-repl'
import { ref, watch } from 'vue'

const props = defineProps({
  src: {
    type: Object,
    required: true,
  },
})

const src = ref(serialize(props.src))
watch(
  () => props.src,
  () => {
    src.value = serialize(props.src)
  },
)
</script>

<template>
  <Repl v-model="src" auto-save layout="vertical" slim preview-theme />
</template>

<style>
.jsx-repl {
  margin-left: auto;
  margin-top: -34px;
  border: 1px solid var(--border);
  width: 100%;
  height: 100%;
}
</style>
