<script setup lang="ts">
import {
  BellIcon,
  CreditCardIcon,
  KeyIcon,
  UserCircleIcon,
  ViewGridAddIcon,
} from '@heroicons/vue/24/outline'
import HackerIcon from '~icons/ant-design/code-outlined'
import ArtistIcon from '~icons/emojione-monotone/artist-palette'

const route = useRoute()

const subNavigation = [
  { name: 'Web3Home', href: '/settings/Web3Home', icon: UserCircleIcon },
  // { name: 'HackerNFT', href: '/settings/HackerNFT', icon: HackerIcon },
  // { name: 'ArtistNFT', href: '/settings/ArtistNFT', icon: ArtistIcon },
  // { name: 'Password', href: '#', icon: KeyIcon, current: false },
  // { name: 'Notifications', href: '#', icon: BellIcon, current: false },
  // { name: 'Billing', href: '#', icon: CreditCardIcon, current: false },
  // { name: 'Integrations', href: '#', icon: ViewGridAddIcon, current: false },
]

const isCurrent = (item) => {
  return item.href === route.path
}
</script>

<template>
  <aside class="py-6 lg:col-span-2">
    <nav class="space-y-1">
      <router-link v-for="item in subNavigation" :key="item.name" :to="item.href" :class="[isCurrent(item) ? 'bg-teal-50 border-teal-500 text-teal-700 hover:bg-teal-50 hover:text-teal-700' : 'border-transparent text-gray-900 hover:bg-gray-50 hover:text-gray-900', 'group border-l-4 px-3 py-2 flex items-center text-sm font-medium']" :aria-current="isCurrent(item) ? 'page' : undefined">
        <component :is="item.icon" :class="[isCurrent(item) ? 'text-teal-500 group-hover:text-teal-500' : 'text-gray-400 group-hover:text-gray-500', 'flex-shrink-0 -ml-1 mr-3 h-6 w-6']" aria-hidden="true" />
        <span class="truncate">
          {{ item.name }}
        </span>
      </router-link>
    </nav>
  </aside>
</template>
