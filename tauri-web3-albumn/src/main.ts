// register vue composition api globally
import { ViteSSG } from 'vite-ssg'
import devtools from '@vue/devtools'

import generatedRoutes from 'virtual:generated-pages'
import { setupLayouts } from 'virtual:generated-layouts'
import App from './App.vue'

// windicss layers
import 'virtual:windi-base.css'
import 'virtual:windi-components.css'
// your custom styles here
import './styles/main.css'
// windicss utilities should be the last style import
import 'virtual:windi-utilities.css'
// windicss devtools support (dev only)
import 'virtual:windi-devtools'

if (process.env.IS_TAURI === 'yes')
  devtools.connect('http://localhost', 8098)

const routes = setupLayouts(generatedRoutes)

const projectFolder = import.meta.env.VITE_PROJECT_FOLDER
const gitHash = import.meta.env.VITE_VERCEL_GIT_COMMIT_SHA
const gitBranch = import.meta.env.VITE_VERCEL_GIT_COMMIT_REF
// eslint-disable-next-line no-console
console.log({
  projectFolder,
  gitHash,
  gitBranch,
})

useLogRocket().doInit(import.meta.env.VITE_LOG_ROCKET_ID)

// https://github.com/antfu/vite-ssg
export const createApp = ViteSSG(
  App,
  { routes, base: import.meta.env.BASE_URL },
  // (ctx = { app, router, routes, isClient, initialState }) => {}
  (ctx) => {
    // install all modules under `modules/`
    Object.values(import.meta.globEager('./modules/*.ts')).forEach(i => i.install?.(ctx))
  },
)
