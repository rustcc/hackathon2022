<script setup lang="ts">
import { XCircleIcon } from '@heroicons/vue/24/solid'
const emit = defineEmits(['update:modelValue'])
interface Props {
  modelValue?: Object | String | ''
}
const {
  modelValue,
} = defineProps<Props>()

const content = $computed(() => {
  if (modelValue.data && modelValue.data.message)
    return modelValue.data.message

  if (modelValue.message)
    return modelValue.message

  return ''
})
</script>
<template>
  <div v-show="modelValue" class="rounded-md bg-red-50 p-4">
    <div class="flex">
      <div class="flex-shrink-0">
        <XCircleIcon class="h-5 text-red-400 w-5" aria-hidden="true" />
      </div>
      <div class="ml-3">
        <h3 class="font-medium text-sm text-red-800">
          {{ content }}
        </h3>
      </div>
    </div>
  </div>
</template>
