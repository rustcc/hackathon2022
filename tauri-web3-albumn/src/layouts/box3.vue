<script setup type="ts">
import { ref } from 'vue'
import {
  Dialog,
  DialogPanel,
  Menu,
  MenuButton,
  MenuItem,
  MenuItems,
  TransitionChild,
  TransitionRoot,
} from '@headlessui/vue'
import {
  Bars3BottomLeftIcon,
  CogIcon,
  HeartIcon,
  HomeIcon,
  PhotoIcon,
  PlusIcon as PlusIconOutline,
  RectangleStackIcon,
  Squares2X2Icon as Squares2X2IconOutline,
  UserGroupIcon,
  XMarkIcon,
} from '@heroicons/vue/24/outline'
import {
  ArrowPathIcon,
  Bars4Icon,
  HeartIcon as HeartIconSolid,
  InboxIcon,
  MagnifyingGlassIcon,
  PencilIcon,
  PlusIcon as PlusIconMini,
  Squares2X2Icon as Squares2X2IconMini,
  TrashIcon,
} from '@heroicons/vue/20/solid'

const { deletedCidArr, categoriesArr, currentCategory, showNewItemModal, favoritedCidArr, needSave, isSaving, saveData } = $(box3Store())

const navigation = $computed(() => {
  const arr = [
    { name: 'All', href: '/', icon: Squares2X2IconOutline, current: currentCategory === undefined },
  ]

  if (favoritedCidArr.length > 0)
    arr.unshift({ name: 'Favorited', href: '/favorited', icon: HeartIconSolid, current: currentCategory === 'favorited' })

  if (categoriesArr.length > 0)
    categoriesArr.forEach(name => arr.push({ name, href: `/${name}`, icon: InboxIcon, current: currentCategory === name }))

  if (deletedCidArr.length > 0)
    arr.push({ name: 'Trash', href: '/trash', icon: TrashIcon, current: currentCategory === 'trash' })

  return arr
})
const userNavigation = [
  { name: 'Your profile', href: '#' },
  { name: 'Sign out', href: '#' },
]

const mobileMenuOpen = ref(false)
</script>

<template>
  <div class="flex h-full">
    <!-- Narrow sidebar -->
    <div class="bg-indigo-700 w-28 hidden overflow-y-auto md:block">
      <div class="flex flex-col w-full py-6 items-center">
        <div class="flex flex-shrink-0 items-center">
          <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=white" alt="Your Company">
        </div>
        <div class="space-y-1 flex-1 mt-6 w-full px-2">
          <router-link v-for="item in navigation" :key="item.name" :to="item.href" :href="item.href" :class="[item.current ? 'bg-indigo-800 text-white' : 'text-indigo-100 hover:bg-indigo-800 hover:text-white', 'group w-full p-3 rounded-md flex flex-col items-center text-xs font-medium']" :aria-current="item.current ? 'page' : undefined">
            <component :is="item.icon" :class="[item.current ? 'text-white' : 'text-indigo-300 group-hover:text-white', 'h-6 w-6']" aria-hidden="true" />
            <span class="mt-2">{{ item.name }}</span>
          </router-link>
        </div>
      </div>
    </div>

    <!-- Mobile menu -->
    <TransitionRoot as="template" :show="mobileMenuOpen">
      <Dialog as="div" class="z-40 relative md:hidden" @close="mobileMenuOpen = false">
        <TransitionChild as="template" enter="transition-opacity ease-linear duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="transition-opacity ease-linear duration-300" leave-from="opacity-100" leave-to="opacity-0">
          <div class="bg-gray-600 bg-opacity-75 inset-0 fixed" />
        </TransitionChild>

        <div class="flex inset-0 z-40 fixed">
          <TransitionChild as="template" enter="transition ease-in-out duration-300 transform" enter-from="-translate-x-full" enter-to="translate-x-0" leave="transition ease-in-out duration-300 transform" leave-from="translate-x-0" leave-to="-translate-x-full">
            <DialogPanel class="flex flex-col max-w-xs bg-indigo-700 flex-1 w-full pt-5 pb-4 relative">
              <TransitionChild as="template" enter="ease-in-out duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="ease-in-out duration-300" leave-from="opacity-100" leave-to="opacity-0">
                <div class="-mr-14 p-1 top-1 right-0 absolute">
                  <button type="button" class="rounded-full flex h-12 w-12 items-center justify-center focus:outline-none focus:ring-white focus:ring-2" @click="mobileMenuOpen = false">
                    <XMarkIcon class="h-6 text-white w-6" aria-hidden="true" />
                    <span class="sr-only">Close sidebar</span>
                  </button>
                </div>
              </TransitionChild>
              <div class="flex flex-shrink-0 px-4 items-center">
                <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=white" alt="Your Company">
              </div>
              <div class="flex-1 h-0 mt-5 px-2 overflow-y-auto">
                <nav class="flex flex-col h-full">
                  <div class="space-y-1">
                    <a v-for="item in navigation" :key="item.name" :href="item.href" :class="[item.current ? 'bg-indigo-800 text-white' : 'text-indigo-100 hover:bg-indigo-800 hover:text-white', 'group py-2 px-3 rounded-md flex items-center text-sm font-medium']" :aria-current="item.current ? 'page' : undefined">
                      <component :is="item.icon" :class="[item.current ? 'text-white' : 'text-indigo-300 group-hover:text-white', 'mr-3 h-6 w-6']" aria-hidden="true" />
                      <span>{{ item.name }}</span>
                    </a>
                  </div>
                </nav>
              </div>
            </DialogPanel>
          </TransitionChild>
          <div class="flex-shrink-0 w-14" aria-hidden="true">
            <!-- Dummy element to force sidebar to shrink to fit close icon -->
          </div>
        </div>
      </Dialog>
    </TransitionRoot>

    <!-- Content area -->
    <div class="flex flex-col flex-1 overflow-hidden">
      <header class="w-full">
        <div class="bg-white border-b flex border-gray-200 flex-shrink-0 h-16 shadow-sm z-10 relative">
          <button type="button" class="border-r border-gray-200 px-4 text-gray-500 md:hidden focus:outline-none focus:ring-inset focus:ring-2 focus:ring-indigo-500" @click="mobileMenuOpen = true">
            <span class="sr-only">Open sidebar</span>
            <Bars3BottomLeftIcon class="h-6 w-6" aria-hidden="true" />
          </button>
          <div class="flex flex-1 px-4 justify-between sm:px-6">
            <div class="flex flex-1">
              <form class="flex w-full md:ml-0" action="#" method="GET">
                <label for="desktop-search-field" class="sr-only">Search all files</label>
                <label for="mobile-search-field" class="sr-only">Search all files</label>
                <div class="w-full text-gray-400 relative focus-within:text-gray-600">
                  <div class="flex inset-y-0 left-0 pointer-events-none absolute items-center">
                    <MagnifyingGlassIcon class="flex-shrink-0 h-5 w-5" aria-hidden="true" />
                  </div>
                  <input id="mobile-search-field" name="mobile-search-field" class="border-transparent h-full text-base w-full py-2 pr-3 pl-8 placeholder-gray-500 text-gray-900 sm:hidden focus:border-transparent focus:outline-none focus:placeholder-gray-400 focus:ring-0" placeholder="Search" type="search">
                  <input id="desktop-search-field" name="desktop-search-field" class="border-transparent h-full text-base w-full py-2 pr-3 pl-8 placeholder-gray-500 text-gray-900 hidden sm:block focus:border-transparent focus:outline-none focus:placeholder-gray-400 focus:ring-0" placeholder="Search all files" type="search">
                </div>
              </form>
            </div>
            <div class="flex space-x-4 ml-2 items-center sm:space-x-6 sm:ml-6">
              <!-- Profile dropdown -->
              <Menu as="div" class="flex-shrink-0 relative hidden">
                <div>
                  <MenuButton class="bg-white rounded-full flex text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                    <span class="sr-only">Open user menu</span>
                    <img class="rounded-full h-8 w-8" src="https://images.unsplash.com/photo-1517365830460-955ce3ccd263?ixlib=rb-=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=8&w=256&h=256&q=80" alt="">
                  </MenuButton>
                </div>
                <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
                  <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 py-1 origin-top-right right-0 ring-1 ring-opacity-5 w-48 z-10 absolute focus:outline-none">
                    <MenuItem v-for="item in userNavigation" :key="item.name" v-slot="{ active }">
                      <a :href="item.href" :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']">{{ item.name }}</a>
                    </MenuItem>
                  </MenuItems>
                </transition>
              </Menu>

              <button type="button" class="rounded-full flex bg-indigo-600 text-white p-1 items-center justify-center hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" @click="showNewItemModal = true">
                <PlusIconOutline class="h-6 w-6" aria-hidden="true" />
                <span class="sr-only">Add file</span>
              </button>
              <BtnGreen v-show="needSave" :is-loading="isSaving" :disabled="isSaving" @click="saveData">
                Save data
              </BtnGreen>
              <a type="button" class="rounded-full flex bg-gray-400 text-white p-1 items-center justify-center hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2" href="">
                <ArrowPathIcon class="h-6 w-6" aria-hidden="true" />
                <span class="sr-only">Force Reload</span>
              </a>
            </div>
          </div>
        </div>
      </header>

      <!-- Main content -->
      <router-view :key="$route.fullPath" />
    </div>
    <Box3NewItemModal />
    <!-- global components -->
    <NotificationDefault />
  </div>
</template>
