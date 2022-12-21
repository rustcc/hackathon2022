<script setup lang="ts">
import VMdPreview from '@kangc/v-md-editor/lib/preview'

interface Props {
  text?: string
}
const {
  text,
} = defineProps<Props>()

</script>
<template>
  <v-md-preview :text="text" />
</template>

<style lang="stylus">
.github-markdown-body
  padding 0
</style>
