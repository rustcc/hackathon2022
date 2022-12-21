<script setup lang="ts">
const { doOnboard, isShowOnboardModal } = $(web3AuthStore())

</script>
<template>
  <DialogDefault :show="isShowOnboardModal">
    <div>
      <div class="mt-3 text-center sm:mt-5">
        <DialogTitle as="h3" class="font-medium text-lg text-gray-900 leading-6">
          MetaMask Required
        </DialogTitle>
      </div>
    </div>
    <div class="mt-5 sm:mt-6">
      <btn-black @click="doOnboard" class="w-full">
        Click here to install MetaMask
      </btn-black>
    </div>
  </DialogDefault>
</template>
