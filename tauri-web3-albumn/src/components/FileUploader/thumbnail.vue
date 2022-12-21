<script setup lang="ts">
import {
  PhotoIcon,
} from '@heroicons/vue/24/outline'
import DropZone from './components/DropZone.vue'
import { useNFTStorage } from '~/composables/useNFTStorage'
interface Props {
  title: string
  modelValue: string
}

const {
  title,
  modelValue,
} = defineProps<Props>()

const { checkExist, storeBlob } = useNFTStorage()
let isLoading = $ref(false)
let error = $ref('')
const emit = defineEmits(['update:modelValue'])

const addFilesAndStartUpload = async(files) => {
  const file = files[0]
  const url = URL.createObjectURL(file)
  emit('update:modelValue', url)

  isLoading = true
  const isExist = await checkExist(file)
  if (isExist) {
    emit('update:modelValue', isExist.cid)
    isLoading = false
    return
  }

  const cid = await storeBlob(file)
  emit('update:modelValue', cid)
  isLoading = false
}

async function onInputChange(e) {
  addFilesAndStartUpload(e.target.files)
  // reset so that selecting the same file again will still cause it to fire this change
  e.target.value = null
}

const resetStatus = () => {
  emit('update:modelValue', '')
  error = ''
  isLoading = false
}

</script>
<template>
  <div>
    <label for="cover-photo" class="font-medium text-sm mr-2 pb-2 text-gray-700 block"> {{ title }} </label>
    <div class="rounded-full h-60 w-60">
      <DropZone v-slot="{ dropZoneActive }" class="drop-area" @files-dropped="addFilesAndStartUpload">
        <div v-if="modelValue" class="relative">
          <div v-show="isLoading" class="bg-black rounded-full flex h-60 w-full opacity-80 p-10 top-0 left-0 justify-center items-center absolute">
            <eos-icons:loading class="h-10 text-white w-10" />
          </div>
          <div v-show="error" class="bg-black flex h-60 w-full opacity-80 p-10 top-0 left-0 text-red-500 justify-center items-center absolute">
            {{ error }}
          </div>
          <ipfs-img class="rounded-full object-cover h-60 w-60" :src="modelValue" :alt="title" />
          <ant-design:close-circle-filled v-show="!isLoading" class="h-8 top-2 right-2 text-gray-400 w-8 absolute hover:cursor-pointer" @click="resetStatus" />
        </div>
        <div v-else class="border-dashed rounded-full flex border-2 border-gray-300 h-60 mt-1 px-6 pt-5 pb-6 w-60 justify-center relative  overflow-hidden">
          <div class="flex flex-col space-y-1 text-center py-10 justify-center items-center">
            <PhotoIcon class="mx-auto h-12 text-gray-400 w-12" />
            <div class="h-10">
              <div v-if="dropZoneActive" class="text-gray-600">
                Drop to upload
              </div>
              <div v-else>
                <div class="text-center text-sm text-gray-600">
                  Drag and Drop
                </div>
                <p class="text-xs text-gray-500">
                  PNG, JPG, GIF up to 10MB
                </p>
              </div>
            </div>
          </div>

          <label for="file-uploader" class="bg-black flex font-medium h-full bg-opacity-75 text-sm text-white w-full opacity-0 inset-0 absolute items-center justify-center hover:opacity-100 focus-within:opacity-100">
            <span>Click to upload</span>
            <span class="sr-only">image</span>
            <input id="file-uploader" type="file" name="user-photo" class="rounded-md cursor-pointer h-full border-gray-300 w-full opacity-0 inset-0 absolute" @change="onInputChange">
          </label>
        </div>
      </DropZone>
    </div>
  </div>
</template>
