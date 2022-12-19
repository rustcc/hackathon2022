<script setup lang="ts">
import {
  web3Accounts,
  web3Enable,
  web3FromAddress,
} from '@polkadot/extension-dapp'

import { Recurrer, Scheduler, oakConstants } from 'oak-js-library'

const emit = defineEmits(['update:modelValue'])
interface Props {
  address: string
}
const {
  address,
} = defineProps<Props>()

const doConnect = async() => {
  const allInjected = await web3Enable(import.meta.env.VITE_PROJECT_NAME)
  console.log('====> allInjected :', allInjected)
  const allAccounts = await web3Accounts()
  const account = allAccounts[0].address
  const injector = await web3FromAddress(account)
  return { account, injector }
}

let isShow = $ref(false)
let isLoading = $ref(false)
let error = $ref('')
const recurrences = $ref(5)
const amount = $ref(1000000000)
const doSubscribe = async() => {
  isLoading = true
  const { account, injector } = await doConnect()
  const providedID = account
  const scheduler = new Scheduler(oakConstants.OakChains.STUR)
  const recurrer = new Recurrer()
  const hourOfDay = 12 // noon UTC
  const dateOfMonth = 1 // first day of month
  // timestamps output is a 5-item array of unix timestamps
  const timestamps = recurrer.getMonthlyRecurringTimestampsByDate(
    Date.now(),
    recurrences,
    hourOfDay,
    dateOfMonth,
  )
  const senderAddress = account
  const receiverAddress = '13MhRhYPTphvQbbipGt1TTuAnP5m2FGkvUXSY2JwjhkgD3f3'
  const extrinsicHex = await scheduler.buildScheduleNativeTransferExtrinsic(
    senderAddress,
    providedID,
    timestamps,
    receiverAddress,
    amount,
    injector.signer,
  )
  console.log('====> extrinsicHex :', extrinsicHex)
  try {
    const tx = await scheduler.sendExtrinsic(extrinsicHex, (result) => {
      console.log('====> result :', result)
      // error = result
    })
    console.log('====> tx :', tx)
    error = `donate success! tx: ${tx}`
    // isShow = false
  }
  catch (err) {
    console.log('====> err :', err)
    error = err.toString()
  }
  isLoading = false
}

const resetDialog = () => {
  isShow = false
  // nextTick(() => {
  //   error = ''
  // })
}
</script>
<template>
  <div>
    <btn-black :is-loading="false" @click="isShow = true">
      Monthly Donate
    </btn-black>
    <DialogDefault :show="isShow" @close="resetDialog">
      <div v-if="isLoading" class="flex p-10 justify-center">
        <eos-icons:loading class="h-10 w-10" />
      </div>
      <div v-else-if="error" class="text-xl">
        <div class="text-center mb-6">
          {{ error }}
        </div>
        <btn-black class="w-full" :is-loading="isLoading" @click="resetDialog">
          Ok
        </btn-black>
      </div>
      <div v-else>
        <div class="">
          <h3 class="font-bold text-center leading-4">
            Donate to Creator
          </h3>
          <div class="py-8 px-4">
            <div class="space-y-6">
              <div>
                <label for="recurrences" class="font-medium text-sm text-gray-700 block"> recurrences(months) </label>
                <div class="mt-1">
                  <input id="recurrences" v-model="recurrences" name="recurrences" type="recurrences" autocomplete="recurrences" required="" class="border rounded-md border-gray-300 shadow-sm w-full py-2 px-3 placeholder-gray-400 appearance-none block sm:text-sm focus:outline-none focus:border-indigo-500 focus:ring-indigo-500">
                </div>
              </div>
              <div>
                <label for="amount" class="font-medium text-sm text-gray-700 block"> amount(per month) </label>
                <div class="mt-1">
                  <input id="amount" v-model="amount" name="amount" type="amount" required="" class="border rounded-md border-gray-300 shadow-sm w-full py-2 px-3 placeholder-gray-400 appearance-none block sm:text-sm focus:outline-none focus:border-indigo-500 focus:ring-indigo-500">
                </div>
              </div>
              <div>
                <btn-black class="w-full" :is-loading="isLoading" @click="doSubscribe">
                  Submit Donate
                </btn-black>
              </div>
            </div>
          </div>
        </div>
      </div>
    </DialogDefault>
  </div>
</template>
