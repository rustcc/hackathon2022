<script setup lang="ts">
import { shortAddress } from '~/helpers/web3'
const { userNavigation, walletAddress } = web3Store()
</script>
<template>
  <div class="hidden lg:flex lg:ml-4 lg:items-center ">
    <button type="button" class="rounded-full bg-sky-500 flex-shrink-0 p-1 text-sky-200 hover:text-white focus:outline-none focus:ring-white focus:ring-2 focus:ring-offset-2 focus:ring-offset-sky-500">
      <span class="sr-only">Notificaitons</span>
      <akar-icons:bell class="h-6 w-6" aria-hidden="true" />
    </button>

    <!-- Profile dropdown -->
    <Menu as="div" class="flex-shrink-0 ml-4 relative">
      <div>
        <MenuButton class="rounded-full flex bg-sky-500 text-sm text-white items-center focus:outline-none focus:ring-white focus:ring-2 focus:ring-offset-2 focus:ring-offset-sky-500">
          <span class="sr-only">Open user menu</span>
          {{ shortAddress(walletAddress) }}
          <EthAvatar :address="walletAddress" class="rounded-full h-8 ml-1 w-8" />
        </MenuButton>
      </div>
      <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
        <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 py-1 origin-top-right right-0 ring-1 ring-opacity-5 w-48 absolute focus:outline-none">
          <MenuItem v-for="item in userNavigation" :key="item.name" v-slot="{ active }">
            <button v-if="item.onClick" class="text-sm text-left w-full py-2 px-4 text-gray-700 hover:bg-gray-100" @click="item.onClick">
              {{ item.name }}
            </button>
            <router-link v-else :to="item.href" :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']">
              {{ item.name }}
            </router-link>
          </MenuItem>
        </MenuItems>
      </transition>
    </Menu>
  </div>
</template>
