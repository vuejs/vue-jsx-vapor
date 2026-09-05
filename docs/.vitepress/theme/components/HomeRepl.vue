<script setup lang="ts">
import {
  defineAsyncComponent,
  onBeforeUnmount,
  onMounted,
  reactive,
  ref,
  watch,
} from 'vue'
import { getDefaultFiles } from '../../../tutorial/template'

const vdomAppCode = `import { defineComponent, ref } from 'vue'

export default defineComponent(() => {
  const count = ref(0)

  return () => (
    <main class="demo">
      <span>Vue JSX</span>
      <h1>Compiler-powered JSX</h1>
      <button onClick={() => count.value++}>
        Count is {count.value}
      </button>
    </main>
  )
})
`

const vaporAppCode = `import { ref } from 'vue'

export default () => {
  const count = ref(0)

  return (
    <main class="demo">
      <span>Vue JSX Vapor</span>
      <h1>Fine-grained rendering</h1>
      <button onClick={() => count.value++}>
        Count is {count.value}
      </button>
    </main>
  )
}
`

const styleCode = `* { box-sizing: border-box; }
    body { margin: 0; font-family: Inter, system-ui, sans-serif; }
    .demo {
      min-height: 100vh;
      display: grid;
      place-content: center;
      justify-items: start;
      padding: 32px;
      color: #17212b;
      background: #f7faf9;
    }
    .demo span { color: #0f8a5f; font-weight: 700; }
    .demo h1 { margin: 8px 0 24px; font-size: 28px; }
    .demo button {
      border: 0;
      border-radius: 6px;
      padding: 10px 16px;
      color: white;
      background: #17212b;
      font: inherit;
      font-weight: 600;
      cursor: pointer;
    }
    html.dark .demo {
      color: #e2e8f0;
      background: #1e1e1e;
    }
    html.dark .demo span { color: #42b883; }
    html.dark .demo button {
      color: #10151c;
      background: #a7d7c4;
    }`

function createHtmlCode(vapor: boolean) {
  return `<html>
  <body>
    <script type="module">
      import { ${vapor ? 'createVaporApp' : 'createApp'} } from 'vue'
      import App from './App.tsx'
      ${vapor ? 'createVaporApp' : 'createApp'}(App).mount('#app')
    ${'</'}script>

    <div id="app"></div>
  </body>

  <style>
    ${styleCode}
  </style>
</html>
`
}

const defaultFiles = getDefaultFiles()
const files = reactive({
  ...defaultFiles,
  'ts-macro.config.ts': undefined,
  'src/index.html': createHtmlCode(false),
  'src/App.tsx': vdomAppCode,
})
const vapor = ref(false)

watch(vapor, (enabled) => {
  localStorage.setItem('vapor', enabled.toString())
  files['src/App.tsx'] = enabled ? vaporAppCode : vdomAppCode
  files['src/index.html'] = createHtmlCode(enabled)
  files['vite.config.ts'] = defaultFiles['vite.config.ts'].replace(
    /(?<=vapor: )(true|false)/,
    enabled.toString(),
  )
})

const layout = ref<'horizontal' | 'vertical'>('horizontal')
let mediaQuery: MediaQueryList | undefined

function updateLayout() {
  layout.value = mediaQuery?.matches ? 'vertical' : 'horizontal'
}

onMounted(() => {
  vapor.value = localStorage.getItem('vapor') === 'true'
  mediaQuery = globalThis.matchMedia('(max-width: 720px)')
  updateLayout()
  mediaQuery.addEventListener('change', updateLayout)
})

onBeforeUnmount(() => {
  mediaQuery?.removeEventListener('change', updateLayout)
})

const Repl = defineAsyncComponent({
  loader: () => import('./Repl.vue'),
})
</script>

<template>
  <div class="home-repl">
    <ClientOnly>
      <Repl
        :files
        :layout
        :editor-options="{
          monacoOptions: {
            lineNumbers: false,
            folding: false,
          },
        }"
      >
        <template #previewActions>
          <button
            type="button"
            role="switch"
            class="home-repl-mode"
            :class="{ active: vapor }"
            :aria-checked="vapor"
            @click="vapor = !vapor"
          >
            <i class="home-repl-switch" aria-hidden="true"><i /></i>
            Vapor
          </button>
        </template>
      </Repl>
      <template #fallback>
        <div class="home-repl-loading">Loading editor...</div>
      </template>
    </ClientOnly>
  </div>
</template>

<style scoped>
.home-repl {
  width: 100%;
  height: 420px;
  overflow: hidden;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--vp-c-bg-soft);
  box-shadow: var(--vp-shadow-4);
}

.home-repl-mode {
  display: flex;
  align-items: center;
  gap: 5px;
  min-height: 30px;
  padding: 4px 0;
  background-color: transparent !important;
  color: var(--vp-c-text-2);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.home-repl-mode.active {
  color: #42b883;
}

.home-repl-switch {
  display: flex;
  width: 32px;
  height: 18px;
  align-items: center;
  padding: 2px;
  border-radius: 9px;
  background: var(--vp-c-divider);
  transition: background-color 0.2s;
}

.home-repl-switch i {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: #fff;
  box-shadow: var(--vp-shadow-1);
  transition: transform 0.2s;
}

.home-repl-mode.active .home-repl-switch {
  background: #42b883;
}

.home-repl-mode.active .home-repl-switch i {
  transform: translateX(14px);
}

.home-repl-loading {
  display: grid;
  width: 100%;
  height: 100%;
  place-items: center;
  color: var(--vp-c-text-2);
  font-size: 14px;
}

:global(.VPHero.has-image .image-bg) {
  display: none;
}

:global(.VPHero.has-image .image-container) {
  width: 100%;
  height: auto;
}

@media (max-width: 1099px) {
  :global(.VPHero.has-image .container) {
    flex-direction: column;
    text-align: center;
  }

  :global(.VPHero.has-image .main) {
    order: 1;
    width: 100%;
    max-width: 640px;
  }

  :global(.VPHero.has-image .name),
  :global(.VPHero.has-image .tagline) {
    margin-right: auto;
    margin-left: auto;
  }

  :global(.VPHero.has-image .actions) {
    justify-content: center;
  }

  :global(.VPHero.has-image .image) {
    order: 2;
    width: 100%;
    margin: 40px 0 0;
  }
}

@media (max-width: 639px) {
  .home-repl {
    height: 640px;
  }
}

@media (min-width: 1100px) {
  :global(.VPHero.has-image .container) {
    max-width: 1344px;
  }

  :global(.VPHero.has-image .main) {
    max-width: 540px !important;
  }

  :global(.VPHero.has-image .image) {
    min-width: 0;
  }
}

:global(.VPHero.has-image .image-container) {
  transform: none;
}
</style>
