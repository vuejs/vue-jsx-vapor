import { defineConfig } from 'vitepress'
import {
  createTwoslasher,
  transformerTwoslash,
} from '../../packages/macros/twoslash'
import vueJsx from '../../packages/vue-jsx/src/volar'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: 'Vue JSX',
  description: 'High-performance Vue JSX compiler powered by Oxc',
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
          { text: '博客', link: '/zh/blog/', activeMatch: '/zh/blog/' },
          {
            text: '教程',
            link: '/zh/tutorial/step-1',
            activeMatch: 'tutorial',
          },
          {
            text: 'Playground',
            link: 'https://repl.vuejsx.dev/',
          },
        ],
        sidebar: {
          '/zh/blog/': [
            {
              text: 'Vue JSX 3.3',
              items: [
                {
                  text: 'Virtual DOM 篇',
                  link: '/zh/blog/vdom',
                },
                {
                  text: 'Vapor 模式',
                  link: '/zh/blog/vapor',
                },
                {
                  text: '原生 TS7 支持',
                  link: '/zh/blog/typescript-7',
                },
              ],
            },
          ],
          '/zh/': [
            {
              text: '介绍',
              items: [
                {
                  text: '快速开始',
                  link: `/zh/introduction/getting-started`,
                },
                {
                  text: 'Vapor 模式',
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
                  text: '列表组件',
                  link: '/zh/features/for',
                },
                {
                  text: 'Custom Element',
                  link: '/zh/features/custom-elements',
                },
              ],
            },
          ],
          '/zh/tutorial/': [
            {
              text: '基础教程',
              items: [
                { text: '1. 快速开始', link: '/zh/tutorial/step-1/' },
                { text: '2. JSX 介绍', link: '/zh/tutorial/step-2/' },
                { text: '3. 属性绑定', link: '/zh/tutorial/step-3/' },
                { text: '4. 事件绑定', link: '/zh/tutorial/step-4/' },
                { text: '5. 条件渲染', link: '/zh/tutorial/step-5/' },
                { text: '6. 列表渲染', link: '/zh/tutorial/step-6/' },
                { text: '7. 组件', link: '/zh/tutorial/step-7/' },
                { text: '8. Props', link: '/zh/tutorial/step-8/' },
                { text: '9. 插槽', link: '/zh/tutorial/step-9/' },
                { text: '10. 作用域插槽', link: '/zh/tutorial/step-10/' },
                { text: '11. Expose', link: '/zh/tutorial/step-11/' },
              ],
            },
            {
              text: '进阶教程',
              items: [
                { text: '12. 双向绑定', link: '/zh/tutorial/step-12/' },
                { text: '13. 动态组件', link: '/zh/tutorial/step-13/' },
                { text: '14. HyperScript', link: '/zh/tutorial/step-14/' },
                { text: '15. Custom Element', link: '/zh/tutorial/step-15/' },
              ],
            },
            { text: '恭喜完成!', link: '/zh/tutorial/done/' },
          ],
        },
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
      { text: 'Blog', link: '/blog/', activeMatch: '/blog/' },
      { text: 'Tutorial', link: '/tutorial/step-1', activeMatch: 'tutorial' },
      { text: 'Playground', link: 'https://repl.vuejsx.dev/' },
    ],

    sidebar: {
      '/blog/': [
        {
          text: 'Vue JSX 3.3',
          items: [
            {
              text: 'Virtual DOM',
              link: '/blog/vdom',
            },
            {
              text: 'Vapor Mode',
              link: '/blog/vapor',
            },
            {
              text: 'Native TypeScript 7',
              link: '/blog/typescript-7',
            },
          ],
        },
      ],
      '/': [
        {
          text: 'Introduction',
          items: [
            {
              text: 'Getting Started',
              link: '/introduction/getting-started',
            },
            {
              text: 'Vapor Mode',
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
              text: 'List Components',
              link: '/features/for',
            },
            {
              text: 'Custom Elements',
              link: '/features/custom-elements',
            },
          ],
        },
      ],
      '/tutorial/': [
        {
          text: 'Basic Tutorial',
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
          ],
        },
        {
          text: 'Advanced Tutorial',
          items: [
            { text: '12. Two-way Binding', link: '/tutorial/step-12/' },
            { text: '13. Dynamic Component', link: '/tutorial/step-13/' },
            { text: '14. HyperScript', link: '/tutorial/step-14/' },
            { text: '15. Custom Elements', link: '/tutorial/step-15/' },
          ],
        },
        { text: 'You Did it!', link: '/tutorial/step-done/' },
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
            jsxImportSource: 'vue-jsx',
            baseUrl: undefined,
          },
          tsmCompilerOptions: {
            plugins: [vueJsx({ macros: true })],
          },
        }),
      }) as any,
    ],
  },
})
