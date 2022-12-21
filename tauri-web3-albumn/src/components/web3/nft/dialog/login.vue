<script setup lang="ts">
const props = defineProps({
  show: {
    type: Boolean,
    default: false,
  },
})
</script>
<template>
  <DialogDefault :show="props.show"/>
</template>
