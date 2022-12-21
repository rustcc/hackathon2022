<script setup lang="ts">
interface Props {
  isLoading?: boolean
}
const {
  isLoading,
} = defineProps<Props>()
</script>
<template>
  <btn-default :isLoading="isLoading" class="bg-gray-900 text-white hover:bg-gray-700">
    <slot />
  </btn-default>
</template>
