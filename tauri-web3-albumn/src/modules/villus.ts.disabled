import { createClient } from 'villus'
import type { UserModule } from '~/types'

// https://villus.logaretm.com/guide/queries
export const install: UserModule = ({ isClient, app }) => {
  if (!isClient) return

  const client = createClient({
    url: 'https://api.cybertino.io/connect/',
  })
  app.use(client)
}
