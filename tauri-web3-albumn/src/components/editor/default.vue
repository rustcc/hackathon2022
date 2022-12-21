<script setup lang="ts">
const emit = defineEmits(['update:modelValue'])
interface Props {
  modelValue: string
  height?: Number
}
const {
  modelValue,
  height = '200px',
} = defineProps<Props>()

// https://code-farmer-i.github.io/vue-markdown-editor/zh/api.html#left-toolbar
const leftToolbar = $ref('clear h bold italic strikethrough quote hr link image code')

</script>
<template>
  <v-md-editor :model-value="modelValue" :leftToolbar="leftToolbar" mode="edit" :height="height"></v-md-editor>
</template>