<script setup lang="ts">
import VueBlockies from 'vue-blockies'

interface Props {
  address: string
}
const {
  address,
} = defineProps<Props>()

</script>
<template>
  <VueBlockies
    :seed="address"
    color="#999"
    bgcolor="#f60"
    :size="16"
    :scale="8"
    spot-color="#888"
  />
</template>
