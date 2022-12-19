<script setup lang="ts">
import axios from 'axios'
import { openseaTokenLink } from '~/helpers/opensea'
import * as ls from '~/helpers/ls'

interface Props {
  address: string
}
const {
  address,
} = defineProps<Props>()

let isLoading = $ref(true)
const url = $computed(() => `https://saas-api.jjf-tech.cn/ifs-api/6132bc67ff155500015e8680/w3ns/userWalletAddress?userWalletAddress=${address}`)
const { isFetching, error, data, execute } = useFetch($$(url), { refetch: true, immediate: false }).get().json()

let items = $ref([])
const total = $computed(() => get(data, 'value.data.total', 0))

watch(isFetching, (val) => {
  isLoading = val
})

watchEffect(() => {
  if (!address) return
  execute()
})

watchEffect(async() => {
  let validateNfts = get(data, 'value.data.nfts', [])
  if (validateNfts.length === 0) {
    items = []
    return
  }

  validateNfts = validateNfts.filter(item => item.cached_file_url)
  items = validateNfts
})

const openseaUserHome = $computed(() => `https://testnets.opensea.io/${address}`)
const placeHolderImg = 'https://storage.googleapis.com/opensea-static/Logomark/Logomark-Transparent%20White.png'
</script>
<template>
  <div>
    <div v-if="isLoading" class="flex p-10 justify-center items-center">
      <eos-icons:loading class="h-10 w-10" />
    </div>
    <div v-else-if="error" class="flex p-10 justify-center items-center">
      {{ error }}
    </div>
    <div v-else-if="items.length">
      <div class="border border-gray-200 mb-5 p-5">
        <a class="font-medium  text-lg text-right text-gray-900 leading-6 block hover:underline" :href="openseaUserHome" target="_blank">
          All My NFTs ({{ total }})
        </a>
      </div>
      <ul role="list" class="grid gap-x-4 gap-y-8 grid-cols-1 sm:gap-x-6 sm:grid-cols-3 lg:grid-cols-4 xl:gap-x-8">
        <li v-for="item in items" :key="item.source" class="relative">
          <div class="rounded-lg bg-gray-100 w-full group block overflow-hidden aspect-w-5 aspect-h-7 focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-offset-gray-100 focus-within:ring-indigo-500">
            <img :src="item.cached_file_url || placeHolderImg" alt="" class="object-cover pointer-events-none group-hover:opacity-75">
            <a :href="openseaTokenLink(item)" target="_blank" class="inset-0 absolute focus:outline-none">
              <span class="sr-only">View details for {{ item.name }}# {{ item.token_id }}</span>
            </a>
          </div>
          <p class="font-medium mt-2 text-sm text-gray-900 block truncate pointer-events-none">
            {{ item.title }}
          </p>
          <p class="font-medium text-sm text-gray-500 block pointer-events-none">
            {{ item.size }}
          </p>
        </li>
      </ul>
    </div>
    <div v-else class="flex border-1 p-10 justify-center items-center">
      <h2 class="font-bold">
        User don't have any NFT yet.
      </h2>
    </div>
  </div>
</template>
