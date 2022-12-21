<script setup lang="ts">
import { Listbox, ListboxButton, ListboxLabel, ListboxOption, ListboxOptions } from '@headlessui/vue'
import { CalendarIcon, PaperClipIcon, TagIcon, UserCircleIcon } from '@heroicons/vue/24/solid'

const assignees = [
  { name: 'Unassigned', value: null },
  {
    name: 'Wade Cooper',
    value: 'wade-cooper',
    avatar:
      'https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
  },
  // More items...
]
const labels = [
  { name: 'Unlabelled', value: null },
  { name: 'Engineering', value: 'engineering' },
  // More items...
]
const dueDates = [
  { name: 'No due date', value: null },
  { name: 'Today', value: 'today' },
  // More items...
]

const assigned = ref(assignees[0])
const labelled = ref(labels[0])
const dated = ref(dueDates[0])
</script>
<template>
  <div class="bg-white border-b border-gray-200 flex-shrink-0">
    <form action="#" class="relative ">
      <div class="border  border-gray-300 shadow-sm overflow-hidden focus-within:border-indigo-500 focus-within:ring-1 focus-within:ring-indigo-500">
        <!-- <label for="title" class="sr-only">Title</label>
        <input id="title" type="text" name="title" class="font-medium border-0 text-lg w-full pt-2.5 placeholder-gray-500 block focus:ring-0" placeholder="Title"> -->
        <label for="description" class="sr-only">Description</label>
        <textarea id="description" rows="2" name="description" class="border-0 w-full p-2 placeholder-gray-500 block resize-none sm:text-sm focus:ring-0" placeholder="Write a message..." />

        <!-- Spacer element to match the height of the toolbar -->
        <div aria-hidden="true">
          <div class="py-2">
            <div class="h-9" />
          </div>
          <div class="h-px" />
          <!-- <div class="py-2">
            <div class="py-px">
              <div class="h-9" />
            </div>
          </div> -->
        </div>
      </div>

      <div class="inset-x-px bottom-0 absolute">
        <!-- Actions: These are just examples to demonstrate the concept, replace/wire these up however makes sense for your project. -->
        <div class="flex flex-nowrap space-x-2 py-2 px-2 justify-end hidden sm:px-3">
          <Listbox v-model="assigned" as="div" class="flex-shrink-0">
            <ListboxLabel class="sr-only">
              Assign
            </ListboxLabel>
            <div class="relative">
              <ListboxButton class="rounded-full font-medium bg-gray-50 text-sm py-2 px-2 text-gray-500 relative inline-flex items-center whitespace-nowrap sm:px-3 hover:bg-gray-100">
                <UserCircleIcon v-if="assigned.value === null" class="flex-shrink-0 h-5 text-gray-300 w-5 sm:-ml-1" aria-hidden="true" />

                <img v-else :src="assigned.avatar" alt="" class="rounded-full flex-shrink-0 h-5 w-5">

                <span :class="[assigned.value === null ? '' : 'text-gray-900', 'hidden truncate sm:ml-2 sm:block']">{{ assigned.value === null ? 'Assign' : assigned.name }}</span>
              </ListboxButton>

              <transition leave-active-class="transition ease-in duration-100" leave-from-class="opacity-100" leave-to-class="opacity-0">
                <ListboxOptions class="bg-white rounded-lg shadow ring-black mt-1 text-base max-h-56 py-3 right-0 ring-1 ring-opacity-5 w-52 z-10 absolute overflow-auto sm:text-sm focus:outline-none">
                  <ListboxOption v-for="assignee in assignees" :key="assignee.value" v-slot="{ active }" as="template" :value="assignee">
                    <li :class="[active ? 'bg-gray-100' : 'bg-white', 'cursor-default select-none relative py-2 px-3']">
                      <div class="flex items-center">
                        <img v-if="assignee.avatar" :src="assignee.avatar" alt="" class="rounded-full flex-shrink-0 h-5 w-5">
                        <UserCircleIcon v-else class="flex-shrink-0 h-5 text-gray-400 w-5" aria-hidden="true" />
                        <span class="font-medium ml-3 block truncate">
                          {{ assignee.name }}
                        </span>
                      </div>
                    </li>
                  </ListboxOption>
                </ListboxOptions>
              </transition>
            </div>
          </Listbox>

          <Listbox v-model="labelled" as="div" class="flex-shrink-0">
            <ListboxLabel class="sr-only">
              Add a label
            </ListboxLabel>
            <div class="relative">
              <ListboxButton class="rounded-full font-medium bg-gray-50 text-sm py-2 px-2 text-gray-500 relative inline-flex items-center whitespace-nowrap sm:px-3 hover:bg-gray-100">
                <TagIcon :class="[labelled.value === null ? 'text-gray-300' : 'text-gray-500', 'flex-shrink-0 h-5 w-5 sm:-ml-1']" aria-hidden="true" />
                <span :class="[labelled.value === null ? '' : 'text-gray-900', 'hidden truncate sm:ml-2 sm:block']">{{ labelled.value === null ? 'Label' : labelled.name }}</span>
              </ListboxButton>

              <transition leave-active-class="transition ease-in duration-100" leave-from-class="opacity-100" leave-to-class="opacity-0">
                <ListboxOptions class="bg-white rounded-lg shadow ring-black mt-1 text-base max-h-56 py-3 right-0 ring-1 ring-opacity-5 w-52 z-10 absolute overflow-auto sm:text-sm focus:outline-none">
                  <ListboxOption v-for="label in labels" :key="label.value" v-slot="{ active }" as="template" :value="label">
                    <li :class="[active ? 'bg-gray-100' : 'bg-white', 'cursor-default select-none relative py-2 px-3']">
                      <div class="flex items-center">
                        <span class="font-medium block truncate">
                          {{ label.name }}
                        </span>
                      </div>
                    </li>
                  </ListboxOption>
                </ListboxOptions>
              </transition>
            </div>
          </Listbox>

          <Listbox v-model="dated" as="div" class="flex-shrink-0">
            <ListboxLabel class="sr-only">
              Add a due date
            </ListboxLabel>
            <div class="relative">
              <ListboxButton class="rounded-full font-medium bg-gray-50 text-sm py-2 px-2 text-gray-500 relative inline-flex items-center whitespace-nowrap sm:px-3 hover:bg-gray-100">
                <CalendarIcon :class="[dated.value === null ? 'text-gray-300' : 'text-gray-500', 'flex-shrink-0 h-5 w-5 sm:-ml-1']" aria-hidden="true" />
                <span :class="[dated.value === null ? '' : 'text-gray-900', 'hidden truncate sm:ml-2 sm:block']">{{ dated.value === null ? 'Due date' : dated.name }}</span>
              </ListboxButton>

              <transition leave-active-class="transition ease-in duration-100" leave-from-class="opacity-100" leave-to-class="opacity-0">
                <ListboxOptions class="bg-white rounded-lg shadow ring-black mt-1 text-base max-h-56 py-3 right-0 ring-1 ring-opacity-5 w-52 z-10 absolute overflow-auto sm:text-sm focus:outline-none">
                  <ListboxOption v-for="dueDate in dueDates" :key="dueDate.value" v-slot="{ active }" as="template" :value="dueDate">
                    <li :class="[active ? 'bg-gray-100' : 'bg-white', 'cursor-default select-none relative py-2 px-3']">
                      <div class="flex items-center">
                        <span class="font-medium block truncate">
                          {{ dueDate.name }}
                        </span>
                      </div>
                    </li>
                  </ListboxOption>
                </ListboxOptions>
              </transition>
            </div>
          </Listbox>
        </div>
        <div class="border-t flex space-x-3 border-gray-200 py-2 px-2 justify-between items-center sm:px-3">
          <div class="flex">
            <button type="button" class="rounded-full -my-2 text-left -ml-2 py-2 px-3 text-gray-400 inline-flex items-center group">
              <PaperClipIcon class="h-5 mr-2 -ml-1 w-5 group-hover:text-gray-500" aria-hidden="true" />
              <span class="text-sm text-gray-500 italic group-hover:text-gray-600">Attach a file</span>
            </button>
          </div>
          <div class="flex-shrink-0">
            <button type="submit" class="border border-transparent rounded-md font-medium bg-sky-400 shadow-sm text-sm text-white py-2 px-4 inline-flex items-center hover:bg-sky-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-600">
              Reply
            </button>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>
