import LitJsSdk from '@lit-protocol/sdk-browser'
import type { UserModule } from '~/types'

// { app, router, routes, isClient, initialState }
export const install: UserModule = async({ isClient, app }) => {
  if (!isClient)
    return

  document.addEventListener('lit-ready', (e) => {
    console.log('===>>> LIT network is ready')
  }, false)

  app.provide('LitJsSdk', LitJsSdk)
  const litNodeClient = new LitJsSdk.LitNodeClient({ debug: false })
  app.provide('litNodeClient', litNodeClient)
  await litNodeClient.connect()
}
