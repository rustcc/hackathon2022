<script setup lang="ts">

interface Props {
  address: string
}
const {
  address,
} = defineProps<Props>()

const links = [
  {
    name: 'My Blog Posts',
    // desc: 'Our personal website',
    type: 'blog',
    to: `/${address}/blog`,
  },
  {
    name: 'All my art NFT Here',
    // desc: 'Our personal website',
    type: 'art',
    to: `/${address}/art`,
  },
  // {
  //   name: 'All my hacker NFT Here',
  //   // desc: 'Our personal website',
  //   type: 'hacker',
  //   to: `/${address}/hacker`,
  // },
  {
    name: 'Web3Hacker\'s personal website',
    // desc: 'Our personal website',
    type: 'website',
    href: 'https://web3hacker.ninja',
  },
  {
    name: 'Web3Hacker\'s twitter',
    // desc: 'share our twitter',
    type: 'twitter',
    href: 'https://twitter.com/web3hacker007',
  },
  {
    name: 'Web3Hacker\'s Instagram',
    // desc: 'share our daily photo',
    type: 'instagram',
    href: 'https://instagram.com/web3hacker007',
  },
  {
    name: 'Web3Hacker\'s Facebook',
    // desc: 'share our facebook page',
    type: 'facebook',
    href: 'https://facebook.com/web3hacker007',
  },
  {
    name: 'Web3Hacker\'s OpenSea',
    // desc: 'share our NFT on OpenSea',
    type: 'opensea',
    href: 'https://opensea.io/web3hacker007',
  },
  {
    name: 'Web3Hacker\'s discord',
    // desc: 'share our NFT on discord',
    type: 'discord',
    href: 'https://discord.com/web3hacker007',
  },
  {
    name: 'Web3Hacker\'s telegram',
    // desc: 'share our NFT on telegram',
    type: 'telegram',
    href: 'https://telegram.com/web3hacker007',
  },
  // More people...
]

</script>
<template>
  <div class="">
    <div v-for="item in links" :key="item.email" class="bg-white border rounded-lg flex space-x-3 border-gray-300 shadow-sm mb-5 py-5 px-6 relative items-center hover:border-gray-400 focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-indigo-500">
      <div class="flex-shrink-0">
        <akar-icons:twitter-fill v-if="item.type === 'twitter'" class="h-10 w-10" />
        <ant-design:instagram-filled v-else-if="item.type === 'instagram'" class="h-10 w-10" />
        <bi:book-half v-else-if="item.type === 'blog'" class="h-10 w-10" />
        <akar-icons:facebook-fill v-else-if="item.type === 'facebook'" class="h-10 w-10" />
        <dashicons:admin-site-alt2 v-else-if="item.type === 'website'" class="h-10 w-10" />
        <simple-icons:opensea v-else-if="item.type === 'opensea'" class="h-10 w-10" />
        <akar-icons:discord-fill v-else-if="item.type === 'discord'" class="h-10 w-10" />
        <akar-icons:telegram-fill v-else-if="item.type === 'telegram'" class="h-10 w-10" />
        <emojione-monotone:artist-palette v-else-if="item.type === 'art'" class="h-10 w-10" />
        <ant-design:code-outlined v-else-if="item.type === 'hacker'" class="h-10 w-10" />
        <img v-else class="rounded-full h-10 w-10" :src="item.imageUrl" alt="">
      </div>
      <div class="flex-1 min-w-0">
        <router-link v-if="item.to" :to="item.to" class="focus:outline-none">
          <span class="inset-0 absolute" aria-hidden="true" />
          <p class="font-medium text-sm text-gray-900">
            {{ item.name }}
          </p>
          <p class="text-sm text-gray-500 truncate">
            {{ item.desc }}
          </p>
        </router-link>
        <a v-else :href="item.href" class="focus:outline-none" target="_blank">
          <span class="inset-0 absolute" aria-hidden="true" />
          <p class="font-medium text-sm text-gray-900">
            {{ item.name }}
          </p>
          <p class="text-sm text-gray-500 truncate">
            {{ item.desc }}
          </p>
        </a>
      </div>
    </div>
  </div>
</template>
