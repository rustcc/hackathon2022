import { createI18n } from 'vue-i18n'
import type { UserModule } from '~/types'
import * as ls from '~/helpers/ls'

// Import i18n resources
// https://vitejs.dev/guide/features.html#glob-import
//
// Don't need this? Try vitesse-lite: https://github.com/antfu/vitesse-lite

const messages = Object.fromEntries(
  Object.entries(
    import.meta.globEager('../../locales/*.y(a)?ml'))
    .map(([key, value]) => {
      const yaml = key.endsWith('.yaml')
      return [key.slice(14, yaml ? -5 : -4), value.default]
    }),
)
export const install: UserModule = ({ app }) => {
  const locale = ls.getItem('locale', 'en')
  const i18n = createI18n({
    legacy: false,
    globalInjection: true, // https://vue-i18n.intlify.dev/guide/advanced/composition.html#implicit-with-injected-properties-and-functions
    locale,
    messages,
  })

  app.use(i18n)
}
