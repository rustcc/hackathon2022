import VueGtag from 'vue-gtag'
import { UserModule } from '~/types'

export const install: UserModule = ({ isClient, app, router }) => {
  if (!isClient) return

  const opts = {
    config: { id: import.meta.env.VITE_GTAG_ID }
  }
  app.use(VueGtag, opts, router)
}
