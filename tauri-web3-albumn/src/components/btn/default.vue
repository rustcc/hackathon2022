<script setup lang="ts">
import { useSound } from '@vueuse/sound'
import PopDown from '~/assets/sound/pop-down.mp3'
interface Props {
  isLoading?: boolean
}
const {
  isLoading,
} = defineProps<Props>()


const { play } = useSound(PopDown)
const playSound = () => {
  play()
}
</script>
<template>
  <button @click="playSound" type="button" class="border rounded-md font-medium shadow-sm text-base  py-2 px-4 inline-flex justify-center items-center sm:text-sm  focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500">
    <eos-icons:loading v-show="isLoading" class="mr-2" />
    <slot />
  </button>
</template>
