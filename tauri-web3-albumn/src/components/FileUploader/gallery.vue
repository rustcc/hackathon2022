<script setup lang="ts">
import draggable from 'vuedraggable'

import {
  PhotoIcon,
  PlusIcon,
} from '@heroicons/vue/24/outline'
import DropZone from './components/DropZone.vue'
import { useNFTStorage } from '~/composables/useNFTStorage'
interface Props {
  title: string
  modelValue: Array
}

const {
  title,
  modelValue,
} = defineProps<Props>()

const error = $ref('')
let currentPreviewSrc = $ref(modelValue[0])

const emit = defineEmits(['update:modelValue'])

const { checkExist, storeBlob } = useNFTStorage()

const items = $ref(modelValue.map((src, index) => {
  return {
    id: src + index,
    src,
  }
}))

watch($$(items), (newItems, oldItems) => {
  if (isEqual(newItems, oldItems)) return
  emit('update:modelValue', newItems.map(({ src }) => src))
})

const addItem = (src, index = null) => {
  const newModelValue = modelValue
  if (index === null) index = newModelValue.length
  newModelValue.splice(index, 0, src)
  items.splice(index, 0, {
    id: src + (index + 1),
    src,
  })
  emit('update:modelValue', newModelValue)
}

const removeItem = (src) => {
  const newModelValue = modelValue
  const index = newModelValue.findIndex(_src => _src === src)
  newModelValue.splice(index, 1)
  items.splice(index, 1)
  if (src === currentPreviewSrc) 
    currentPreviewSrc = newModelValue[index] ? newModelValue[index] : newModelValue[index-1]
  
  emit('update:modelValue', newModelValue)
}

const replaceItem = (src, newSrc) => {
  const newModelValue = modelValue
  const index = newModelValue.findIndex(_src => _src === src)
  newModelValue.splice(index, 1, newSrc)
  items.splice(index, 1, {
    id: newSrc + (index + 1),
    src: newSrc,
  })
  emit('update:modelValue', newModelValue)
}

const doUploadOneFile = async (file) => {
  const url = URL.createObjectURL(file)
  addItem(url)

  const isExist = await checkExist(file)
  if (isExist) {
    replaceItem(url, isExist.cid)
    return
  }
  const cid = await storeBlob(file)
  replaceItem(url, cid)
}
const addFilesAndStartUpload = async(files) => {
  const lens = files.length

  const url = URL.createObjectURL(files[lens - 1])
  currentPreviewSrc = url

  const jobs = []
  for (let i = 0; i < lens; i++) 
    jobs.push(doUploadOneFile(files[i]))
  
  await Promise.all(jobs)
  currentPreviewSrc = modelValue[modelValue.length - 1]
}

const onInputChange = (e) => {
  addFilesAndStartUpload(e.target.files)
  // reset so that selecting the same file again will still cause it to fire this change
  e.target.value = null
}

let drag = $ref(false)
const dragOptions = {
  animation: 200,
  group: 'description',
  disabled: false,
  ghostClass: 'ghost'
}

const compData = $computed(() => {
  return {
    tag: 'div',
    type: 'transition-group',
    name: !drag ? 'flip-list' : null,
  }
})

const onStart = (event) => {
  currentPreviewSrc = modelValue[event.oldIndex]
  drag = true
}

</script>
<template>
  <div>
    <label for="cover-photo" class="font-medium text-sm pb-2 text-gray-700 block"> {{ title }} </label>
    <DropZone v-slot="{ dropZoneActive }" class="drop-area" @files-dropped="addFilesAndStartUpload">
      <div v-if="currentPreviewSrc" class="rounded-lg h-60 relative overflow-hidden">
        <div v-show="currentPreviewSrc.startsWith('blob:')" class="bg-black flex h-60 w-full opacity-80 p-10 top-0 left-0 justify-center items-center absolute">
          <eos-icons:loading class="h-10 text-white w-10" />
        </div>
        <div v-show="error" class="bg-black flex h-60 w-full opacity-80 p-10 top-0 left-0 text-red-500 justify-center items-center absolute">
          {{ error }}
        </div>
        <IpfsImg class="object-cover h-60 w-full" :src="currentPreviewSrc" />
      </div>
      <div v-else class="border-dashed rounded-md flex border-2 border-gray-300mt-1 h-60 px-6 pt-5 pb-6 justify-center relative overflow-hidden">
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
          <input type="file" multiple class="rounded-md cursor-pointer h-full border-gray-300 w-full opacity-0 inset-0 absolute" @change="onInputChange">
        </label>
      </div>
    </DropZone>
    <div class="w-full pt-3">
      <draggable v-model="items" item-key="id" class="list-group grid gap-1 grid-cols-9" tag="transition-group" :component-data="compData" v-bind="dragOptions" @start="onStart" @end="drag = false">
        <template #item="{ element }">
          <div class="aspect-square relative group">
            <div class="rounded-md cursor-move h-full border-2 w-full relative overflow-hidden" :class="currentPreviewSrc === element.src ? 'border-green-400' : 'border-white'" @click="currentPreviewSrc = element.src">
              <ipfs-img class="h-full object-cover w-full" :src="element.src" />
              <div v-show="element.src.startsWith('blob:')" class="bg-black flex h-full w-full opacity-80 top-0 left-0 justify-center items-center absolute">
                <eos-icons:loading class="h-10 text-white w-10" />
              </div>
            </div>
            <div class="bg-white rounded-full h-6 -top-2 -right-2 text-gray-400 w-6 justify-center items-center hidden absolute hover:cursor-pointer group-hover:flex" @click="removeItem(element.src)">
              <ant-design:close-circle-filled />
            </div>
          </div>
        </template>
        <template #footer>
          <div key="uploader" class="border-dashed rounded-md flex h-full border-2 border-red-400 text-xl w-full text-red-400 aspect-square justify-center items-center relative overflow-hidden">
            <PlusIcon class="h-6 w-6" />
            <label for="file-uploader" class="bg-black flex font-medium h-full bg-opacity-75 text-sm text-white w-full opacity-0 inset-0 absolute items-center justify-center hover:opacity-100 focus-within:opacity-100">
              <span class="cursor-pointer">Upload</span>
              <input id="file-uploader" type="file" class="rounded-md cursor-pointer h-full border-gray-300 w-full opacity-0 inset-0 absolute" multiple @change="onInputChange">
            </label>
          </div>
        </template>
      </draggable>
    </div>
  </div>
</template>

<style lang="stylus" scoped>
.flip-list-move {
  transition: transform 0.5s;
}
.no-move {
  transition: transform 0s;
}
.ghost {
  opacity: 0.5;
  background: #c8ebfb;
}
.list-group {
  min-height: 20px;
}
.list-group-item {
  cursor: move;
}
.list-group-item i {
  cursor: pointer;
}
</style>
