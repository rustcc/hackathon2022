<script setup lang="ts">
const emit = defineEmits(['update:modelValue'])
interface Props {
  modelValue?: string
  text?: string
}
const {
  modelValue,
  text ='Loading, pls wait...'
} = defineProps<Props>()

</script>
<template>
  <div class="flex justify-center items-center" b-bind="$attrs">
    <eos-icons:loading class="h-10 mr-2 w-10" />
    <div>
      {{text}}
    </div>
  </div>
</template>