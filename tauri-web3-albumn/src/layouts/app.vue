<script setup lang="ts">
import {
  Dialog,
  DialogPanel,
  TransitionChild,
  TransitionRoot,
} from '@headlessui/vue'
import {
  ChevronDownIcon,

  MagnifyingGlassIcon,
} from '@heroicons/vue/24/solid'
import {
  Bars3Icon,
  BellIcon,
  XMarkIcon,
} from '@heroicons/vue/24/outline'

const user = {
  name: 'Whitney Francis',
  email: 'whitney.francis@example.com',
  imageUrl:
    'https://images.unsplash.com/photo-1517365830460-955ce3ccd263?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
}
const navigation = [
  {
    name: 'Reporting',
    href: '#',
    children: [
      { name: 'Friends', href: '#' },
      { name: 'Follow', href: '#' },
      { name: 'Follower', href: '#' },
      { name: 'All', href: '#' },
    ],
  },
  // { name: 'Reporting', href: '#', children: [] },
  { name: 'Settings', href: '#', children: [] },
]
const sidebarNavigation = [
  // { name: 'Open', href: '#', icon: InboXMarkIcon, current: true },
  // { name: 'Archive', href: '#', icon: ArchiveBoxIconOutline, current: false },
  // { name: 'Customers', href: '#', icon: UserCircleIcon, current: false },
  // { name: 'Flagged', href: '#', icon: FlagIcon, current: false },
  // { name: 'Spam', href: '#', icon: BanIcon, current: false },
  // { name: 'Drafts', href: '#', icon: PencilAltIcon, current: false },
]
const userNavigation = [
  { name: 'Your Profile', href: '#' },
  { name: 'Sign out', href: '#' },
]

const open = ref(false)
</script>
<template>
  <!--
    This example requires updating your template:

    ```
    <html class="h-full bg-gray-100">
    <body class="h-full overflow-hidden">
    ```
  -->
  <div class="flex flex-col h-full">
    <!-- Top nav-->
    <header class="bg-white flex flex-shrink-0 h-16 relative items-center">
      <!-- Logo area -->
      <div class="inset-y-0 left-0 absolute lg:flex-shrink-0 lg:static">
        <a href="#" class="flex bg-cyan-400 h-16 w-16 items-center justify-center lg:w-20 focus:outline-none focus:ring-inset focus:ring-2 focus:ring-blue-600">
          <img class="h-8 w-auto" src="/logo.png">
        </a>
      </div>

      <!-- Picker area -->
      <div class="mx-auto lg:hidden">
        <div class="relative">
          <label for="inbox-select" class="sr-only">Choose inbox</label>
          <select id="inbox-select" class="bg-none rounded-md font-medium border-0 text-base pr-8 pl-3 text-gray-900 focus:ring-2 focus:ring-blue-600">
            <option value="/open">
              Open
            </option>
            <option value="/archived">
              Archived
            </option>
            <option value="/assigned">
              Assigned
            </option>
            <option value="/flagged">
              Flagged
            </option>
            <option value="/spam">
              Spam
            </option>
            <option value="/drafts">
              Drafts
            </option>
          </select>
          <div class="flex pr-2 inset-y-0 right-0 pointer-events-none absolute items-center justify-center">
            <ChevronDownIcon class="h-5 text-gray-500 w-5" aria-hidden="true" />
          </div>
        </div>
      </div>

      <!-- Menu button area -->
      <div class="flex pr-4 inset-y-0 right-0 absolute items-center sm:pr-6 lg:hidden">
        <!-- Mobile menu button -->
        <button type="button" class="rounded-md -mr-2 p-2 text-gray-400 inline-flex items-center justify-center hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-inset focus:ring-2 focus:ring-blue-600" @click="open = true">
          <span class="sr-only">Open main menu</span>
          <Bars3Icon class="h-6 w-6 block" aria-hidden="true" />
        </button>
      </div>

      <!-- Desktop nav area -->
      <div class="hidden lg:flex lg:flex-1 lg:min-w-0 lg:items-center lg:justify-between">
        <div class="flex-1 min-w-0">
          <div class="max-w-2xl text-gray-400 relative focus-within:text-gray-500">
            <label for="desktop-search" class="sr-only">Search</label>
            <input id="desktop-search" type="search" placeholder="Search" class="border-transparent w-full pl-12 placeholder-gray-500 block sm:text-sm focus:border-transparent focus:ring-0">
            <div class="flex pl-4 inset-y-0 left-0 pointer-events-none absolute items-center justify-center">
              <MagnifyingGlassIcon class="h-5 w-5" aria-hidden="true" />
            </div>
          </div>
        </div>
        <div class="flex space-x-10 flex-shrink-0 ml-10 pr-4 items-center">
          <nav aria-label="Global" class="flex space-x-10">
            <template v-for="item in navigation" :key="item.name">
              <Menu v-if="item.children.length" as="div" class="text-left relative">
                <MenuButton class="rounded-md flex font-medium text-sm text-gray-900 items-center focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-600">
                  <span>{{ item.name }}</span>
                  <ChevronDownIcon class="h-5 ml-1 text-gray-500 w-5" aria-hidden="true" />
                </MenuButton>

                <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
                  <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 origin-top-right right-0 ring-1 ring-opacity-5 w-40 z-30 absolute focus:outline-none">
                    <div class="py-1">
                      <MenuItem v-for="child in item.children" :key="child.name" v-slot="{ active }">
                        <a :href="child.href" :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']">
                          {{ child.name }}
                        </a>
                      </MenuItem>
                    </div>
                  </MenuItems>
                </transition>
              </Menu>
              <a v-else :href="item.href" class="font-medium text-sm text-gray-900">{{ item.name }}</a>
            </template>
          </nav>
          <div class="flex space-x-8 items-center">
            <span class="inline-flex">
              <a href="#" class="bg-white rounded-full -mx-1 p-1 text-gray-400 hover:text-gray-500">
                <span class="sr-only">View notifications</span>
                <BellIcon class="h-6 w-6" aria-hidden="true" />
              </a>
            </span>

            <Menu as="div" class="text-left relative inline-block">
              <MenuButton class="bg-white rounded-full flex text-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-600">
                <span class="sr-only">Open user menu</span>
                <img class="rounded-full h-8 w-8" :src="user.imageUrl" alt="">
              </MenuButton>

              <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
                <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 origin-top-right right-0 ring-1 ring-opacity-5 w-56 z-30 absolute focus:outline-none">
                  <div class="py-1">
                    <MenuItem v-slot="{ active }">
                      <a href="#" :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']"> Your Profile </a>
                    </MenuItem>
                    <MenuItem v-slot="{ active }">
                      <a href="#" :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']"> Sign Out </a>
                    </MenuItem>
                  </div>
                </MenuItems>
              </transition>
            </Menu>
          </div>
        </div>
      </div>

      <!-- Mobile menu, show/hide this `div` based on menu open/closed state -->
      <TransitionRoot as="template" :show="open">
        <Dialog as="div" class="z-40 relative lg:hidden" @close="open = false">
          <TransitionChild as="template" enter="transition-opacity ease-linear duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="transition-opacity ease-linear duration-300" leave-from="opacity-100" leave-to="opacity-0">
            <div class="hidden sm:bg-gray-600 sm:bg-opacity-75 sm:inset-0 sm:block sm:fixed" />
          </TransitionChild>

          <div class="inset-0 z-40 fixed">
            <TransitionChild as="template" enter="transition ease-out duration-150 sm:ease-in-out sm:duration-300" enter-from="transform opacity-0 scale-110 sm:translate-x-full sm:scale-100 sm:opacity-100" enter-to="transform opacity-100 scale-100 sm:translate-x-0 sm:scale-100 sm:opacity-100" leave="transition ease-in duration-150 sm:ease-in-out sm:duration-300" leave-from="transform opacity-100 scale-100 sm:translate-x-0 sm:scale-100 sm:opacity-100" leave-to="transform opacity-0 scale-110 sm:translate-x-full sm:scale-100 sm:opacity-100">
              <DialogPanel class="bg-white h-full w-full inset-0 z-40 fixed sm:max-w-sm sm:left-auto sm:shadow-lg sm:w-full sm:inset-y-0 sm:right-0" aria-label="Global">
                <div class="flex h-16 px-4 items-center justify-between sm:px-6">
                  <a href="#">
                    <img class="h-8 w-auto block" src="https://tailwindui.com/img/logos/workflow-mark.svg?color=cyan&shade=400" alt="Workflow">
                  </a>
                  <button type="button" class="rounded-md -mr-2 p-2 text-gray-400 inline-flex items-center justify-center hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-inset focus:ring-2 focus:ring-blue-600" @click="open = false">
                    <span class="sr-only">Close main menu</span>
                    <XMarkIcon class="h-6 w-6 block" aria-hidden="true" />
                  </button>
                </div>
                <div class="mx-auto mt-2 max-w-8xl px-4 sm:px-6">
                  <div class="text-gray-400 relative focus-within:text-gray-500">
                    <label for="mobile-search" class="sr-only">Search all inboxes</label>
                    <input id="mobile-search" type="search" placeholder="Search all inboxes" class="rounded-md border-gray-300 w-full pl-10 placeholder-gray-500 block focus:border-blue-600 focus:ring-blue-600">
                    <div class="flex pl-3 inset-y-0 left-0 absolute items-center justify-center">
                      <MagnifyingGlassIcon class="h-5 w-5" aria-hidden="true" />
                    </div>
                  </div>
                </div>
                <div class="mx-auto max-w-8xl py-3 px-2 sm:px-4">
                  <template v-for="item in navigation" :key="item.name">
                    <a :href="item.href" class="rounded-md font-medium text-base py-2 px-3 text-gray-900 block hover:bg-gray-100">{{ item.name }}</a>
                    <a v-for="child in item.children" :key="child.name" :href="child.href" class="rounded-md font-medium text-base py-2 pr-3 pl-5 text-gray-500 block hover:bg-gray-100">{{ child.name }}</a>
                  </template>
                </div>
                <div class="border-t border-gray-200 pt-4 pb-3">
                  <div class="flex mx-auto max-w-8xl px-4 items-center sm:px-6">
                    <div class="flex-shrink-0">
                      <img class="rounded-full h-10 w-10" :src="user.imageUrl" alt="">
                    </div>
                    <div class="flex-1 ml-3 min-w-0">
                      <div class="font-medium text-base text-gray-800 truncate">
                        {{ user.name }}
                      </div>
                      <div class="font-medium text-sm text-gray-500 truncate">
                        {{ user.email }}
                      </div>
                    </div>
                    <a href="#" class="bg-white ml-auto flex-shrink-0 p-2 text-gray-400 hover:text-gray-500">
                      <span class="sr-only">View notifications</span>
                      <BellIcon class="h-6 w-6" aria-hidden="true" />
                    </a>
                  </div>
                  <div class="mx-auto space-y-1 mt-3 max-w-8xl px-2 sm:px-4">
                    <a v-for="item in userNavigation" :key="item.name" :href="item.href" class="rounded-md font-medium text-base py-2 px-3 text-gray-900 block hover:bg-gray-50">{{ item.name }}</a>
                  </div>
                </div>
              </DialogPanel>
            </TransitionChild>
          </div>
        </Dialog>
      </TransitionRoot>
    </header>

    <!-- Bottom section -->
    <div class="flex flex-1 min-h-0 overflow-hidden">
      <!-- Narrow sidebar-->
      <nav aria-label="Sidebar" class="hidden lg:bg-gray-800 lg:flex-shrink-0 lg:block lg:overflow-y-auto">
        <div class="flex flex-col space-y-3 p-3 w-20 relative">
          <a v-for="item in sidebarNavigation" :key="item.name" :href="item.href" :class="[item.current ? 'bg-gray-900 text-white' : 'text-gray-400 hover:bg-gray-700', 'flex-shrink-0 inline-flex items-center justify-center h-14 w-14 rounded-lg']">
            <span class="sr-only">{{ item.name }}</span>
            <component :is="item.icon" class="h-6 w-6" aria-hidden="true" />
          </a>
        </div>
      </nav>

      <!-- Main area -->
      <main class="border-t border-gray-200 flex-1 min-w-0 xl:flex">
        <section aria-labelledby="message-heading" class="flex flex-col h-full flex-1 min-w-0 overflow-hidden xl:order-last">
          <AppChatMessageItem />
        </section>

        <!-- Message list-->
        <aside class="hidden xl:order-first xl:flex-shrink-0 xl:block">
          <div class="border-r flex flex-col h-full bg-gray-100 border-gray-200 w-96 relative">
            <div class="flex-shrink-0">
              <AppChatFilter />
              <AppChatCategory />
            </div>
            <AppChatMessageList />
          </div>
        </aside>
      </main>
    </div>
  </div>
</template>
