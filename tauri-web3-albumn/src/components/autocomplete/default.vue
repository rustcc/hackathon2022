<script setup lang="ts">
import { CheckIcon, SelectorIcon } from '@heroicons/vue/24/solid'
import { XCircleIcon } from '@heroicons/vue/24/outline'
import {
  Combobox,
  ComboboxButton,
  ComboboxInput,
  ComboboxLabel,
  ComboboxOption,
  ComboboxOptions,
} from '@headlessui/vue'
const emit = defineEmits(['update:modelValue'])
interface Props {
  modelValue: Array
  options: Array
}
const {
  modelValue,
  options,
} = defineProps<Props>()

let currentSelect = $ref(modelValue)
watch($$(currentSelect), () => {
  emit('update:modelValue', currentSelect)
})

let query = $ref('')

const queryItem = $computed(() => {
  if (query === '') return null

  const isExistInCurrentSelect = currentSelect.filter(item => item.name.toLowerCase() === query)
  if (isExistInCurrentSelect.length > 0) return null

  const isExistInItems = options.filter(item => item.name.toLowerCase() === query)
  if (isExistInItems.length > 0) return null

  return { id: null, name: query }
})

const filteredItems = $computed(() =>
  query === ''
    ? options
    : options.filter(item =>
      item.name
        .toLowerCase()
        .replace(/\s+/g, '')
        .includes(query.toLowerCase().replace(/\s+/g, '')),
    ),
)

const onInputChange = ($event) => {
  query = $event.target.value
}

const removeItem = (item) => {
  currentSelect = currentSelect.filter(_item => item.name !== _item.name)
}
</script>
<template>
  <Combobox v-model="currentSelect" multiple>
    <div class="mt-1 w-full relative">
      <div v-if="currentSelect.length > 0" class="divide-y border-y border-gray-200 my-4 text-sm grid grid-cols-1">
        <div v-for="item in currentSelect" :key="item.id" class="flex p-2 justify-between hover:(bg-gray-200) ">
          <span>{{ item.name }}</span>
          <span class="text-gray-400 hover:(cursor-pointer text-green-400) " @click="removeItem(item)">
            <XCircleIcon class="h-5  w-5" />
          </span>
        </div>
      </div>
      <div class="bg-white rounded-lg cursor-default border-1 text-left w-full relative overflow-hidden sm:text-sm focus:outline-none focus-visible:ring-white focus-visible:ring-2 focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-teal-300">
        <ComboboxInput class="border-none text-sm w-full py-2 pr-10 pl-3 text-gray-900 leading-5 focus:ring-0" :display-value="item => item.name" @change="onInputChange" />
        <ComboboxButton class="flex pr-2 inset-y-0 right-0 absolute items-center">
          <SelectorIcon class="h-5 text-gray-400 w-5" aria-hidden="true" />
        </ComboboxButton>
      </div>
      <TransitionRoot leave="transition ease-in duration-100" leave-from="opacity-100" leave-to="opacity-0" @after-leave="query = ''">
        <ComboboxOptions class="bg-white rounded-md shadow-lg ring-black mt-1 text-base w-full max-h-60 py-1 ring-1 ring-opacity-5 absolute overflow-auto sm:text-sm focus:outline-none">
          <!-- <div v-if="filteredItems.length === 0 && query !== ''" class="cursor-default py-2 px-4 text-gray-700 relative select-none">
            Nothing found.
          </div> -->
          <ComboboxOption v-if="queryItem" :value="queryItem" class="cursor-pointer py-2 px-4 text-gray-700 relative select-none">
            Create "{{ query }}"
          </ComboboxOption>
          <ComboboxOption v-for="item in filteredItems" :key="item.id" v-slot="{ selected, active }" as="template" :value="item" :disabled="item.unavailable">
            <li class="cursor-pointer py-2 pr-4 pl-10 relative select-none" :class="{
                'bg-teal-600 text-white': active,
                'text-gray-900': !active,
              }">
              <span class="block truncate" :class="{ 'font-medium': selected, 'font-normal': !selected }">
                {{ item.name }}
              </span>
              <span v-if="selected" class="flex pl-3 inset-y-0 left-0 absolute items-center" :class="{ 'text-white': active, 'text-teal-600': !active }">
                <CheckIcon class="h-5 w-5" aria-hidden="true" />
              </span>
            </li>
          </ComboboxOption>
        </ComboboxOptions>
      </TransitionRoot>

    </div>
  </Combobox>
</template>
