<script setup lang="ts">
import axios from 'axios'
import { litHelper } from '~/helpers/litHelper'
const litNodeClient = inject('litNodeClient')
const emit = defineEmits(['close'])

const title = $ref(`the blog title${Date.now()}`)
const excerpt = $ref('the content intro here, this is a great blog, you should mint the nft and read it! Aliquet nec orci mattis amet quisque ullamcorper neque, nibh sem. At arcu, sit dui mi, nibh dui, diam eget aliquam. Quisque id at vitae feugiat egestas ac. Diam nulla orci at in viverra scelerisque eget. Eleifend egestas fringilla sapien.')
// const coverImg = $ref('')
const content = $ref(`<p>Faucibus commodo massa rhoncus, volutpat. <strong>Dignissim</strong> sed <strong>eget risus enim</strong>. Mattis mauris semper sed amet vitae sed turpis id. Id dolor praesent donec est. Odio penatibus risus viverra tellus varius sit neque erat velit. Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. <a href="#">Mattis mauris semper</a> sed amet vitae sed turpis id.</p>
        <ul role="list">
          <li>Quis elit egestas venenatis mattis dignissim.</li>
          <li>Cras cras lobortis vitae vivamus ultricies facilisis tempus.</li>
          <li>Orci in sit morbi dignissim metus diam arcu pretium.</li>
        </ul>
        <p>Quis semper vulputate aliquam venenatis egestas sagittis quisque orci. Donec commodo sit viverra aliquam porttitor ultrices gravida eu. Tincidunt leo, elementum mattis elementum ut nisl, justo, amet, mattis. Nunc purus, diam commodo tincidunt turpis. Amet, duis sed elit interdum dignissim.</p>
        <h2>From beginner to expert in 30 days</h2>
        <p>Id orci tellus laoreet id ac. Dolor, aenean leo, ac etiam consequat in. Convallis arcu ipsum urna nibh. Pharetra, euismod vitae interdum mauris enim, consequat vulputate nibh. Maecenas pellentesque id sed tellus mauris, ultrices mauris. Tincidunt enim cursus ridiculus mi. Pellentesque nam sed nullam sed diam turpis ipsum eu a sed convallis diam.</p>
        <blockquote>
          <p>Sagittis scelerisque nulla cursus in enim consectetur quam. Dictum urna sed consectetur neque tristique pellentesque. Blandit amet, sed aenean erat arcu morbi.</p>
        </blockquote>
        <p>Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id. Id dolor praesent donec est. Odio penatibus risus viverra tellus varius sit neque erat velit.</p>
        <h2>Everything you need to get up and running</h2>
        <p>Purus morbi dignissim senectus mattis <a href="#">adipiscing</a>. Amet, massa quam varius orci dapibus volutpat cras. In amet eu ridiculus leo sodales cursus tristique. Tincidunt sed tempus ut viverra ridiculus non molestie. Gravida quis fringilla amet eget dui tempor dignissim. Facilisis auctor venenatis varius nunc, congue erat ac. Cras fermentum convallis quam.</p>
        <p>Faucibus commodo massa rhoncus, volutpat. Dignissim sed eget risus enim. Mattis mauris semper sed amet vitae sed turpis id. Id dolor praesent donec est. Odio penatibus risus viverra tellus varius sit neque erat velit.</p>`)

const { addSuccess } = $(notificationStore())
const { walletAddress } = $(web3Store())
const baseUrl = 'https://api.pinata.cloud'
const coverImg = $ref('')
const isGating = $ref(true)
const nftContractAddress = $ref('0x17f6bdf57384fd9f24f1d9a4681c3a9dc839d79e')
// const nftContractAddress = $ref('0x88b48f654c30e99bc2e4a1559b4dcf1ad93fa656')
let isLoading = $ref(false)

const createPost = async() => {
  isLoading = true
  let theContent = content
  if (isGating) {
    const chain = 'rinkeby'
    const { doEncryptedString } = await litHelper({ chain, walletAddress, litNodeClient })

    const {
      encryptedString,
      encryptedSymmetricKey,
    } = await doEncryptedString(content, nftContractAddress)

    theContent = {
      encryptedString,
      encryptedSymmetricKey,
    }
    // const { decryptedString } = await doDecryptString(encryptedSymmetricKey, encryptedString, nftContractAddress)
  }

  const requestBody = {
    pinataContent: {
      coverImg,
      title,
      excerpt,
      content: theContent,
      isGating,
      nftContractAddress,
    },
    pinataMetadata: {
      keyvalues: {
        type: 'blog',
      },
    },
  }
  const rz = await axios.post(
    `${baseUrl}/pinning/pinJSONToIPFS`,
    requestBody,
    {
      withCredentials: true,
      headers: {
        pinata_api_key: PINATA_KEY,
        pinata_secret_api_key: PINATA_SEC,
      },
    })

  if (rz.data.IpfsHash) {
    isLoading = false
    emit('close')
    addSuccess('add success')
  }
}

</script>
<template>
  <DialogWide @close="$emit('close')">
    <div v-if="isLoading" class="flex h-50 w-full justify-center items-center">
      <eos-icons:loading class="h-10 w-10" />
    </div>
    <div v-else class="divide-y space-y-8 divide-gray-200 p-5">
      <div class="divide-y space-y-8 divide-gray-200">
        <div>
          <div class="mt-6 grid gap-y-6 gap-x-4 grid-cols-1 sm:grid-cols-6">
            <Web3NftFileUploaderDefault v-model="coverImg" title="Cover image" class="sm:col-span-6" />
            <div class="sm:col-span-6">
              <label for="title" class="font-medium text-sm text-gray-700 block"> Title </label>
              <div class="mt-1">
                <input id="title" v-model="title" type="text" name="title" autocomplete="title" class="rounded-md border-gray-300 shadow-sm w-full block sm:text-sm focus:border-indigo-500 focus:ring-indigo-500">
              </div>
            </div>
            <div class="sm:col-span-6">
              <label for="excerpt" class="font-medium text-sm text-gray-700 block"> Intro </label>
              <div class="mt-1">
                <textarea id="excerpt" v-model="excerpt" name="excerpt" rows="2" class="border rounded-md border-gray-300 shadow-sm w-full p-4 block sm:text-sm focus:border-indigo-500 focus:ring-indigo-500" />
              </div>
            </div>
            <div class="sm:col-span-6">
              <label for="content" class="font-medium text-sm text-gray-700 block"> Content </label>
              <div class="mt-1">
                <textarea id="content" v-model="content" name="content" rows="6" class="border rounded-md border-gray-300 shadow-sm w-full p-4 block sm:text-sm focus:border-indigo-500 focus:ring-indigo-500" />
              </div>
            </div>
            <div class="sm:col-span-6">
              <div class="flex mt-1 justify-end items-center">
                <label for="content" class="font-medium text-sm mr-2 text-gray-700 block"> NFT Gating </label>
                <SwitchDefault id="nft-gating" v-model="isGating" />
              </div>
            </div>
            <div v-if="isGating" class="sm:col-span-6">
              <div class="flex mt-1 justify-end items-center">
                <label for="nftContractAddress" class="font-medium text-sm mr-2 text-gray-700 block"> NFT Contract Address </label>
                <input id="nftContractAddress" v-model="nftContractAddress" type="text" name="nftContractAddress" class="rounded-md border-gray-300 flex-1 shadow-sm block sm:text-sm focus:border-indigo-500 focus:ring-indigo-500" placeholder="Your gating ERC721 NFT contract address">
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="pt-5">
        <div class="flex justify-end">
          <button type="button" class="bg-white border rounded-md font-medium border-gray-300 shadow-sm text-sm py-2 px-4 text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" @click="$emit('close')">
            Cancel
          </button>
          <button class="border border-transparent rounded-md font-medium bg-indigo-600 shadow-sm text-sm text-white ml-3 py-2 px-4 inline-flex justify-center hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" @click="createPost">
            Save
          </button>
        </div>
      </div>
    </div>
  </DialogWide>
</template>
