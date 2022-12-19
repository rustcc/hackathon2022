<script setup lang="ts">
import { useSound } from '@vueuse/sound'
import PopDown from '~/assets/sound/pop-down.mp3'

const { play } = useSound(PopDown)
const playSound = () => {
  play()
}
</script>
<template>
  <button @click="playSound">
    <slot />
  </button>
</template>
