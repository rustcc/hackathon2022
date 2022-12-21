import type { UserModule } from '~/types'

// https://github.com/antfu/vite-plugin-pwa#automatic-reload-when-new-content-available
export const install: UserModule = ({ app, isClient }) => {
  if (!isClient)
    return

  app.config.errorHandler = (err) => {
   console.log('====> err :', err)
  // alert(err);
};
}
