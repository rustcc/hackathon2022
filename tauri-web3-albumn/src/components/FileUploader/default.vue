<script setup lang="ts">
import axios from 'axios'
import DropZone from './components/DropZone.vue'
interface Props {
  title: string
  modelValue: string
}

const {
  title,
  modelValue,
} = defineProps<Props>()

let isLoading = $ref(false)
let error = $ref('')
const emit = defineEmits(['update:modelValue'])

const addFilesAndStartUpload = async(files) => {
  const file = files[0]
  const url = URL.createObjectURL(file)
  emit('update:modelValue', url)

  // do upload
  const formData = new FormData()
  // formData.append('title', file.name)
  formData.append('file', file)

  const baseUrl = 'https://api.pinata.cloud'
  const uploadUrl = `${baseUrl}/pinning/pinFileToIPFS`
  isLoading = true
  try {
    const rz = await axios.post(
      uploadUrl,
      // file,
      formData,
      {
        withCredentials: true,
        maxContentLength: -1, // this is needed to prevent axios from erroring out with large files
        maxBodyLength: -1,
        headers: {
        // 'Content-type': `multipart/form-data; boundary= ${formData._boundary}`,
          'Content-type': 'multipart/form-data;',
          // 'Content-Type': 'multipart/form-data',
          "pinata_api_key": PINATA_KEY,
          "pinata_secret_api_key": PINATA_SEC,
        // 'path': file.name,
        },
      })

    const ipfsHash = get(rz, 'data.IpfsHash', '')
    const gatewayUrl = `https://gateway.pinata.cloud/ipfs/${ipfsHash}`
    emit('update:modelValue', gatewayUrl)
  }
  catch (err) {
    error = err.toString()
  }
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
    <label for="cover-photo" class="font-medium text-sm pb-2 text-gray-700 block"> {{ title }} </label>
    <DropZone v-slot="{ dropZoneActive }" class="drop-area" @files-dropped="addFilesAndStartUpload">
      <div v-if="modelValue" class="relative">
        <div v-show="isLoading" class="bg-black flex h-60 w-full opacity-80 p-10 top-0 left-0 justify-center items-center absolute">
          <eos-icons:loading class="h-10 text-white w-10" />
        </div>
        <div v-show="error" class="bg-black flex h-60 w-full opacity-80 p-10 top-0 left-0 text-red-500 justify-center items-center absolute">
          {{ error }}
        </div>
        <ant-design:close-circle-filled v-show="!isLoading" class="h-8 text-white top-2 right-2 w-8 absolute hover:cursor-pointer" @click="resetStatus" />
        <img class="rounded-lg object-cover h-60 w-full" :src="modelValue" :alt="title">
      </div>
      <div v-else class="border-dashed rounded-md flex border-2 border-gray-300mt-1 px-6 pt-5 pb-6 justify-center">
        <div class="flex flex-col space-y-1 text-center py-10 justify-center items-center">
          <svg class="mx-auto h-12 text-gray-400 w-12" stroke="currentColor" fill="none" viewBox="0 0 48 48" aria-hidden="true">
            <path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
          <div class="h-10">
            <div v-if="dropZoneActive" class="text-gray-600">
              Drop Here
            </div>
            <div v-else>
              <div class="flex text-sm text-gray-600">
                <label for="file-upload" class="bg-white rounded-md cursor-pointer font-medium text-indigo-600 relative hover:text-indigo-500 focus-within:outline-none focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-indigo-500">
                  <span>Upload a file</span>
                  <input id="file-upload" name="file-upload" type="file" class="sr-only" @change="onInputChange">
                </label>
                <p class="pl-1">
                  or drag and drop
                </p>
              </div>
              <p class="text-xs text-gray-500">
                PNG, JPG, GIF up to 10MB
              </p>
            </div>
          </div>
        </div>
      </div>
    </DropZone>
  </div>
</template>
