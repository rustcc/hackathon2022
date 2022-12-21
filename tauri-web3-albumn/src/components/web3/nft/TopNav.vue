<script setup lang="ts">
import { isValidateAddress } from '~/helpers/web3'
const navigation = [
  { name: 'Web3Home', href: '/' },
  // { name: 'HackerNFT', href: '/HackerNFT' },
  // { name: 'ArtistNFT', href: '/ArtistNFT' },
  // { name: 'About', href: '/about' },
]

// new code here
const $web3 = web3Store()

const router = useRouter()
const address = $ref('')
let isShowError = $ref(false)
const go = () => {
  if (!isValidateAddress(address)) {
    // show error alert dialog
    isShowError = true
  }
  // router.push(`/${address}`)
  router.push('/0xC6E58fb4aFFB6aB8A392b7CC23CD3feF74517F6C')
}

</script>

<template>
  <Disclosure v-slot="{ open }" as="nav" class="bg-sky-500 top-0 z-10 sticky" aria-label="Global">
    <div class="mx-auto max-w-7xl px-2 2xl:px-0">
      <div class="flex h-16 justify-between">
        <!-- main nav -->
        <div class="flex px-2 items-center lg:px-0">
          <router-link to="/" class="flex flex-shrink-0 items-center">
            <img class="h-8 w-auto" src="/logo.png" alt="Workflow">
          </router-link>
          <div class="hidden lg:flex lg:space-x-4 lg:ml-8">
            <router-link v-for="item in navigation" :key="item.name" :to="item.href" class="rounded-md font-medium text-sm text-white py-2 px-3 hover:bg-sky-400">
              {{ item.name }}
            </router-link>
          </div>
        </div>
        <!-- search form -->
        <div class="flex flex-1 px-2 items-center justify-center lg:ml-6 lg:justify-end">
          <div class="max-w-lg w-full lg:max-w-xs">
            <label for="search" class="sr-only">Search</label>
            <div class="text-white relative focus-within:text-gray-400">
              <div class="flex pl-3 inset-y-0 left-0 absolute items-center pointer-events-none">
                <akar-icons:search class="flex-shrink-0 h-5 w-5" aria-hidden="true" />
              </div>
              <input id="search" v-model="address" name="search" class="border-transparent rounded-md bg-sky-400 placeholder-white text-base w-full py-2 pr-3 pl-10 leading-5 block sm:text-sm focus:bg-white focus:border-white focus:outline-none focus:placeholder-gray-400 focus:ring-0 focus:text-gray-900" placeholder="Search" type="search" @keyup.enter.native="go">
            </div>
          </div>
          <DialogDefault :show="isShowError" @close="isShowError=false">
            {{ address }} address not validate
          </DialogDefault>
        </div>
        <div class="flex items-center lg:hidden">
          <DisclosureButton class="rounded-md p-2 text-sky-200 inline-flex items-center justify-center hover:bg-sky-400 hover:text-white focus:outline-none focus:ring-inset focus:ring-white focus:ring-2">
            <span class="sr-only">Open menu</span>
            <dashicons:menu-alt v-if="!open" class="h-6 w-6 block" aria-hidden="true" />
            <ep:close-bold v-else class="h-6 w-6 block" aria-hidden="true" />
          </DisclosureButton>
        </div>
        <Web3NftAccountContentPC v-if="$web3.walletAddress" />
        <Web3NftAccountLoginBtnPC v-else />
      </div>
    </div>

    <DialogDefault :show="$web3.error !== ''" @close="$web3.error = ''">
      <div>
        <div class="mt-3 text-center">
          <DialogTitle as="h3" class="font-medium text-lg text-gray-900 leading-6">
            Error
          </DialogTitle>
        </div>
      </div>
      <div class="my-10 text-center">
        {{ $web3.error }}
      </div>
      <div class="mt-5 sm:mt-6">
        <button type="button" class="border border-transparent rounded-md font-medium bg-gray-900 shadow-sm text-base text-white w-full py-2 px-4 inline-flex justify-center sm:text-sm hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500" @click="$web3.error = ''">
          Ok
        </button>
      </div>
    </DialogDefault>

    <DisclosurePanel class="lg:hidden">
      <div class="space-y-1 px-2 pt-2 pb-3">
        <DisclosureButton v-for="item in navigation" :key="item.name" as="a" :href="item.href" class="rounded-md font-medium text-base text-white py-2 px-3 block hover:bg-sky-400 hover:text-white">
          {{ item.name }}
        </DisclosureButton>
      </div>
      <Web3NftAccountContentMobile v-if="$web3.walletAddress" />
      <Web3NftAccountLoginBtnMobile v-else />
    </DisclosurePanel>
  </Disclosure>
</template>
