import Arweave from 'arweave'
import type { UserModule } from '~/types'

// { app, router, routes, isClient, initialState }
export const install: UserModule = async({ isClient, app }) => {
  if (!isClient)
    return

  const arweave = Arweave.init({})
  app.provide('arweave', arweave)
}
