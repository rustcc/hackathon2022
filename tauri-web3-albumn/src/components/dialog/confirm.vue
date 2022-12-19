<script setup lang="ts">
import { CheckIcon } from '@heroicons/vue/24/solid'
import { ExclamationCircleIcon } from '@heroicons/vue/24/outline'

interface Props {
  show: Boolean,
  status?: string,
  title: string
}
const {
  show,
  status = 'success',
  title,
} = defineProps<Props>()

const icon = $computed(() => status === 'success' ? CheckIcon : ExclamationCircleIcon)
</script>
<template>
  <dialog-default :show="show" @close="$emit('close')">
    <div>
      <div class="rounded-full flex mx-auto  h-12 w-12 items-center justify-center" :class="status === 'success' ? 'bg-green-100' : 'bg-red-100'">
        <component :is="icon" class="h-6 w-6" :class="status === 'success' ? 'text-green-600' : 'text-red-600'" aria-hidden="true" />
      </div>
      <div class="mt-3 text-center sm:mt-5">
        <DialogTitle as="h3" class="font-medium text-lg text-gray-900 leading-6"> {{title}}</DialogTitle>
        <div class="mt-2">
          <slot/>
        </div>
      </div>
    </div>
    <div class="mt-5 sm:mt-6">
      <slot name="footer" />
    </div>
  </dialog-default>
</template>
