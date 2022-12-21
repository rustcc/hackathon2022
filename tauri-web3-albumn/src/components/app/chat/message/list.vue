<script setup lang="ts">
// Create the client with an `ethers.Signer` from your application
// const xmtp = await Client.create(wallet)
// const conversations = xmtp.conversations

// List existing conversations
// const allConversations = await xmtp.conversations.list()

const messages = [
  {
    id: 1,
    subject: 'Velit placeat sit ducimus non sed',
    sender: 'Gloria Roberston',
    avatar: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',

    href: '#',
    date: '1d ago',
    datetime: '2021-01-27T16:35',
    preview:
      'Doloremque dolorem maiores assumenda dolorem facilis. Velit vel in a rerum natus facere. Enim rerum eaque qui facilis. Numquam laudantium sed id dolores omnis in. Eos reiciendis deserunt maiores et accusamus quod dolor.',
  },
  {
    id: 2,
    subject: 'Nemo mollitia repudiandae adipisci explicabo optio consequatur tempora ut nihil',
    sender: 'Virginia Abshire',
    avatar: 'https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
    href: '#',
    date: '1d ago',
    datetime: '2021-01-27T16:35',
    preview:
      'Doloremque dolorem maiores assumenda dolorem facilis. Velit vel in a rerum natus facere. Enim rerum eaque qui facilis. Numquam laudantium sed id dolores omnis in. Eos reiciendis deserunt maiores et accusamus quod dolor.',
  },
  {
    id: 3,
    subject: 'Doloremque reprehenderit et harum quas explicabo nulla architecto dicta voluptatibus',
    sender: 'Kyle Gulgowski',
    avatar: 'https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
    href: '#',
    date: '1d ago',
    datetime: '2021-01-27T16:35',
    preview:
      'Doloremque dolorem maiores assumenda dolorem facilis. Velit vel in a rerum natus facere. Enim rerum eaque qui facilis. Numquam laudantium sed id dolores omnis in. Eos reiciendis deserunt maiores et accusamus quod dolor.',
  },
]

</script>
<template>
  <nav aria-label="Message list" class="flex-1 min-h-0 overflow-y-auto">
    <ul role="list" class="divide-y border-b divide-gray-200 border-gray-200">
      <li v-for="message in messages" :key="message.id" class="bg-white py-5 px-6 relative hover:bg-gray-50 focus-within:ring-inset focus-within:ring-2 focus-within:ring-blue-600">
        <div class="flex space-x-3 justify-between">
          <div class="flex flex-1 min-w-0">
            <div class="flex-shrink-0 pr-2">
              <img class="rounded-full h-12 w-12" :src="message.avatar" alt="">
            </div>
            <a :href="message.href" class="block focus:outline-none">
              <span class="inset-0 absolute" aria-hidden="true" />
              <p class="font-medium text-sm text-gray-900 truncate">{{ message.sender }}</p>
              <p class="text-sm text-gray-500">{{ message.subject }}</p>
            </a>
          </div>
          <time :datetime="message.datetime" class="flex-shrink-0 text-xs text-gray-500 whitespace-nowrap">{{ message.date }}</time>
        </div>
      </li>
    </ul>
  </nav>
</template>
