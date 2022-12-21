import VMdEditor from '@kangc/v-md-editor';
import '@kangc/v-md-editor/lib/style/base-editor.css';

import VMdPreview from '@kangc/v-md-editor/lib/preview'
import '@kangc/v-md-editor/lib/style/preview.css'

import githubTheme from '@kangc/v-md-editor/lib/theme/github.js'
import '@kangc/v-md-editor/lib/theme/style/github.css'
import hljs from 'highlight.js'

import type { UserModule } from '~/types'

// { app, router, routes, isClient, initialState }
export const install: UserModule = async({ isClient, app }) => {
  if (!isClient)
    return

  VMdEditor.use(githubTheme, {
    Hljs: hljs,
  });
  app.use(VMdEditor)
  
  VMdPreview.use(githubTheme, {
    Hljs: hljs,
  })
  app.use(VMdPreview)
}
