<script setup lang="ts">
import {
  computed,
  defineAsyncComponent,
  onBeforeUnmount,
  onMounted,
  reactive,
  ref,
  watch,
} from 'vue'
import { getDefaultFiles } from '../../../tutorial/template'

const props = defineProps<{
  app: string
  height?: string
  /** click the "js" output tab once the REPL is ready */
  autoSelectOutput?: boolean
  /** compile and run the app in Vapor mode */
  vapor?: boolean
}>()

const defaultFiles = getDefaultFiles()

function createHtmlCode(vapor: boolean) {
  const create = vapor ? 'createVaporApp' : 'createApp'
  return `<html>
  <body>
    <script type="module">
      import { ${create} } from 'vue'
      import App from './App.tsx'

      ${create}(App).mount('#app')
    ${'</'}script>

    <div id="app"></div>
  </body>

  <style>
    * { box-sizing: border-box; }
    body {
      margin: 0;
      color: #18222f;
      background: #f4f8f6;
      font-family:
        Inter,
        ui-sans-serif,
        system-ui,
        -apple-system,
        BlinkMacSystemFont,
        "Segoe UI",
        sans-serif;
    }
    button {
      border: 0;
      border-radius: 6px;
      padding: 8px 14px;
      color: #fff;
      background: #1f2a37;
      font: inherit;
      font-weight: 700;
      cursor: pointer;
    }
    .demo {
      min-height: 100vh;
      display: grid;
      align-content: start;
      gap: 12px;
      padding: 24px;
    }
    .count,
    .output {
      width: min(100%, 420px);
      border: 1px solid #d9e3dd;
      border-radius: 8px;
      padding: 12px 14px;
      background: #fff;
      font-size: 15px;
      line-height: 1.5;
    }
    .count {
      border-color: #a7d7c4;
      background: #eaf8f2;
      font-weight: 700;
    }
    html.dark body {
      color: #e2e8f0;
      background: #1e1e1e;
    }
    html.dark button {
      color: #10151c;
      background: #a7d7c4;
    }
    html.dark .count,
    html.dark .output {
      border-color: #2c3a47;
      background: #1a222c;
    }
    html.dark .count {
      border-color: #2f6b55;
      background: #142b23;
    }
  </style>
</html>
`
}

const vapor = props.vapor ?? false

const files = reactive({
  ...defaultFiles,
  'ts-macro.config.ts': undefined,
  'src/index.html': createHtmlCode(vapor),
  'src/App.tsx': props.app,
  'vite.config.ts': defaultFiles['vite.config.ts'].replace(
    /(?<=vapor: )(true|false)/,
    vapor.toString(),
  ),
})

watch(
  () => props.app,
  (app) => {
    files['src/App.tsx'] = app
  },
)

const replStyle = computed(() => ({
  height: props.height ?? '720px',
}))

const root = ref<HTMLElement>()
let pollTimer: ReturnType<typeof setInterval> | undefined

onMounted(() => {
  if (!props.autoSelectOutput) return
  pollTimer = setInterval(() => {
    const jsTab = Array.from(root.value?.querySelectorAll<HTMLButtonElement>('button') ?? []).find(
      (btn) => btn.textContent?.trim() === 'js',
    )
    if (jsTab) {
      jsTab.click()
      clearInterval(pollTimer)
      pollTimer = undefined
    }
  }, 300)
})

onBeforeUnmount(() => clearInterval(pollTimer))

const Repl = defineAsyncComponent({
  loader: () => import('./Repl.vue'),
})
</script>

<template>
  <div ref="root" class="blog-repl" :style="replStyle">
    <ClientOnly>
      <Repl
        :files
        :auto-save="false"
        layout="vertical"
        :editor-options="{
          monacoOptions: {
            minimap: { enabled: false },
            fontSize: 13,
          },
        }"
      />
      <template #fallback>
        <div class="blog-repl-loading">Loading editor...</div>
      </template>
    </ClientOnly>
  </div>
</template>

<style scoped>
.blog-repl {
  width: 100%;
  margin: 18px 0 24px;
  overflow: hidden;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--vp-c-bg-soft);
}

.blog-repl-loading {
  display: grid;
  height: 100%;
  place-items: center;
  color: var(--vp-c-text-2);
  font-size: 14px;
}
</style>
