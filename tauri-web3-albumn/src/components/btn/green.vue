<script setup lang="ts">
interface Props {
  isLoading?: boolean
}
const {
  isLoading,
} = defineProps<Props>()
</script>
<template>
  <btn-default :isLoading="isLoading" class="bg-green-400 text-white hover:bg-green-600">
    <slot />
  </btn-default>
</template>
