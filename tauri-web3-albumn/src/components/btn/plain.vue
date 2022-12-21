<script setup lang="ts">
interface Props {
  isLoading?: boolean
}
const {
  isLoading,
} = defineProps<Props>()
</script>
<template>
  <btn-default :isLoading="isLoading" class="bg-white border rounded-md font-medium border-gray-300 shadow-sm text-sm py-2 px-4 text-gray-700 hover:bg-gray-50 ">
    <slot />
  </btn-default>
</template>
