<script setup lang="ts">
import { chatLink, shortAddress } from '~/helpers/web3'
const emit = defineEmits(['close'])

interface Props {
  items: []
}
const {
  items,
} = defineProps<Props>()
const { isOwner } = $(web3Store())
const router = useRouter()

let isLoading = $ref(true)
onMounted(() => {
  setTimeout(() => {
    isLoading = false
  }, 1000)
})

const goto = (address: string) => {
  emit('close')
  nextTick(() => {
    router.push({ path: `/${address}` })
  })
}
</script><template>
  <DialogDefault @close="$emit('close')">
    <div v-if="isLoading" class="text-center">
      loading...
    </div>
    <div v-else-if="items.length">
      <ul role="list" class="divide-y divide-gray-200 -my-5">
        <li v-for="item in items" :key="item.address" class="py-4">
          <div class="flex space-x-4 items-center">
            <div class="flex-shrink-0">
              <EthAvatar :address="item.address" class="rounded-full h-8 w-8" />
            </div>
            <div class="flex-1 min-w-0 hover:(cursor-pointer underline) focus:outline-none " @click="goto(item.address)">
              <p class="font-medium text-sm text-gray-900 truncate">
                {{ item.name }}
              </p>
              <p class="text-sm text-gray-500 truncate">
                {{ shortAddress(item.address) }}
              </p>
            </div>
            <div class="text-right w-40">
              <div v-if="!isOwner(item.address)" class="flex justify-end">
                <Web3NftBtnFollow :address="item.address" :hide-icon="true" />
                <a :href="chatLink(item.address)" target="_blank" class="bg-white border rounded-md font-medium border-gray-300 shadow-sm text-sm ml-1 py-2 px-4 text-gray-700 inline-flex justify-center hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-sky-500 disabled:(cursor-not-allowed) ">Chat</a>
              </div>
              <span v-else class="text-sm mr-2">My Self</span>
            </div>
          </div>
        </li>
      </ul>
    </div>
    <div v-else class="text-center text-lg">
      Uh oh!
      Nobody on this land yet ;)
    </div>
  </DialogDefault>
</template>
