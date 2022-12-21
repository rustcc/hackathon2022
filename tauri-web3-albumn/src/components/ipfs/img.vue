<script setup lang="ts">
import { PhotoIcon } from '@heroicons/vue/24/solid'

interface Props {
  src?: string
  hasModal?: Boolean
}
const {
  src,
  hasModal,
} = defineProps<Props>()

const theSrc = $computed(() => {
  if (!src) return ''
  return src.replace('ipfs://', 'https://nftstorage.link/ipfs/')
})

const isLoaded = $ref(false)
const isShowModal = $ref(false)
let isError = $ref(false)
setTimeout(() => {
  if (isLoaded) return
  isError = true
}, 5000)

</script>
<template>
  <div v-if="isError">
    <PhotoIcon v-bind="$attrs" />
  </div>
  <div v-else>
    <div v-if="!isLoaded" v-bind="$attrs" class="bg-black flex bg-opacity-75 justify-center items-center">
      <eos-icons:loading class="h-10 text-white w-10" />
    </div>
    <img v-else :src="theSrc" v-bind="$attrs" loading="lazy" :class="hasModal ? 'hover:cursor-pointer' : ''" @click="isShowModal=true" >
    <img :src="theSrc" class="h-0 w-0" @load="isLoaded = true" >
    <dialog-wide :show="isShowModal" v-if="hasModal" @close="isShowModal=false">
      <a :href="theSrc" target="_blank"><img :src="theSrc" class="w-auto" ></a>
    </dialog-wide>
  </div>
</template>
