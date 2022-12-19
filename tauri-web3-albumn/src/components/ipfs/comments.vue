<script setup lang="ts">
import { UseTimeAgo } from '@vueuse/components'

interface Props {
  items: Array
}
const {
  items,
} = defineProps<Props>()

const { getJson, getStatus } = $(useNFTStorage())

let jsonItems = $ref([])
watchEffect(async() => {
  jsonItems = await Promise.all(items.map(cid => getJson(cid)))
  const status = await Promise.all(items.map(cid => getStatus(cid)))
  if (status.length > 0) {
    jsonItems = jsonItems.map((item, index) => {
      return {
        ...item,
        created: status[index].created,
      }
    })
  }
})

const theName = (name) => {
  if (name) return `@${name}`
  return '@Anonymous'
}
</script>
<template>
  <ul role="list">
    <li v-for="item in jsonItems" :key="item">
      <div class="flex py-6 px-5 group relative items-center">
        <div class="flex-1 -m-1 p-1 block">
          <div class="inset-0 absolute group-hover:bg-gray-50" aria-hidden="true" />
          <div class="flex flex-1 min-w-0 relative items-center">
            <span class="flex-shrink-0 relative inline-block">
              <IpfsImg class="rounded-full h-10 w-10" :src="item.avatar" alt="" />
              <span :class="[item.status === 'online' ? 'bg-green-400' : 'bg-gray-300', 'hidden absolute top-0 right-0 block h-2.5 w-2.5 rounded-full ring-2 ring-white']" aria-hidden="true" />
            </span>
            <div class="flex-1 mx-4">
              <p class="flex text-sm text-gray-500 truncate">
                <span class="font-bold mr-2">{{ theName(item.name) }}</span>
                <UseTimeAgo v-if="item.created" v-slot="{ timeAgo }" :time="item.created">
                  {{ timeAgo }}
                </UseTimeAgo>
                <eos-icons:loading v-else class="h-6 text-black w-6" />
              </p>
              <p class="font-medium text-sm text-gray-900 break-words">
                {{ item.comment }}
              </p>
            </div>
            <div class="rounded-lg flex flex-col border-gray-400 border-1 p-1 px-2 text-gray-600 items-center justify-center">
              <ant-design:caret-up-filled class="h-6 w-6 block" />
              {{ item.amount }}
            </div>
          </div>
        </div>
      </div>
    </li>
  </ul>
</template>
