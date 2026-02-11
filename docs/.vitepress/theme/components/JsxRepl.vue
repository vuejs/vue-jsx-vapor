<script setup lang="ts">
import { defineAsyncComponent, ref, watch, type PropType } from 'vue'
import ReplOptions from './ReplOptions.vue'
import { useRouteQuery } from './utils'

const props = defineProps({
  files: {
    type: Object,
    required: true,
  },
  apps: {
    type: Object as PropType<{
      app: object
      solved: object
      interop: object
      interopSolved: object
    }>,
    required: true,
  },
  prev: String,
  next: String,
})

const solved = ref(false)
const interop = useRouteQuery<boolean>('interop', false)
watch(
  () => [interop.value, solved.value],
  () => {
    setApp()
  },
  { immediate: true },
)
function setApp() {
  Object.assign(
    props.files,
    props.apps[
      interop.value
        ? solved.value
          ? 'interopSolved'
          : 'interop'
        : solved.value
          ? 'solved'
          : 'app'
    ],
  )
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
        <a v-show="prev" :href="prev">← Prev</a>
        <a v-show="next" :href="next" style="margin-left: auto">Next →</a>
      </div>
    </div>
    <div class="repl-right">
      <ReplOptions v-model:interop="interop" v-model:solved="solved" :files />
      <ClientOnly>
        <Repl :files />
      </ClientOnly>
    </div>
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

.repl-right {
  margin-top: -34px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  height: 100%;
  z-index: 1;
}

.repl-content {
  flex: 1;
  overflow: auto;
}

.repl-bottom {
  display: flex;
  align-items: center;
  margin-top: auto;
  margin-bottom: 34px;
  padding-top: 10px;
  border-top: 1px solid var(--vp-c-gray-1);
  a {
    text-decoration: unset;
  }
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
