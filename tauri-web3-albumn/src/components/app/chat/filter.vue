<script setup lang="ts">
import {
  ChevronDownIcon,
} from '@heroicons/vue/24/solid'
const name = 'Friends'
const items = [
  { name: 'Friends', href: '#' },
  { name: 'Follow', href: '#' },
  { name: 'Follower', href: '#' },
  { name: 'All', href: '#' },
  { name: 'Archive', href: '#' },
]
</script>
<template>
  <div class="bg-white border-t border-b flex flex-col font-medium bg-gray-50 border-gray-200 h-8 px-6 justify-center">
    <Menu as="div" class="text-left relative">
      <MenuButton class="rounded-md flex font-medium  text-sm w-full text-gray-900 items-center justify-end">
        <h2 class="font-medium text-xs  text-gray-900">
          Filter by {{ name }}
        </h2>
        <ChevronDownIcon class="h-5 ml-1 text-gray-500 w-5" aria-hidden="true" />
      </MenuButton>

      <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
        <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 origin-top-right right-0 ring-1 ring-opacity-5 w-40 z-30 absolute focus:outline-none">
          <div class="py-1">
            <MenuItem v-for="child in items" :key="child.name" v-slot="{ active }">
              <a :href="child.href" :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']">
                {{ child.name }}
              </a>
            </MenuItem>
          </div>
        </MenuItems>
      </transition>
    </Menu>
  </div>
</template>
