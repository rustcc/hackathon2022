<script setup lang="ts">
import { UseTimeAgo } from '@vueuse/components'

import {
  HeartIcon,
} from '@heroicons/vue/24/outline'
import {
  Bars4Icon,
  // PencilIcon,
  PlusIcon as PlusIconMini,
  HeartIcon as HeartIconSolid,
  Squares2X2Icon as Squares2X2IconMini,
} from '@heroicons/vue/20/solid'

const tabs = [
  // { name: 'Recently Viewed', href: '#', current: true },
  { name: 'Recently', href: '#', current: true },
  { name: 'Favorited', href: '#', current: false },
]

const route = useRoute()

const { getIpfsUrl } = $(useNFTStorage())
const { items, currentItem, currentCategory, showNewItemModal, updateCategory, setCurrentItem, toggleFavorited, toggleDeleted } = $(box3Store())

updateCategory('')

</script>

<template>
  <div class="flex flex-1 items-stretch overflow-hidden">
    <main class="flex-1 overflow-y-auto">
      <div class="mx-auto max-w-7xl px-4 pt-8 sm:px-6 lg:px-8">
        <div class="flex">
          <h1 class="border-b font-bold flex-1 pb-4 text-2xl text-gray-900">
            {{ currentCategory || 'All' }}({{ items.length }} Items)
          </h1>
          <div class="rounded-lg flex bg-gray-100 ml-6 p-0.5 items-center sm:hidden">
            <button type="button" class="rounded-md p-1.5 text-gray-400 hover:bg-white hover:shadow-sm focus:outline-none focus:ring-inset focus:ring-2 focus:ring-indigo-500">
              <Bars4Icon class="h-5 w-5" aria-hidden="true" />
              <span class="sr-only">Use list view</span>
            </button>
            <button type="button" class="bg-white rounded-md shadow-sm ml-0.5 p-1.5 text-gray-400 focus:outline-none focus:ring-inset focus:ring-2 focus:ring-indigo-500">
              <Squares2X2IconMini class="h-5 w-5" aria-hidden="true" />
              <span class="sr-only">Use grid view</span>
            </button>
          </div>
        </div>

        <!-- Tabs -->
        <div class="mt-3 sm:mt-2">
          <div class="sm:hidden">
            <label for="tabs" class="sr-only">Select a tab</label>
            <!-- Use an "onChange" listener to redirect the user to the selected tab URL. -->
            <select id="tabs" name="tabs" class="rounded-md border-gray-300 text-base w-full py-2 pr-10 pl-3 block sm:text-sm focus:outline-none focus:border-indigo-500 focus:ring-indigo-500" @change="setFilter">
              <option selected="">
                Recently Added
              </option>
              <option>Favorited</option>
            </select>
          </div>
          <!-- <div class="hidden sm:block">
            <div class="border-b flex border-gray-200 items-center">
              <nav class="flex -mb-px space-x-6 flex-1 xl:space-x-8" aria-label="Tabs">
                <button v-for="tab in tabs" :key="tab.name" :href="tab.href" :aria-current="tab.current ? 'page' : undefined" :class="[tab.current ? 'border-indigo-500 text-indigo-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300', 'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm']">
                  {{ tab.name }}
                </button>
              </nav>
              <div v-if="false" class="rounded-lg bg-gray-100 ml-6 p-0.5  items-center sm:flex ">
                <button type="button" class="rounded-md p-1.5 text-gray-400 hover:bg-white hover:shadow-sm focus:outline-none focus:ring-inset focus:ring-2 focus:ring-indigo-500">
                  <Bars4Icon class="h-5 w-5" aria-hidden="true" />
                  <span class="sr-only">Use list view</span>
                </button>
                <button type="button" class="bg-white rounded-md shadow-sm ml-0.5 p-1.5 text-gray-400 focus:outline-none focus:ring-inset focus:ring-2 focus:ring-indigo-500">
                  <Squares2X2IconMini class="h-5 w-5" aria-hidden="true" />
                  <span class="sr-only">Use grid view</span>
                </button>
              </div>
            </div>
          </div> -->
        </div>

        <!-- Gallery -->
        <section class="mt-8 pb-16" aria-labelledby="gallery-heading">
          <h2 id="gallery-heading" class="sr-only">
            Recently viewed
          </h2>
          <ul v-if="items.length > 0" role="list" class="grid gap-x-4 gap-y-8 grid-cols-2 sm:gap-x-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-3 xl:gap-x-8 xl:grid-cols-4">
            <li v-for="item in items" :key="item.name" class=" relative">
              <div :class="[item.current ? 'ring-2 ring-offset-2 ring-indigo-500' : 'focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-offset-gray-100 focus-within:ring-indigo-500', 'cursor-pointer group block w-full aspect-w-10 aspect-h-7 rounded-lg bg-gray-100 overflow-hidden']" @click="setCurrentItem(item)">
                <IpfsImg :src="item.image" alt="" :class="[item.current ? '' : 'group-hover:opacity-75', 'object-cover pointer-events-none']">
                  <button type="button" class="inset-0 absolute focus:outline-none">
                    <span class="sr-only">View details for {{ item.name }}</span>
                  </button>
                </ipfsimg>
              </div>
              <p class="font-medium mt-2 text-sm text-gray-900 pointer-events-none block truncate">
                {{ item.name }}
              </p>
              <p class="flex font-medium text-sm text-gray-500 pointer-events-none justify-between items-center">
                <span><UseTimeAgo v-if="item.status?.created" v-slot="{ timeAgo }" :time="item.status?.created">
                  {{ timeAgo }}
                </UseTimeAgo></span>
                <HeartIconSolid v-if="item.isFavorited" class="h-5 text-red-500 w-5" aria-hidden="true" />
              </p>
            </li>
          </ul>
          <Empty v-else-if="currentCategory !== 'trash'" @newBtnClick="showNewItemModal = true">
            Create New
          </Empty>
        </section>
      </div>
    </main>

    <!-- Details sidebar -->
    <aside class="bg-white border-l border-gray-200 p-8 w-96 hidden overflow-y-auto lg:block">
      <div v-if="currentItem?.cid" class="space-y-6 pb-16">
        <div>
          <div class="rounded-lg cursor-pointer w-full block overflow-hidden aspect-w-10 aspect-h-7">
            <IpfsImg :has-modal="true" :src="currentItem.image" alt="" class="object-cover" />
          </div>
          <div class="flex mt-4 items-start justify-between">
            <div>
              <h2 class="font-medium text-lg text-gray-900">
                <span class="sr-only">Details for </span>{{ currentItem.name }}
              </h2>
              <p class="font-medium text-sm text-gray-500">
                <UseTimeAgo v-if="currentItem?.status?.created" v-slot="{ timeAgo }" :time="currentItem.status?.created">
                  {{ timeAgo }}
                </UseTimeAgo>
              </p>
            </div>
            <button type="button" class="bg-white rounded-full flex h-8 ml-4 text-gray-400 w-8 items-center justify-center hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500" @click="toggleFavorited(currentItem)">
              <HeartIconSolid v-if="currentItem.isFavorited" class="h-6 text-red-500 w-6" aria-hidden="true" />
              <HeartIcon v-else class="h-6  w-6" aria-hidden="true" />
              <span class="sr-only">Favorite</span>
            </button>
          </div>
        </div>
        <div>
          <h3 class="font-medium text-gray-900">
            Information
          </h3>
          <dl class="divide-y border-t border-b divide-gray-200 border-gray-200 mt-2">
            <div v-for="key in Object.keys(currentItem?.properties || {})" :key="key" class="flex font-medium text-sm py-3 justify-between">
              <dt class="text-gray-500">
                {{ key }}
              </dt>
              <dd class="text-gray-900 whitespace-nowrap">
                {{ currentItem.properties[key] }}
              </dd>
            </div>
          </dl>
        </div>
        <div>
          <h3 class="font-medium text-gray-900">
            Description
          </h3>
          <div class="flex mt-2 items-center justify-between">
            <p class="text-sm text-gray-500">
              {{ currentItem.description }}
            </p>
          <!-- <button type="button" class="bg-white rounded-full flex h-8 text-gray-400 w-8 items-center justify-center hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500">
            <PencilIcon class="h-5 w-5" aria-hidden="true" />
            <span class="sr-only">Add description</span>
          </button> -->
          </div>
        </div>
        <div class="hidden">
          <h3 class="font-medium text-gray-900">
            Shared with
          </h3>
          <ul role="list" class="divide-y border-t border-b divide-gray-200 border-gray-200 mt-2">
            <li v-for="person in currentItem.sharedWith" :key="person.id" class="flex py-3 items-center justify-between">
              <div class="flex items-center">
                <img :src="person.imageUrl" alt="" class="rounded-full h-8 w-8">
                <p class="font-medium text-sm ml-4 text-gray-900">
                  {{ person.name }}
                </p>
              </div>
              <button type="button" class="bg-white rounded-md font-medium text-sm ml-6 text-indigo-600 hover:text-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                Remove<span class="sr-only"> {{ person.name }}</span>
              </button>
            </li>
            <li class="flex py-2 items-center justify-between">
              <button type="button" class="bg-white rounded-md flex -ml-1 p-1 group items-center focus:outline-none focus:ring-2 focus:ring-indigo-500">
                <span class="border-dashed rounded-full flex border-2 border-gray-300 h-8 text-gray-400 w-8 items-center justify-center">
                  <PlusIconMini class="h-5 w-5" aria-hidden="true" />
                </span>
                <span class="font-medium text-sm ml-4 text-indigo-600 group-hover:text-indigo-500">Share</span>
              </button>
            </li>
          </ul>
        </div>
        <div class="flex">
          <a target="_blank" :href="getIpfsUrl(currentItem.image)" class="border border-transparent rounded-md font-medium bg-indigo-600 flex-1 shadow-sm text-center text-sm text-white py-2 px-4 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
            Download
          </a>
          <button type="button" class="bg-white border rounded-md font-medium border-gray-300 flex-1 shadow-sm text-sm ml-3 py-2 px-4 text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" @click="toggleDeleted(currentItem)">
            {{ currentItem.isDeleted ? 'Recover' : 'Delete' }}
          </button>
        </div>
      </div>
    </aside>
  </div>
</template>

<route lang="yaml">
meta:
  layout: box3
</route>
