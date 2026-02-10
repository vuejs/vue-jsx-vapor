<script setup lang="ts">
import { defineAsyncComponent, ref } from 'vue'

const props = defineProps({
  src: {
    type: Object,
    required: true,
  },
  solvedSrc: {
    type: Object,
    required: true,
  },
  next: String,
})

const src = ref(props.src)
const solved = ref(false)
function onSolved() {
  solved.value = !solved.value
  src.value = solved.value ? props.solvedSrc : props.src
}

const Repl = defineAsyncComponent({
  loader: () => import('./Repl.vue'),
})
</script>

<template>
  <div class="repl-container">
    <div class="repl-left">
      <div class="repl-content">
        <slot foo="foo" />
      </div>
      <div class="repl-bottom">
        <button class="repl-button" @click="onSolved">
          {{ solved ? 'Reset' : 'Solve' }}
        </button>
        <a v-if="next" :href="next" style="text-decoration: unset">Next →</a>
      </div>
    </div>
    <ClientOnly>
      <Repl :src />
    </ClientOnly>
  </div>
</template>

<style>
.VPContent.has-sidebar {
  padding-right: 0 !important;
}
.VPDoc .content {
  padding: 0 !important;
  margin-top: -20px;
}

.repl-left {
  overflow: auto;
  display: flex;
  flex-direction: column;
}

.repl-content {
  flex: 1;
  overflow: auto;
}

.repl-button {
  background-color: var(--vp-c-brand);
  color: var(--vp-c-bg);
  padding: 4px 12px 3px;
  border-radius: 8px;
  font-weight: 600;
  font-size: 14px;
  margin-right: auto;
}

.repl-bottom {
  display: flex;
  align-items: center;
  margin-top: auto;
  margin-bottom: 34px;
  padding-top: 10px;
  border-top: 1px solid var(--vp-c-gray-1);
}

.repl-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  height: calc(100vh - 132px - 48px);
}
@media (min-width: 1280px) {
  .repl-container {
    height: calc(100vh - 132px);
  }
}
</style>
