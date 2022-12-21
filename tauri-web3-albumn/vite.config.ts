import path from 'path'
import dotenv from 'dotenv'
import { defineConfig } from 'vite'
import Vue from '@vitejs/plugin-vue'
import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'
import Icons from 'unplugin-icons/vite'
import IconsResolver from 'unplugin-icons/resolver'
import Components from 'unplugin-vue-components/vite'
import AutoImport from 'unplugin-auto-import/vite'
import Markdown from 'vite-plugin-md'
import WindiCSS from 'vite-plugin-windicss'
import { VitePWA } from 'vite-plugin-pwa'
import VueI18n from '@intlify/vite-plugin-vue-i18n'
import Inspect from 'vite-plugin-inspect'
import Prism from 'markdown-it-prism'
import LinkAttributes from 'markdown-it-link-attributes'
import { HeadlessUiResolver } from 'unplugin-vue-components/resolvers'
import nodePolyfills from 'rollup-plugin-polyfill-node'
// import { NodeGlobalsPolyfillPlugin } from '@esbuild-plugins/node-globals-polyfill'
// import { NodeModulesPolyfillPlugin } from '@esbuild-plugins/node-modules-polyfill'
// import rollupNodePolyFill from 'rollup-plugin-node-polyfills'
// import builtins from 'rollup-plugin-node-builtins'

const production = process.env.NODE_ENV === 'production'

const markdownWrapperClasses = 'prose prose-sm m-auto text-left'

dotenv.config({ path: '.env' })
const projectFolder = process.env.VITE_PROJECT_FOLDER || 'default'
const gitHash = process.env.VITE_VERCEL_GIT_COMMIT_SHA
const gitBranch = process.env.VITE_VERCEL_GIT_COMMIT_REF
console.log('====> build env :', {
  projectFolder,
  gitHash,
  gitBranch,
})

dotenv.config({ path: 'env/global.env' })
dotenv.config({ path: `env/${projectFolder}.env` })

export default defineConfig({
  publicDir: `public/${projectFolder}`,
  resolve: {
    alias: {
      '~/': `${path.resolve(__dirname, 'src')}/`,
      'process': 'process/browser',
      'stream': 'stream-browserify',
      'zlib': 'browserify-zlib',
      'util': 'util',
    },
  },
  plugins: [
    // ↓ Needed for development mode
    !production && nodePolyfills({
      include: ['node_modules/**/*.js', new RegExp('node_modules/.vite/.*js')],
    }),

    Vue({
      include: [/\.vue$/, /\.md$/],
      reactivityTransform: true,
    }),

    // https://github.com/hannoeru/vite-plugin-pages
    Pages({
      dirs: `src/pages/${projectFolder}`,
      exclude: ['src/pages/**/components/*.vue'],
      extensions: ['vue', 'md'],
    }),

    // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
    Layouts(),

    // https://github.com/antfu/unplugin-auto-import
    AutoImport({
      imports: [
        'vue',
        'vue/macros',
        'vue-router',
        'vue-i18n',
        '@vueuse/head',
        '@vueuse/core',
        {
          'lodash': [
            'isEqual',
            'uniq',
            'random',
            'remove',
            'forEach',
            'get',
            'reverse',
            'filter',
            'take',
            'shuffle',
          ],
          'pinia': [
            'acceptHMRUpdate',
            'defineStore',
            'storeToRefs',
          ],
          'villus': [
            'useQuery',
          ],
          'graphql-tag': [
            'gql',
          ],
          '@headlessui/vue': [
            'TransitionRoot',
            'TransitionChild',
            'Dialog',
            'DialogPanel',
            'DialogOverlay',
            'DialogTitle',
            'DialogDescription',
            'Menu',
            'MenuButton',
            'MenuItem',
            'MenuItems',
            'TabGroup',
            'TabList',
            'Tab',
            'TabPanels',
            'TabPanel',
            'Combobox',
            'ComboboxInput',
            'ComboboxButton',
            'ComboboxOptions',
            'ComboboxOption',
          ],
        },
      ],
      dirs: [
        'src/composables',
        'src/stores',
        'src/config',
        'src/gql',
      ],
      dts: 'src/auto-imports.d.ts',
      vueTemplate: true,
    }),

    // https://github.com/antfu/unplugin-vue-components
    Components({
      // allow auto load markdown components under `./src/components/`
      extensions: ['vue', 'md'],

      // allow auto import and register components used in markdown
      include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
      deep: true,
      directoryAsNamespace: true,
      // custom resolvers
      resolvers: [
        // auto import icons
        // https://github.com/antfu/unplugin-icons
        IconsResolver({
          prefix: false,
          // enabledCollections: ['carbon']
        }),
        HeadlessUiResolver({ prefix: '' }),
      ],

      dts: 'src/components.d.ts',
    }),

    // https://github.com/antfu/unplugin-icons
    Icons({
      autoInstall: true,
    }),

    // https://github.com/antfu/vite-plugin-windicss
    WindiCSS({
      safelist: markdownWrapperClasses,
    }),

    // https://github.com/antfu/vite-plugin-md
    // Don't need this? Try vitesse-lite: https://github.com/antfu/vitesse-lite
    Markdown({
      wrapperClasses: markdownWrapperClasses,
      headEnabled: true,
      markdownItSetup(md) {
        // https://prismjs.com/
        md.use(Prism)
        md.use(LinkAttributes, {
          matcher: (link: string) => /^https?:\/\//.test(link),
          attrs: {
            target: '_blank',
            rel: 'noopener',
          },
        })
      },
    }),

    // https://github.com/antfu/vite-plugin-pwa
    VitePWA({
      registerType: 'autoUpdate',
      includeAssets: ['favicon.svg', 'robots.txt', 'safari-pinned-tab.svg'],
      manifest: {
        name: 'Vitesse',
        short_name: 'Vitesse',
        theme_color: '#ffffff',
        icons: [
          {
            src: '/pwa-192x192.png',
            sizes: '192x192',
            type: 'image/png',
          },
          {
            src: '/pwa-512x512.png',
            sizes: '512x512',
            type: 'image/png',
          },
          {
            src: '/pwa-512x512.png',
            sizes: '512x512',
            type: 'image/png',
            purpose: 'any maskable',
          },
        ],
      },
    }),

    // https://github.com/intlify/bundle-tools/tree/main/packages/vite-plugin-vue-i18n
    VueI18n({
      runtimeOnly: false,
      compositionOnly: true,
      include: [
        path.resolve(__dirname, 'locales/**'),
        // path.resolve(__dirname, 'locales/cryptotoys.club/**'),
      ],
    }),

    // https://github.com/antfu/vite-plugin-inspect
    Inspect({
      // change this to enable inspect for debugging
      enabled: false,
    }),
  ],

  server: {
    fs: {
      strict: true,
    },
  },

  // https://github.com/antfu/vite-ssg
  ssgOptions: {
    script: 'async',
    formatting: 'minify',
  },

  optimizeDeps: {
    include: [
      'vue',
      'vue-router',
      '@vueuse/core',
      '@vueuse/head',
    ],
    exclude: [
      'vue-demi',
    ],
    esbuildOptions: {
      // plugins: [
      //   NodeGlobalsPolyfillPlugin({
      //     process: true,
      //     buffer: true,
      //   }),
      //   NodeModulesPolyfillPlugin(),
      // ],
    },
  },

  // https://github.com/vitest-dev/vitest
  test: {
    include: ['test/**/*.test.ts'],
    environment: 'jsdom',
    deps: {
      inline: ['@vue', '@vueuse', 'vue-demi'],
    },
  },
  build: {
    target: 'es2020',
    // plugins: [
    //   builtins(),
    //   rollupNodePolyFill(),
    // ],
  },
  // build: {
  //   rollupOptions: {
  //     plugins: [
  //       // ↓ Needed for build
  //       nodePolyfills(),
  //     ],
  //   },
  //   // ↓ Needed for build if using WalletConnect and other providers
  //   commonjsOptions: {
  //     transformMixedEsModules: true,
  //   },
  // },
})
