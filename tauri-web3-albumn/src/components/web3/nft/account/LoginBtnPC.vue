<script setup lang="ts">
const { doLogin, isLoading, isShowOnboardModal, doOnboard } = $(web3Store())
const text = $computed(() => isLoading ? 'Loading' : 'Login')
</script>
<template>
  <div class="flex items-center">
    <btn-black :disabled="isLoading" @click="doLogin" :isLoading="isLoading">
      {{ text }}
    </btn-black>
    <DialogDefault :show="isShowOnboardModal">
      <div>
        <div class="mt-3 text-center sm:mt-5">
          <DialogTitle as="h3" class="font-medium text-lg text-gray-900 leading-6">
            MetaMask Required
          </DialogTitle>
        </div>
      </div>
      <div class="mt-5 sm:mt-6">
        <btn-black @click="doOnboard">
          Click here to install MetaMask
        </btn-black>
      </div>
    </DialogDefault>
  </div>
</template>
