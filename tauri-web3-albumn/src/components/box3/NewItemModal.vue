<script setup lang="ts">
import {
  Dialog,
  DialogPanel,
  TransitionChild,
  TransitionRoot,
} from '@headlessui/vue'
import { MinusCircleIcon, XMarkIcon } from '@heroicons/vue/24/outline'
import { PlusCircleIcon } from '@heroicons/vue/24/solid'
// import { litHelper } from '~/helpers/litHelper'
const { addSuccess } = $(notificationStore())

// const litNodeClient = inject('litNodeClient')
const { storeJson } = $(useNFTStorage())
const { showNewItemModal, addBoxItem } = $(box3Store())
const { walletAddress, initContract, parseEther, getContractAddress } = $(web3AuthStore())

const defaultValMap = {
  name: '',
  basicPrice: '1', // TODO: add USDC support?
  maxSupply: '1000000000',
  inviteCommission: '1',
  description: 'Your photo description here',
}
// const image = $ref('ipfs://bafybeibpcniplxkhajbmai25hwpuemqbskr43qxmtak6cwsqzf4w55vzo4')
let image = $ref('')
let name = $ref(defaultValMap.name || '')
const basicPrice = $ref(defaultValMap.basicPrice || '')
const maxSupply = $ref(defaultValMap.maxSupply || '')
const inviteCommission = $ref(defaultValMap.inviteCommission || '')
const description = $ref(defaultValMap.description || '')
const contentToBeLocked = $ref(defaultValMap.contentToBeLocked || '')

const properties = $ref([
  { label: 'category', value: 'Category Name' },
  { label: 'tag1', value: 'Hacker' },
  { label: 'tag2', value: 'AirBnB' },
  { label: 'tag3', value: 'Work Together' },
  { label: 'tag4', value: 'Study Together' },
  { label: 'tag5', value: 'Buidl Together' },
])

const requiredETHToUnlock = $ref(0)

const addItem = () => {
  properties.push({ label: '', value: '' })
}

const removeItem = (index) => {
  if (properties.length <= 1) return
  properties.splice(index, 1)
}

let state = $ref('addThumbnailAndBasicInfor')
// state = 'addLockedContentAndCondition'

let isLoading = $ref(false)
// let error = $ref({ message: 'some error' })
const error = $ref('')
const doSubmit = async() => {
  if (isLoading) return
  isLoading = true
  state = 'uploadPhoto'

  const _basicPrice = parseEther(basicPrice)
  const _inviteCommission = inviteCommission * 100
  let _properties = {}
  properties.forEach(({ label, value }) => {
    if (!label || !value) return
    _properties[label] = value
  })

  _properties = {
    ..._properties,
    from: 'Box3',
    // basicPrice,
    // maxSupply,
    // gallery,
    // inviteCommission: _inviteCommission,
    // requiredETHToUnlock,
  }

  const metadata = {
    name,
    description,
    image,
    properties: _properties,
  }
  const cid = await storeJson(metadata)
  // TODO: store in an big array?
  isLoading = false
  showNewItemModal = false
  state = 'addThumbnailAndBasicInfor'
  await addBoxItem({
    ...metadata,
    cid,
  })
  addSuccess(`Upload success ipfs cid: ${cid}`)
  name = ''
  image = ''

  // const metadataCID = 'ipfs://bafkreidktss36q3fhxwof6lwpedtpjwk5ao6xkq54claptj7g54kozasyy'
  // console.log('====> metadataCID :', metadataCID, gallery)
  // let tokenId = ''
  // const contractWriter = await initContract('BuidlerProtocol', true)
  // const value = parseEther('0.01')
  // try {
  //   const tokenType = 'SBT.Dating'
  //   const tx = await contractWriter.addToken(tokenType, _basicPrice, _inviteCommission, maxSupply, metadataCID, { value })
  //   // const txUrl = getTxUrl(tx.hash)
  //   const rc = await tx.wait()
  //   const event = rc.events.find(event => event.event === 'AddToken')
  //   const rz = event.args
  //   tokenId = rz.tokenId.toString()
  // }
  // catch (err) {
  //   isLoading = false
  //   error = err
  //   return
  // }

  // state = 'addLockContentOnChain'
  // // add locked content cid
  // const chain = CHAIN_NAME
  // const { doEncryptedString } = await litHelper({ chain, litNodeClient })
  // const contractAddress = getContractAddress('BuidlerProtocol')
  // const myselfCondition = {
  //   contractAddress: '',
  //   standardContractType: '',
  //   chain,
  //   method: '',
  //   parameters: [
  //     ':userAddress',
  //   ],
  //   returnValueTest: {
  //     comparator: '=',
  //     value: walletAddress,
  //   },
  // }
  // const currentNFTCondition = {
  //   contractAddress,
  //   standardContractType: 'ERC1155',
  //   chain,
  //   method: 'balanceOf',
  //   parameters: [
  //     ':userAddress',
  //     tokenId,
  //   ],
  //   returnValueTest: {
  //     comparator: '>=',
  //     value: '1',
  //   },
  // }

  // // https://lit-share-modal-v3-playground.netlify.app/
  // let accessControlConditions = [
  //   myselfCondition,
  //   {
  //     operator: 'or',
  //   },
  //   currentNFTCondition,
  // ]
  // if (requiredETHToUnlock > 0) {
  //   const additionalConditions = [
  //     currentNFTCondition,
  //     {
  //       operator: 'and',
  //     },
  //     {
  //       chain,
  //       conditionType: 'evmBasic',
  //       contractAddress: '',
  //       standardContractType: '',
  //       method: 'eth_getBalance',
  //       parameters: [
  //         ':userAddress',
  //         'latest',
  //       ],
  //       returnValueTest: {
  //         comparator: '>=',
  //         value: parseEther(requiredETHToUnlock).toString(),
  //       },
  //     },
  //   ]
  //   accessControlConditions = [
  //     myselfCondition,
  //     {
  //       operator: 'or',
  //     },
  //     additionalConditions,
  //   ]
  // }
  // const {
  //   encryptedString,
  //   encryptedSymmetricKey,
  // } = await doEncryptedString(contentToBeLocked, accessControlConditions)
  // metadata.properties.content = {
  //   encryptedString,
  //   encryptedSymmetricKey,
  //   accessControlConditions,
  // }
  // const newMetadataCID = await storeJson(metadata)
  // const tx = await contractWriter.updateToken(tokenId, _basicPrice, _inviteCommission, maxSupply, newMetadataCID)
  // await tx.wait()

  // addSuccess('Submit success')
  // showNewItemModal = false
  // await updateProductList()
  // setTimeout(() => {
  //   isLoading = false
  //   state = 'addThumbnailAndBasicInfor'
  // }, 1000)
}
</script>
<template>
  <TransitionRoot as="template" :show="showNewItemModal">
    <Dialog as="div" class="z-10 relative" @close="showNewItemModal = false">
      <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="ease-in duration-200" leave-from="opacity-100" leave-to="opacity-0">
        <div class="bg-gray-500 bg-opacity-75 inset-0 transition-opacity hidden fixed md:block" />
      </TransitionChild>

      <div class="inset-0 z-10 fixed overflow-y-auto">
        <div class="flex min-h-full text-center items-stretch justify-center md:px-2 md:items-center lg:px-4">
          <TransitionChild as="template" enter="ease-out duration-300" enter-from="opacity-0 translate-y-4 md:translate-y-0 md:scale-95" enter-to="opacity-100 translate-y-0 md:scale-100" leave="ease-in duration-200" leave-from="opacity-100 translate-y-0 md:scale-100" leave-to="opacity-0 translate-y-4 md:translate-y-0 md:scale-95">
            <DialogPanel class="flex text-base text-left w-full transform transition md:my-8 md:max-w-2xl md:px-4 lg:max-w-4xl">
              <div class="bg-white flex w-full px-4 pt-14 pb-8 shadow-2xl relative items-center overflow-hidden sm:px-6 sm:pt-8 md:p-6 lg:p-8">
                <button type="button" class="top-2 right-2 text-gray-400 absolute hover:text-gray-500" @click="showNewItemModal = false">
                  <span class="sr-only">Close</span>
                  <XMarkIcon class="h-6 w-6" aria-hidden="true" />
                </button>

                <div v-if="state === 'addThumbnailAndBasicInfor'" class="mt-2 w-full grid gap-y-8 gap-x-6 grid-cols-1 items-start sm:grid-cols-12 lg:gap-x-8">
                  <div class="rounded-lg overflow-hidden aspect-w-2 aspect-h-3 sm:col-span-4 lg:col-span-5">
                    <FileUploaderBanner v-model="image" class="h-full" />
                  </div>
                  <div class="flex flex-col h-full sm:col-span-8 lg:col-span-7">
                    <h2 class="font-bold text-2xl text-gray-900">
                      <input id="name" v-model="name" type="text" name="name" autocomplete="name" placeholder="Your photo title" class="rounded-md border-gray-300 flex-1 w-full min-w-0 block sm:text-sm focus:border-indigo-500 focus:ring-indigo-500">
                    </h2>
                    <section aria-labelledby="information-heading" class="mt-2">
                      <h3 id="information-heading" class="sr-only">
                        description
                      </h3>
                      <p class="mt-2 text-2xl text-gray-900">
                        <EditorDefault v-model="description" height="414px" />
                      </p>
                    </section>
                  </div>
                  <div class="flex col-span-12 justify-between items-center">
                    <div class="text-gray-400">
                      Image, title, description
                    </div>
                    <btn-black class="py-3" @click="state = 'addTags'">
                      Next
                    </btn-black>
                  </div>
                </div>
                <div v-if="state === 'addTags'" class="mt-2 w-full grid gap-y-8 gap-x-6 grid-cols-1 items-start sm:grid-cols-12 lg:gap-x-8">
                  <section aria-labelledby="properties-heading" class="col-span-12">
                    <div v-for="(item, index) in properties" :key="index" class="flex pb-2">
                      <input v-model="item.label" type="text" class="rounded-md max-w-sm max-w-xs border-gray-300 shadow-sm mr-2 sm:text-sm focus:border-indigo-500 focus:ring-indigo-500" placeholder="property name">
                      <div class="flex w-full">
                        <input v-model="item.value" type="text" class="rounded-md border-gray-300 shadow-sm w-full block  sm:text-sm focus:border-indigo-500 focus:ring-indigo-500" placeholder="property value">
                        <div class="p-2" :class="properties.length > 1 ? 'cursor-pointer' : 'cursor-not-allowed text-gray-400'" @click="removeItem(index)">
                          <MinusCircleIcon class="h-6  w-6" />
                        </div>
                      </div>
                    </div>
                    <div class="flex justify-end items-center ">
                      <div class="cursor-pointer p-2" @click="addItem">
                        <PlusCircleIcon class="h-6 w-6" />
                      </div>
                    </div>
                  </section>
                  <div class="flex col-span-12  justify-between items-center">
                    <div class="text-gray-400">
                      Any custom stuff you want to add for your photo
                    </div>
                    <div>
                      <btn-plain v-if="!isLoading" class="mr-2 py-3" @click="state = 'addThumbnailAndBasicInfor'">
                        Prev
                      </btn-plain>
                      <btn-black class="py-3" :is-loading="isLoading" @click="doSubmit">
                        Submit
                      </btn-black>
                    </div>
                  </div>
                </div>
                <div v-if="state === 'addToken'" class="mt-2 w-full grid gap-y-8 gap-x-6 grid-cols-1 items-start sm:grid-cols-12 lg:gap-x-8">
                  <div class="flex col-span-12 justify-center">
                    <Loading v-if="isLoading" class="h-20" text="Creating your token on Chain" />
                    <Error v-model="error" />
                  </div>
                </div>
                <div v-if="state === 'uploadPhoto'" class="mt-2 w-full grid gap-y-8 gap-x-6 grid-cols-1 items-start sm:grid-cols-12 lg:gap-x-8">
                  <div class="flex col-span-12 justify-center">
                    <Loading v-if="isLoading" class="h-20" text="uploading your photo, pls wait..." />
                    <Error v-model="error" />
                  </div>
                </div>
                <div v-if="state === 'addLockContentOnChain'" class="mt-2 w-full grid gap-y-8 gap-x-6 grid-cols-1 items-start sm:grid-cols-12 lg:gap-x-8">
                  <div class="flex col-span-12 justify-center">
                    <Loading v-if="isLoading" class="h-20" text="Add your locked content on chain" />
                    <Error v-model="error" />
                  </div>
                </div>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>
