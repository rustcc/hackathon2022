<script setup lang="ts">
import {
  Dialog,
  DialogPanel,
  TransitionChild,
  TransitionRoot,
} from '@headlessui/vue'
import { XMarkIcon } from '@heroicons/vue/24/outline'
const open = ref(true)
</script>
<template>
  <TransitionRoot as="template" :show="open">
    <Dialog as="div" class="z-10 relative" @close="open = false">
      <div class="inset-0 fixed" />
      <div class="inset-0 fixed overflow-hidden">
        <div class="inset-0 absolute overflow-hidden">
          <div class="flex max-w-full pl-10 inset-y-0 right-0 pointer-events-none fixed">
            <TransitionChild as="template" enter="transform transition ease-in-out duration-500 sm:duration-700" enter-from="translate-x-full" enter-to="translate-x-0" leave="transform transition ease-in-out duration-500 sm:duration-700" leave-from="translate-x-0" leave-to="translate-x-full">
              <DialogPanel class="max-w-md w-screen pointer-events-auto">
                <div class="bg-white flex flex-col h-full shadow-xl py-6 overflow-y-scroll">
                  <div class="px-4 sm:px-6">
                    <div class="flex items-start justify-between">
                      <DialogTitle class="font-medium text-lg text-gray-900">
                        Panel title
                      </DialogTitle>
                      <div class="flex h-7 ml-3 items-center">
                        <button type="button" class="bg-white rounded-md text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2" @click="open = false">
                          <span class="sr-only">Close panel</span>
                          <XMarkIcon class="h-6 w-6" aria-hidden="true" />
                        </button>
                      </div>
                    </div>
                  </div>
                  <div class="flex-1 mt-6 px-4 relative sm:px-6">
                    <!-- Replace with your content -->
                    <div class="px-4 inset-0 absolute sm:px-6">
                      <div class="border-dashed h-full border-2 border-gray-200" aria-hidden="true" />
                    </div>
                    <!-- /End replace -->
                  </div>
                </div>
              </DialogPanel>
            </TransitionChild>
          </div>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>
