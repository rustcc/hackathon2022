import { useClipboard } from '@vueuse/core'
import * as ls from '~/helpers/ls'

export const inviteStore = defineStore('inviteStore', () => {
  let refUser = $ref(ls.getItem('invite.refUser'), '')
  let showInviteDialog = $ref(false)
  let refLink = $ref('')
  const { walletAddress } = $(web3AuthStore())
  const { copy, copied } = useClipboard()
  const params = useUrlSearchParams('history')

  const toggleInviteDialog = () => {
    showInviteDialog = !showInviteDialog
  }

  watchEffect(async() => {
    if (!walletAddress) return

    refLink = `${location.origin}${location.pathname}?refUser=${walletAddress}`
  })
  const saveRefUserWhileMounted = () => {
    if (refUser !== '') return
    if (!params.refUser) return
    refUser = params.refUser
    ls.setItem('invite.refUser', refUser)
  }

  return $$({
    showInviteDialog,
    refLink,
    copied,
    toggleInviteDialog,
    copy,
    saveRefUserWhileMounted,
  })
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(inviteStore, import.meta.hot))
