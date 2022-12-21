<script setup lang="ts">
const props = defineProps({
  show: {
    type: Boolean,
    default: false,
  },
})
</script>

<template>
  <TransitionRoot as="template" :show="props.show" @close="$emit('close')">
    <Dialog as="div" class="inset-0 z-10 fixed overflow-y-auto">
      <div class="flex min-h-screen text-center px-4 pt-4 pb-20 items-end justify-center sm:p-0 sm:block">
        <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="ease-in duration-200" leave-from="opacity-100" leave-to="opacity-0">
          <DialogOverlay class="bg-gray-500 bg-opacity-75 inset-0 transition-opacity fixed" />
        </TransitionChild>

        <!-- This element is to trick the browser into centering the modal contents. -->
        <span class="hidden sm:h-screen sm:inline-block sm:align-middle" aria-hidden="true">&#8203;</span>
        <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95" enter-to="opacity-100 translate-y-0 sm:scale-100" leave="ease-in duration-200" leave-from="opacity-100 translate-y-0 sm:scale-100" leave-to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95">
          <div class="bg-white rounded-lg shadow-xl text-left w-auto max-w-4/5 max-h-4/5 px-4 transform pt-5 pb-4 transition-all relative inline-block align-middle overflow-hidden">
            <slot />
          </div>
        </TransitionChild>
      </div>
    </Dialog>
  </TransitionRoot>
</template>
