<script setup lang="ts">
const { doLogin, isLoading, error, isShowLoginModal } = $(web3AuthStore())

</script>
<template>
  <DialogDefault :show="isShowLoginModal">
    <div>
      <div class="mt-3 text-center sm:mt-5">
        <DialogTitle as="h3" class="font-medium text-lg text-gray-900 leading-6">
          Login Now
        </DialogTitle>
      </div>
    </div>
    <div class="mt-5  sm:mt-6">
      <div class="rounded-md bg-red-50 my-4 text-center text-sm p-4 text-red-700" v-show="error">
        {{error}}
      </div>
      <btn-black @click="doLogin" :isLoading="isLoading" class="w-full">
        Click here to Login with metamask
      </btn-black>
    </div>
  </DialogDefault>
</template>