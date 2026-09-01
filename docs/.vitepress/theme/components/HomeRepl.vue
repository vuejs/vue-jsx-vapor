<script setup lang="ts">
import { defineAsyncComponent, onBeforeUnmount, onMounted, ref } from 'vue'
import { getDefaultFiles } from '../../../tutorial/template'

const appCode = `import { defineComponent, ref } from 'vue'

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
    }`

const htmlCode = `<html>
  <body>
    <script type="module">
      import { createApp } from 'vue'
      import App from './App.tsx'
      createApp(App).mount('#app')
    <\/script>

    <div id="app"></div>
  </body>

  <style>
    ${styleCode}
  </style>
</html>
`

const files = {
  ...getDefaultFiles(),
  'src/index.html': htmlCode,
  'ts-macro.config.ts': undefined,
  'src/App.tsx': appCode,
}

const layout = ref<'horizontal' | 'vertical'>('horizontal')
let mediaQuery: MediaQueryList | undefined

function updateLayout() {
  layout.value = mediaQuery?.matches ? 'vertical' : 'horizontal'
}

onMounted(() => {
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
        :editor-options="{ monacoOptions: { lineNumbers: false } }"
      />
      <template #fallback>
        <div class="home-repl-loading">Loading editor...</div>
      </template>
    </ClientOnly>
  </div>
</template>

<style scoped>
.home-repl {
  width: 100%;
  height: 412px;
  overflow: hidden;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--vp-c-bg-soft);
  box-shadow: var(--vp-shadow-4);
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

  :global(.VPHero.has-image .image) {
    min-width: 0;
  }
}

:global(.VPHero.has-image .image-container) {
  transform: none;
}
</style>
