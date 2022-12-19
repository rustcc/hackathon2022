<script setup lang="ts">
import { ethers } from 'ethers'
const { initContract, web3Provider } = $(web3AuthStore())

const emit = defineEmits(['update:modelValue'])
interface Props {
  address: string
}
const {
  address,
} = defineProps<Props>()

const voteUpCount = $ref(0)
const voteDownCount = $ref(0)
const doAction = async(_voteType) => {
  const contractWriter = await initContract('Vote', true)

  const value = ethers.utils.parseEther('0.01')
  const voteType = _voteType === 'VoteUp' ? 0 : 1

  try {
    const rz = await contractWriter.vote(address, voteType, { value })
    console.log('====> rz :', rz)
  }
  catch (e) {
    console.log('====> e :', e)
  }
}

watchEffect(async() => {
  if (!address || !web3Provider) return
  const contract = await initContract('Vote')
  voteUpCount = await contract.voteUpCount(address)
  voteDownCount = await contract.voteDownCount(address)
  console.log('====> voteUpCount :', voteUpCount, voteDownCount)
})
</script>
<template>
  <span class="rounded-md shadow-sm z-0 relative inline-flex">
    <button type="button" class="bg-white border rounded-l-md font-medium border-gray-300 text-sm py-2 px-4 text-gray-700 relative inline-flex items-center hover:bg-gray-50 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 focus:z-10" @click="doAction('VoteUp')">
      <akar-icons:chevron-up class="h-5 mr-2 -ml-1 text-gray-400 w-5" aria-hidden="true" />
      {{ voteUpCount }}
    </button>
    <button type="button" class="bg-white border rounded-r-md font-medium -ml-px border-gray-300 text-sm py-2 px-3 text-gray-700 relative inline-flex items-center hover:bg-gray-50 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 focus:z-10" @click="doAction('VoteDown')">
      <akar-icons:chevron-down class="h-5 mr-2 -ml-1 text-gray-400 w-5" aria-hidden="true" />
      {{ voteDownCount }}
    </button>
  </span>
</template>
