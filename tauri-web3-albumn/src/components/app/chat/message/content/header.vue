<script setup lang="ts">
import {
  ArchiveBoxIcon as ArchiveBoxIconSolid,
  ChevronDownIcon,
  ChevronUpIcon,
  // DotsVerticalIcon,
  // FolderDownloadIcon,
  PencilIcon,
  ChatBubbleBottomCenterIcon,
  UserPlusIcon,
} from '@heroicons/vue/24/solid'

const message = {
  subject: 'Re: New pricing for existing customers',
  sender: 'Virginia Abshire',
  lastSeen: '2021-01-28T19:24',
  avatar: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
  status: 'Open',
  items: [
    {
      id: 1,
      author: 'Joe Armstrong',
      date: 'Yesterday at 7:24am',
      datetime: '2021-01-28T19:24',
      body: '<p>Thanks so much! Can\'t wait to try it out.</p>',
    },
    {
      id: 2,
      author: 'Monica White',
      date: 'Wednesday at 4:35pm',
      datetime: '2021-01-27T16:35',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
        <p><strong style="font-weight: 600;">Monica White</strong><br/>Customer Service</p>
      `,
    },
    {
      id: 3,
      author: 'Joe Armstrong',
      date: 'Wednesday at 4:09pm',
      datetime: '2021-01-27T16:09',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
        <p>– Joe</p>
      `,
    },
    {
      id: 1,
      author: 'Joe Armstrong',
      date: 'Yesterday at 7:24am',
      datetime: '2021-01-28T19:24',
      body: '<p>Thanks so much! Can\'t wait to try it out.</p>',
    },
    {
      id: 2,
      author: 'Monica White',
      date: 'Wednesday at 4:35pm',
      datetime: '2021-01-27T16:35',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
        <p><strong style="font-weight: 600;">Monica White</strong><br/>Customer Service</p>
      `,
    },
    {
      id: 3,
      author: 'Joe Armstrong',
      date: 'Wednesday at 4:09pm',
      datetime: '2021-01-27T16:09',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
        <p>– Joe</p>
      `,
    },
  ],
}
</script>
<template>
  <!-- Top section -->
  <div class="bg-white border-b border-gray-200 flex-shrink-0">
    <!-- Toolbar-->
    <div class="flex flex-col h-16 justify-center hidden">
      <div class="px-4 sm:px-6 lg:px-8">
        <div class="flex py-3 justify-between">
          <!-- Left buttons -->
          <div>
            <div class="rounded-md shadow-sm z-0 relative inline-flex sm:space-x-3 sm:shadow-none">
              <span class="inline-flex sm:shadow-sm">
                <button type="button" class="bg-white border rounded-l-md font-medium border-gray-300 text-sm py-2 px-4 text-gray-900 relative inline-flex items-center hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                  <ChatBubbleBottomCenterIcon class="h-5 mr-2.5 text-gray-400 w-5" aria-hidden="true" />
                  <span>Reply</span>
                </button>
                <button type="button" class="bg-white border font-medium -ml-px border-gray-300 text-sm py-2 px-4 text-gray-900 hidden relative items-center sm:inline-flex hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                  <PencilIcon class="h-5 mr-2.5 text-gray-400 w-5" aria-hidden="true" />
                  <span>Note</span>
                </button>
                <button type="button" class="bg-white border rounded-r-md font-medium -ml-px border-gray-300 text-sm py-2 px-4 text-gray-900 hidden relative items-center sm:inline-flex hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                  <UserPlusIcon class="h-5 mr-2.5 text-gray-400 w-5" aria-hidden="true" />
                  <span>Assign</span>
                </button>
              </span>

              <span class="space-x-3 hidden lg:flex">
                <button type="button" class="bg-white border rounded-md font-medium -ml-px border-gray-300 text-sm py-2 px-4 text-gray-900 hidden relative items-center sm:inline-flex hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                  <ArchiveBoxIconSolid class="h-5 mr-2.5 text-gray-400 w-5" aria-hidden="true" />
                  <span>Archive</span>
                </button>
                <button type="button" class="bg-white border rounded-md font-medium -ml-px border-gray-300 text-sm py-2 px-4 text-gray-900 hidden relative items-center sm:inline-flex hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                  <FolderDownloadIcon class="h-5 mr-2.5 text-gray-400 w-5" aria-hidden="true" />
                  <span>Move</span>
                </button>
              </span>

              <Menu as="div" class="-ml-px relative block sm:shadow-sm lg:hidden">
                <div>
                  <MenuButton class="bg-white border rounded-r-md font-medium border-gray-300 text-sm py-2 px-2 text-gray-900 relative inline-flex items-center sm:rounded-md sm:px-3 hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                    <span class="sr-only sm:hidden">More</span>
                    <span class="hidden sm:inline">More</span>
                    <ChevronDownIcon class="h-5 text-gray-400 w-5 sm:-mr-1 sm:ml-2" aria-hidden="true" />
                  </MenuButton>
                </div>

                <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
                  <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 origin-top-right right-0 ring-1 ring-opacity-5 w-36 absolute focus:outline-none">
                    <div class="py-1">
                      <MenuItem v-slot="{ active }">
                        <a href="#" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block sm:hidden px-4 py-2 text-sm']"> Note </a>
                      </MenuItem>
                      <MenuItem v-slot="{ active }">
                        <a href="#" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block sm:hidden px-4 py-2 text-sm']"> Assign </a>
                      </MenuItem>
                      <MenuItem v-slot="{ active }">
                        <a href="#" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']"> Archive </a>
                      </MenuItem>
                      <MenuItem v-slot="{ active }">
                        <a href="#" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'block px-4 py-2 text-sm']"> Move </a>
                      </MenuItem>
                    </div>
                  </MenuItems>
                </transition>
              </Menu>
            </div>
          </div>

          <!-- Right buttons -->
          <nav aria-label="Pagination">
            <span class="rounded-md shadow-sm z-0 relative inline-flex">
              <a href="#" class="bg-white border rounded-l-md font-medium border-gray-300 text-sm py-2 px-4 text-gray-500 relative inline-flex items-center hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                <span class="sr-only">Next</span>
                <ChevronUpIcon class="h-5 w-5" aria-hidden="true" />
              </a>
              <a href="#" class="bg-white border rounded-r-md font-medium -ml-px border-gray-300 text-sm py-2 px-4 text-gray-500 relative inline-flex items-center hover:bg-gray-50 focus:outline-none focus:border-blue-600 focus:ring-1 focus:ring-blue-600 focus:z-10">
                <span class="sr-only">Previous</span>
                <ChevronDownIcon class="h-5 w-5" aria-hidden="true" />
              </a>
            </span>
          </nav>
        </div>
      </div>
    </div>
    <!-- Message header -->

    <div class="bg-white shadow pt-5 pb-6">
      <div class="px-4 sm:flex sm:px-6 sm:justify-between sm:items-start lg:px-8">
        <div class="flex-shrink-0 pr-2">
          <img class="rounded-full h-12 w-12" :src="message.avatar" alt="">
        </div>
        <div class="sm:flex-1 sm:w-0">
          <h1 id="message-heading" class="font-medium text-lg text-gray-900">
            {{ message.sender }}
          </h1>
          <p class="mt-1 text-sm text-gray-500 truncate">
            last seen {{ message.lastSeen }}
          </p>
        </div>

        <div class="flex mt-4 items-center justify-between sm:flex-shrink-0 sm:mt-0 sm:ml-6 sm:justify-start">
          <span class="rounded-full font-medium bg-cyan-100 text-sm py-0.5 px-3 text-cyan-800 inline-flex items-center">
            {{ message.status }}
          </span>
          <Menu as="div" class="text-left ml-3 relative inline-block">
            <div>
              <MenuButton class="bg-white rounded-full flex -my-2 p-2 text-gray-400 items-center hover:text-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-600">
                <span class="sr-only">Open options</span>
                <DotsVerticalIcon class="h-5 w-5" aria-hidden="true" />
              </MenuButton>
            </div>

            <transition enter-active-class="transition ease-out duration-100" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
              <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 origin-top-right right-0 ring-1 ring-opacity-5 w-56 absolute focus:outline-none">
                <div class="py-1">
                  <MenuItem v-slot="{ active }">
                    <button type="button" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'w-full flex justify-between px-4 py-2 text-sm']">
                      <span>Copy email address</span>
                    </button>
                  </MenuItem>
                  <MenuItem v-slot="{ active }">
                    <a href="#" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'flex justify-between px-4 py-2 text-sm']">
                      <span>Previous conversations</span>
                    </a>
                  </MenuItem>
                  <MenuItem v-slot="{ active }">
                    <a href="#" :class="[active ? 'bg-gray-100 text-gray-900' : 'text-gray-700', 'flex justify-between px-4 py-2 text-sm']">
                      <span>View original</span>
                    </a>
                  </MenuItem>
                </div>
              </MenuItems>
            </transition>
          </Menu>
        </div>
      </div>
    </div>
  </div>
</template>
