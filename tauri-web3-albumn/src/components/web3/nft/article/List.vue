
<script setup lang="ts">
import axios from 'axios'
import { queryBuilder } from '~/helpers/pinata'
const route = useRoute()
const userWalletAddress = $computed(() => route.params.userWalletAddress)

const articleLink = (item) => {
  return `/${userWalletAddress}/blog/${item.id}`
}
let posts = $ref([])
const isShowPostNew = $ref(false)
let isLoading = $ref(false)
const getList = async() => {
  isLoading = true
  const baseUrl = 'https://api.pinata.cloud'
  const metadataFilter = {
    keyvalues: {
      type: {
        value: 'blog',
        op: 'eq',
      },
    },
  }

  const filters = {
    // status: 'pinned',
    pageLimit: 10,
    pageOffset: 0,
    metadata: metadataFilter,
  }

  const baseEndpoint = `${baseUrl}/data/pinList`
  const endpoint = queryBuilder(baseEndpoint, filters)

  const rz = await axios.get(
    endpoint,
    {
      withCredentials: true,
      headers: {
        pinata_api_key: PINATA_KEY,
        pinata_secret_api_key: PINATA_SEC,
      },
    })

  const ipfsIds = rz.data.rows.map(({ ipfs_pin_hash }) => ipfs_pin_hash)
  const requestArr = ipfsIds.map(async(id) => {
    const rz = await axios.get(`https://gateway.pinata.cloud/ipfs/${id}`)
    return {
      id,
      ...rz.data,
    }
  })
  const blogPostsArr = await Promise.all(requestArr)
  posts = blogPostsArr
  isLoading = false
}
getList()

const formClose = async() => {
  isShowPostNew = false
  await getList()
}
</script>

<template>
  <div class="bg-gray-50 px-4 pt-16 pb-20 relative sm:px-6 lg:px-8 lg:pt-24 lg:pb-28">
    <div class="inset-0 absolute">
      <div class="bg-white h-1/3 sm:h-2/3" />
    </div>
    <div class="mx-auto max-w-7xl relative">
      <button class="rounded-full bg-red-500 border-1 text-white py-4 px-10 -top-20 right-1 absolute" @click="isShowPostNew = true">
        Post New
      </button>
      <div class="text-center">
        <h2 class="font-extrabold tracking-tight text-3xl text-gray-900 sm:text-4xl">
          From the blog
        </h2>
        <p class="mx-auto mt-3 text-xl max-w-2xl text-gray-500 sm:mt-4">
          All the thoughts for the Web3.
        </p>
      </div>
      <div class="mx-auto max-w-lg mt-12 grid gap-5 lg:max-w-none lg:grid-cols-3">
        <div v-for="post in posts" :key="post.title" class="rounded-lg flex flex-col shadow-lg overflow-hidden">
          <div class="flex-shrink-0">
            <img class="object-cover h-48 w-full" :src="post.coverImg" alt="">
          </div>
          <div class="bg-white flex flex-col flex-1 p-6 justify-between">
            <div class="flex-1">
              <!-- <p class="font-medium text-sm text-indigo-600">
                <a :href="post.category.href" class="hover:underline">
                  {{ post.category.name }}
                </a>
              </p> -->
              <router-link :to="articleLink(post)" class="mt-2 block">
                <p class="font-semibold text-xl text-gray-900">
                  {{ post.title }}
                </p>
                <p class="mt-3 text-base text-gray-500">
                  {{ post.excerpt }}
                </p>
              </router-link>
            </div>
            <!-- <div class="flex mt-6 items-center hidden">
              <div class="flex-shrink-0">
                <a :href="post.author.href">
                  <span class="sr-only">{{ post.author.name }}</span>
                  <img class="rounded-full h-10 w-10" :src="post.author.imageUrl" alt="">
                </a>
              </div>
              <div class="ml-3">
                <p class="font-medium text-sm text-gray-900">
                  <a :href="post.author.href" class="hover:underline">
                    {{ post.author.name }}
                  </a>
                </p>
                <div class="flex space-x-1 text-sm text-gray-500">
                  <time :datetime="post.datetime">
                    {{ post.date }}
                  </time>
                  <span aria-hidden="true"> &middot; </span>
                  <span> {{ post.readingTime }} read </span>
                </div>
              </div>
            </div> -->
          </div>
        </div>
      </div>
      <div v-if="isLoading" class="flex mb-20 justify-center items-center">
        <eos-icons:loading class="mr-2" />
      </div>
      <div v-else-if="!posts.length" class="flex mb-20 justify-center items-center">
        <h2 class="text-6xl">
          No post yet!
        </h2>
      </div>
    </div>
  </div>
  <Web3NftArticleForm :show="isShowPostNew" @close="formClose" />
</template>
