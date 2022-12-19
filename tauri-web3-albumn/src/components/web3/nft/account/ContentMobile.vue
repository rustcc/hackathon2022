<script setup lang="ts">
const { user } = storeToRefs(web3Store())
</script>
<template>
  <div class="border-t border-sky-500 pt-4 pb-3">
    <div class="flex px-4 items-center">
      <div class="flex-shrink-0">
        <img class="rounded-full h-10 w-10" :src="user.imageUrl" alt="">
      </div>
      <div class="ml-3">
        <div class="font-medium text-base text-white">
          {{ user.name }}
        </div>
        <div class="font-medium text-sm text-sky-200">
          {{ user.email }}
        </div>
      </div>
      <button type="button" class="rounded-full ml-auto bg-sky-500 flex-shrink-0 p-1 text-sky-200 hover:text-white focus:outline-none focus:ring-white focus:ring-2 focus:ring-offset-2 focus:ring-offset-sky-500">
        <span class="sr-only">View notifications</span>
        <akar-icons:bell class="h-6 w-6" aria-hidden="true" />
      </button>
    </div>
    <div class="mt-3 px-2">
      <DisclosureButton v-for="item in userNavigation" :key="item.name" as="a" :href="item.href" class="rounded-md font-medium text-base py-2 px-3 text-sky-200 block hover:bg-sky-400 hover:text-white">
        {{ item.name }}
      </DisclosureButton>
    </div>
  </div>
</template>
