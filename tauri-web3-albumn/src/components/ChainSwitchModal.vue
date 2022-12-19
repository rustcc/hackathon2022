<script setup lang="ts">
const { isLoading, error, doSwitchChain, isShowChainSwitchModal } = $(web3AuthStore())

</script>
<template>
  <DialogDefault :show="isShowChainSwitchModal">
    <div>
      <div class="mt-3 text-center sm:mt-5">
        <DialogTitle as="h3" class="font-medium text-lg text-gray-900 leading-6">
          Wrong Chain
        </DialogTitle>
      </div>
    </div>
    <div class="mt-5  sm:mt-6">
      <div class="rounded-md bg-red-50 my-4 text-center text-sm p-4 text-red-700" v-show="error">
        {{error}}
      </div>
      <btn-black @click="doSwitchChain" :isLoading="isLoading" class="w-full">
        Click here to Switch Chain
      </btn-black>
    </div>
  </DialogDefault>
</template>