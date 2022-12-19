<script setup lang="ts">
import { CheckCircleIcon } from '@heroicons/vue/24/outline'
import { XMarkIcon } from '@heroicons/vue/24/solid'
const { items, removeItem } = $(notificationStore())
</script>

<template>
  <div aria-live="assertive" class="flex py-6 px-4 top-16 right-0 w-100 z-20 fixed items-end pointer-events-none sm:p-6 sm:items-start">
    <div class="flex flex-col space-y-4 w-full items-center sm:items-end">
      <TransitionGroup name="list" tag="ul">
        <li v-for="item in items" :key="item._id">
          <div class="bg-white rounded-lg shadow-lg  ring-black mb-2  ring-1 ring-opacity-5 w-80 z-50 pointer-events-auto overflow-hidden">
            <div class="p-4">
              <div class="flex items-start">
                <div class="flex-shrink-0">
                  <codicon:error v-if="item.type === 'error'" class="h-6 text-red-400 w-6" aria-hidden="true" />
                  <bx-error-circle v-if="item.type === 'warning'" class="h-6 text-orange-400 w-6" aria-hidden="true" />
                  <CheckCircleIcon v-if="item.type === 'success'" class="h-6 text-green-400 w-6" aria-hidden="true" />
                </div>
                <div class="flex-1 ml-3 pt-0.5 w-0">
                  <p class="font-medium text-sm text-gray-900">
                    {{ item.title }}
                  </p>
                </div>
                <div class="flex flex-shrink-0 ml-4">
                  <button type="button" class="bg-white rounded-md text-gray-400 inline-flex hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" @click="removeItem(item)">
                    <span class="sr-only">Close</span>
                    <XMarkIcon class="h-5 w-5" aria-hidden="true" />
                  </button>
                </div>
              </div>
            </div>
          </div>
        </li>
      </TransitionGroup>
    </div>
  </div>
</template>

<style lang="stylus">
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.list-leave-active {
  position: absolute;
}

</style>
