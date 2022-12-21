<script setup lang="ts">
import CyberConnect, { Blockchain, ConnectionType, Env } from '@cyberlab/cyberconnect'
import { find } from 'lodash'
import { isSameAddress } from '~/helpers/web3'
const emit = defineEmits(['update'])

interface Props {
  address: string
  hideIcon?: Boolean
  alias?: string
}
const {
  address,
  hideIcon = false,
  alias = '',
} = defineProps<Props>()

const { addMessage } = $(notificationStore())
const { signer, doLogin } = $(web3Store())
const { identity, isFetching } = $(myIdentityStore())

let cyberConnect = $ref('')

let isFollow = $ref(false)
const isLoading = $computed(() => isFetching)

watchEffect(() => {
  if (isFetching || !identity) return
  const hasMatch = find(identity.followings.list, item => isSameAddress(item.address, address))
  if (hasMatch)
    isFollow = true
})

const toggleFollow = async() => {
  if (!signer)
    await doLogin()

  try {
    if (!cyberConnect) {
      cyberConnect = new CyberConnect({
        namespace: 'Web3NFT.Social',
        env: Env.PRODUCTION,
        chain: Blockchain.ETH,
        provider: signer.provider,
        signingMessageEntity: 'Web3NFT.Social',
      })
    }

    if (!isFollow) {
      isFollow = true
      addMessage({ title: 'Submit follow action Success!' })
      await cyberConnect.connect(address, alias, ConnectionType.FOLLOW)
    }
    else {
      isFollow = false
      addMessage({ title: 'Submit unfollow action Success!' })
      await cyberConnect.disconnect(address)
    }
    emit('update')
  }
  catch (error) {
    console.error(error.message)
  }
}
</script>

<template>
  <button type="button" class="bg-white border rounded-md font-medium border-gray-300 shadow-sm text-sm py-2 px-4 text-gray-700 inline-flex justify-center hover:bg-gray-50 focus:outline-none disabled:(cursor-not-allowed) " :disabled="isLoading" @click="toggleFollow">
    <template v-if="isLoading">
      <eos-icons:loading />
    </template>
    <template v-else-if="isFollow">
      <ri:user-follow-line class="h-5 text-gray-400 w-5" aria-hidden="true" />
    </template>
    <template v-else>
      <simple-line-icons:user-follow v-if="!hideIcon" class="h-5 mr-2 -ml-1 text-gray-400 w-5" aria-hidden="true" />
      <span>Follow</span>
    </template>
  </button>
</template>
