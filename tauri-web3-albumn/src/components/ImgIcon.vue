<script setup lang="ts">
import * as iconMap from "~/helpers/icon";
const props = defineProps({
  type: {
    type: String,
    default: 'pending'
  },
})
const { type } = toRefs(props)
const imgSrc = computed(() => type.value.includes('http') ? type.value : iconMap[type.value] ? iconMap[type.value] : '' )
</script>

<template>
  <img :src="imgSrc" :alt="imgSrc" class="flex-shrink-0" />
</template>
