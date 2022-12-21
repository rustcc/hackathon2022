<script setup lang="ts">
// import { CheckIcon } from '@heroicons/vue/24/outline'
const chainStr = Object.values(WEB3_NETWORK_LIST).map(val => val).join(', ')
const features = [
  {
    name: 'All EVM Chain Support',
    description: chainStr,
  },
  { name: 'Bookmark', description: 'You can manage all your NFT search log with tag.' },
  {
    name: 'Collection Page',
    description: 'You can add anyone\'s NFT into your collection page to marketing them.',
  },
  { name: 'Profile Page', description: 'You can edit your profile like instagram.' },
  { name: 'Marketing Campaign', description: 'You can make marketing campaign, buy other KOL\'s traffic' },
  { name: 'Reporting', description: 'Get all NFT traffic/status report.' },
  { name: 'Review', description: 'Review any NFT by rating it or comment it.' },
  { name: 'Support all platform', description: 'Web, Mobile(iOS, Android), mini program platform(weixin, toutiao, baidu...), PWA, Chrome extension, Firefox extension' },
]

</script>
<template>
  <div class="mx-auto max-w-7xl py-16 px-4 sm:px-6 lg:grid lg:py-24 lg:px-8 lg:gap-x-8 lg:grid-cols-3">
    <div class="mt-12 lg:mt-0 lg:col-span-3">
      <dl class="space-y-10 sm:space-y-0 sm:grid sm:grid-flow-col sm:gap-x-6 sm:gap-y-10 sm:grid-cols-2 sm:grid-rows-4 lg:gap-x-8">
        <div v-for="feature in features" :key="feature.name" class="relative">
          <dt>
            <!-- <CheckIcon class="h-6 text-green-500 w-6 absolute" aria-hidden="true" /> -->
            <p class="font-medium text-lg ml-9 text-gray-900 leading-6">
              {{ feature.name }}
            </p>
          </dt>
          <dd class="mt-2 text-base ml-9 text-gray-500">
            {{ feature.description }}
          </dd>
        </div>
      </dl>
    </div>
  </div>
</template>
