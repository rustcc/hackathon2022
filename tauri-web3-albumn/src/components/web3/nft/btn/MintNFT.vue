<script setup lang="ts">
import { ethers } from 'ethers'
const emit = defineEmits(['update:modelValue'])
interface Props {
  address: string
}
const {
  address,
} = defineProps<Props>()

const { initContract } = $(web3AuthStore())
let isLoading = $ref(false)
const doAction = async() => {
  isLoading = true
  const contractWriter = await initContract('CTC', true)

  const mintAmount = 1
  const proof = []
  const value = ethers.utils.parseEther('0.1')

  try {
    const rz = await contractWriter.mint(mintAmount, false, proof, { value })
    console.log('====> rz :', rz)
  }
  catch (e) {
    console.log('====> e :', e)
  }
  isLoading = false
}

</script>
<template>
  <btn-black :is-loading="isLoading" @click="doAction">
    Mint NFT
  </btn-black>
</template>
