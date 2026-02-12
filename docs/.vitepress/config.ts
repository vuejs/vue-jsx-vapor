import { transformerTwoslash } from '@shikijs/vitepress-twoslash'
import { createTwoslasher } from '@ts-macro/twoslash'
import { defineConfig } from 'vitepress'
import vueJsxVapor from '../../packages/vue-jsx-vapor/src/volar'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: 'Vue JSX Vapor',
  description: 'Vue JSX Vapor',
  head: [['link', { rel: 'icon', type: 'image/svg+xml', href: '/logo.svg' }]],
  locales: {
    root: {
      label: 'English',
      lang: 'en',
    },
    zh: {
      label: '简体中文',
      lang: 'zh-CN',
      link: '/zh/',
      themeConfig: {
        nav: [
          { text: '首页', link: '/zh/' },
          { text: '特性', link: '/zh/features/directives' },
          {
            text: 'Playground',
            link: 'https://repl.zmjs.dev/vuejs/vue-jsx-vapor',
          },
        ],
        sidebar: [
          {
            text: '介绍',
            items: [
              {
                text: '快速开始',
                link: `/zh/introduction/getting-started`,
              },
              {
                text: '互操作性',
                link: `/zh/introduction/interop`,
              },
              {
                text: '迁移',
                link: `/zh/introduction/migration`,
              },
              {
                text: 'ESLint',
                link: `/zh/introduction/eslint`,
              },
            ],
          },
          {
            text: '特性',
            items: [
              {
                text: '指令',
                link: '/zh/features/directives',
              },
              {
                text: '宏',
                link: '/zh/features/macros',
              },
              {
                text: 'useRef',
                link: '/zh/features/use-ref',
              },
            ],
          },
        ],
      },
    },
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    logo: '/logo.svg',
    nav: [
      { text: 'Home', link: '/' },
      {
        text: 'Features',
        link: '/features/directives',
        activeMatch: 'features',
      },
      { text: 'Tutorial', link: '/tutorial/step-1', activeMatch: 'tutorial' },
      { text: 'Playground', link: 'https://repl.zmjs.dev/vuejs/vue-jsx-vapor' },
    ],

    sidebar: {
      '/': [
        {
          text: 'Introduction',
          items: [
            {
              text: 'Getting Started',
              link: '/introduction/getting-started',
            },
            {
              text: 'Interop',
              link: '/introduction/interop',
            },
            {
              text: 'Migration',
              link: '/introduction/migration',
            },
            {
              text: 'ESLint',
              link: '/introduction/eslint',
            },
          ],
        },
        {
          text: 'Features',
          items: [
            {
              text: 'Directives',
              link: '/features/directives',
            },
            {
              text: 'Macros',
              link: '/features/macros',
            },
            {
              text: 'useRef',
              link: '/features/use-ref',
            },
          ],
        },
      ],
      '/tutorial/': [
        {
          text: 'Tutorial',
          items: [
            { text: '1. Getting Started', link: '/tutorial/step-1/' },
            { text: '2. Introduction JSX', link: '/tutorial/step-2/' },
            { text: '3. Attribute Bindings', link: '/tutorial/step-3/' },
            { text: '4. Event Bindings', link: '/tutorial/step-4/' },
            { text: '5. Conditional Rendering', link: '/tutorial/step-5/' },
            { text: '6. List Rendering', link: '/tutorial/step-6/' },
            { text: '7. Components', link: '/tutorial/step-7/' },
            { text: '8. Props', link: '/tutorial/step-8/' },
            { text: '9. Slots', link: '/tutorial/step-9/' },
            { text: '10. Scoped Slots', link: '/tutorial/step-10/' },
            { text: '11. Expose', link: '/tutorial/step-11/' },
            { text: '12. You Did it!', link: '/tutorial/step-12/' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'discord', link: 'https://discord.gg/hMnyhpJH' },
      { icon: 'github', link: 'https://github.com/vuejs/vue-jsx-vapor' },
    ],
  },
  markdown: {
    languages: ['js', 'ts', 'tsx'],
    codeTransformers: [
      transformerTwoslash({
        twoslasher: createTwoslasher({
          compilerOptions: {
            jsx: 1,
            jsxImportSource: 'vue-jsx-vapor',
            customConditions: ['jsx-vapor-dev'],
          },
          tsmCompilerOptions: {
            plugins: [vueJsxVapor({ macros: true })],
          },
        }),
      }) as any,
    ],
  },
})
