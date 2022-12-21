import type { UserModule } from '~/types'

// https://github.com/antfu/vite-plugin-pwa#automatic-reload-when-new-content-available
export const install: UserModule = ({ isClient, router }) => {
  if (!isClient)
    return

  router.options.scrollBehavior = (to, from, savedPosition) => {
    if (savedPosition)
      return savedPosition

    else
      return { top: 0, behavior: 'smooth' }
  }
}
