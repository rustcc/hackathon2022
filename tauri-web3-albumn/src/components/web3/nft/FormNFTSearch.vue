<script setup lang="ts">
// import { MagnifyingGlassIcon } from '@heroicons/vue/24/solid'
const route = useRoute()
const defaultChainId = Object.keys(WEB3_NETWORK_LIST)[0]
const address = ref(route.params.address || '')
const chainId = ref(route.params.chainId || defaultChainId)

const router = useRouter()
const go = () => {
  // router.push(`/${address.value}`)
  router.push('/0xC6E58fb4aFFB6aB8A392b7CC23CD3feF74517F6C')
}

</script>
<template>
  <select id="chainId" v-model="chainId" name="chainId" class="rounded-md border-gray-300 mr-2 text-base py-2 pr-10 pl-3 block sm:text-sm focus:outline-none focus:border-indigo-500 focus:ring-indigo-500">
    <option v-for="(name, id) in WEB3_NETWORK_LIST" :key="id" :value="id">
      {{ name }}
    </option>
  </select>
  <div class="max-w-lg w-full lg:max-w-xs">
    <label for="search" class="sr-only">Search</label>
    <div class="text-gray-400 relative focus-within:text-gray-600">
      <input id="search" v-model="address" class="bg-white border rounded-md w-full py-2 pr-3 pl-3 placeholder-gray-500 text-gray-900 leading-5 block sm:text-sm focus:border-white focus:outline-none focus:ring-white focus:ring-2 focus:ring-offset-2 focus:ring-offset-indigo-600" placeholder="NFT Contract Address 0xXXXXX...XXXX" type="search" name="search">
    </div>
  </div>
  <button type="button" class="border border-transparent rounded-md font-medium bg-sky-400 shadow-sm text-sm text-white ml-2 py-2 px-3 leading-4 inline-flex items-center hover:bg-sky-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 disabled:(bg-gray-400 cursor-not-allowed) " :disabled="!address" @click="go">
    <!-- <MagnifyingGlassIcon class="h-4 w-4" aria-hidden="true" /> -->
    Search
  </button>
</template>
