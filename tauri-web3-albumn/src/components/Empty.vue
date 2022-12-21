<template>
  <div class="text-center">
    <CircleStackIcon class="mx-auto h-12 text-gray-400 w-12" />
    <h3 class="font-medium mt-2 text-sm text-gray-900">
      No Data
    </h3>
    <p class="mt-1 text-sm text-gray-500">
      Get started by Click the Button below!
    </p>
    <div class="mt-6">
      <button @click="$emit('newBtnClick')" type="button" class="border border-transparent rounded-md font-medium bg-indigo-600 shadow-sm text-sm text-white py-2 px-4 inline-flex items-center hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
        <PlusIcon class="h-5 mr-2 -ml-1 w-5" aria-hidden="true" />
        <slot />
      </button>
    </div>
  </div>
</template>

<script setup type="ts">
import { PlusIcon } from '@heroicons/vue/20/solid'
import { CircleStackIcon } from '@heroicons/vue/24/outline'

const emit = defineEmits(['newBtnClick'])
</script>
