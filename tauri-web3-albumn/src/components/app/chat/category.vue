<script setup lang="ts">
const tabs = [
  { name: 'All', href: '#', count: '52', current: false },
  { name: 'By NFT', href: '#', count: '6', current: false },
  { name: 'By Space', href: '#', count: '4', current: true },
]
</script>
<template>
  <div class="bg-white flex flex-col justify-center">
    <div class="sm:hidden">
      <label for="tabs" class="sr-only">Select a tab</label>
      <!-- Use an "onChange" listener to redirect the user to the selected tab URL. -->
      <select id="tabs" name="tabs" class="rounded-md border-gray-300 text-base w-full py-2 pr-10 pl-3 block sm:text-sm focus:outline-none focus:border-indigo-500 focus:ring-indigo-500">
        <option v-for="tab in tabs" :key="tab.name" :selected="tab.current">
          {{ tab.name }}
        </option>
      </select>
    </div>
    <div class="hidden sm:block">
      <div class="border-b border-gray-200">
        <nav class="flex -mb-px space-x-8 px-6" aria-label="Tabs">
          <a v-for="tab in tabs" :key="tab.name" href="#" :class="[tab.current ? 'border-indigo-500 text-indigo-600' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-200', 'whitespace-nowrap flex py-4 px-1 border-b-2 font-medium text-sm']" :aria-current="tab.current ? 'page' : undefined">
            {{ tab.name }}
            <span v-if="tab.count" :class="[tab.current ? 'bg-indigo-100 text-indigo-600' : 'bg-gray-100 text-gray-900', 'hidden ml-3 py-0.5 px-2.5 rounded-full text-xs font-medium md:inline-block']">{{ tab.count }}</span>
          </a>
        </nav>
      </div>
    </div>
  </div>
</template>
