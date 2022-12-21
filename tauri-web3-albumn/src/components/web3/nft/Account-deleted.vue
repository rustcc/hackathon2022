<script setup lang="ts">
const isLoading = ref(false)
const username = computed(() => {
  if (!user.value) return 'not login'
  console.log('====> user.value :', user.value)
  return pick(user.value, 'attributes.username')
})
const avatar = computed(() => {
  return ''
})
const doLogin = async() => {
  isLoading.value = true
  isLoading.value = false
}
// if (!user.value)
//   doLogin()

const doSignOut = async() => {
  // const rz = await Moralis.User.logOut()
}
const userNavigation = [
  { name: 'Your Profile', href: '#' },
  { name: 'Settings', href: '#' },
  { name: 'Sign out', action: doSignOut },
]

</script>
<template>
  <button v-if="user" type="button" class="border border-transparent rounded-md font-medium bg-red-400 shadow-sm text-sm text-white ml-2 py-2 px-3 leading-4 inline-flex items-center hover:bg-red-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 disabled:(bg-gray-400 cursor-not-allowed hover:bg-gray-400) " :disabled="isLoading" @click="doLogin">
    Login
  </button>
  <Menu v-else as="div" class="ml-3 relative">
    <div>
      <MenuButton class="bg-white rounded-full flex max-w-xs text-sm items-center focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
        <span class="sr-only">Open user menu</span>
        <img v-if="avatar" class="rounded-full h-8 w-8" :src="avatar">
        <carbon-user-avatar v-else class="rounded-full h-8 w-8" />
      </MenuButton>
    </div>
    <transition enter-active-class="transition ease-out duration-200" enter-from-class="transform opacity-0 scale-95" enter-to-class="transform opacity-100 scale-100" leave-active-class="transition ease-in duration-75" leave-from-class="transform opacity-100 scale-100" leave-to-class="transform opacity-0 scale-95">
      <MenuItems class="bg-white rounded-md shadow-lg ring-black mt-2 py-1 origin-top-right right-0 ring-1 ring-opacity-5 w-48 absolute focus:outline-none">
        <MenuItem v-for="item in userNavigation" :key="item.name" v-slot="{ active }">
          <a :class="[active ? 'bg-gray-100' : '', 'block px-4 py-2 text-sm text-gray-700']" @click="item.action">
            {{ item.name }}
          </a>
        </MenuItem>
      </MenuItems>
    </transition>
  </Menu>
</template>
