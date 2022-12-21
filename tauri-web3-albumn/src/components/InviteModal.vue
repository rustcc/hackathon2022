<script setup lang="ts">
const { showInviteDialog, refLink, toggleInviteDialog, copy, copied } = $(inviteStore())

</script>
<template>
  <DialogDefault :show="showInviteDialog" @close="showInviteDialog = false">
    <div class="flex flex-col">
      <h2 class="font-bold text-center mb-2">
        Invite to Earn
      </h2>
      <p class="mb-2 p-2">
        Copy the link below and share to your friends, any of your friend mint NFT by your link, you get rewards!
      </p>
      <input id="refLink" type="text" name="refLink" autocomplete="refLink" class="rounded-none rounded-r-md border-gray-300 flex-1 w-full min-w-0 block sm:text-sm focus:border-indigo-500 focus:ring-indigo-500" :value="refLink" disabled>
      <div v-show="copied" class="text-center py-2 text-green-400">
        Copy successed
      </div>
      <btn-black class=" mx-auto mt-2 w-30" @click="copy(refLink)">
        Copy
      </btn-black>
    </div>
  </DialogDefault>
</template>
