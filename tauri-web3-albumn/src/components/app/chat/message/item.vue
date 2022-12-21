<script setup lang="ts">
const message = {
  subject: 'Re: New pricing for existing customers',
  sender: 'Virginia Abshire',
  lastSeen: '2021-01-28T19:24',
  avatar: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80',
  status: 'Open',
  items: [
    {
      id: 1,
      author: 'Joe Armstrong',
      isSelf: true,
      date: 'Yesterday at 7:24am',
      datetime: '2021-01-28T19:24',
      body: '<p>Thanks so much! Can\'t wait to try it out.</p>',
    },
    {
      id: 2,
      author: 'Monica White',
      date: 'Wednesday at 4:35pm',
      datetime: '2021-01-27T16:35',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
      `,
    },
    {
      id: 3,
      author: 'Joe Armstrong',
      date: 'Wednesday at 4:09pm',
      datetime: '2021-01-27T16:09',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
      `,
    },
    {
      id: 1,
      author: 'Joe Armstrong',
      date: 'Yesterday at 7:24am',
      datetime: '2021-01-28T19:24',
      body: '<p>Thanks so much! Can\'t wait to try it out.</p>',
    },
    {
      id: 2,
      author: 'Monica White',
      date: 'Wednesday at 4:35pm',
      datetime: '2021-01-27T16:35',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
      `,
    },
    {
      id: 3,
      author: 'Joe Armstrong',
      date: 'Wednesday at 4:09pm',
      datetime: '2021-01-27T16:09',
      body: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Malesuada at ultricies tincidunt elit et, enim. Habitant nunc, adipiscing non fermentum, sed est a, aliquet. Lorem in vel libero vel augue aliquet dui commodo.</p>
        <p>Nec malesuada sed sit ut aliquet. Cras ac pharetra, sapien purus vitae vestibulum auctor faucibus ullamcorper. Leo quam tincidunt porttitor neque, velit sed. Tortor mauris ornare ut tellus sed aliquet amet venenatis condimentum. Convallis accumsan et nunc eleifend.</p>
      `,
    },
  ],
}
</script>
<template>
  <AppChatMessageContentHeader />
  <div class="flex-1 min-h-0 overflow-y-auto">
    <ul role="list" class="space-y-2 bg-gray-200 py-4 sm:space-y-4 sm:px-6 lg:px-8">
      <li v-for="item in message.items" :key="item.id" class="flex" :class="item.isSelf ? '' : 'flex-row-reverse'">
        <div class="flex-1 min-w-10" />
        <div class="shadow px-2 pt-4 sm:rounded-lg sm:px-4" :class="item.isSelf ? 'bg-green-100 text-right' : 'bg-white'">
          <div class="space-y-6 text-sm text-gray-800" v-html="item.body" />
          <div class="pb-2 sm:flex sm:justify-end sm:items-baseline">
            <h3 class="font-medium text-base hidden">
              <span class="text-gray-900">{{ item.author }}</span>
              {{ ' ' }}
              <span class="text-gray-600">wrote</span>
            </h3>
            <p class="mt-1 text-sm text-gray-400 whitespace-nowrap sm:mt-0 sm:ml-3">
              <time :datetime="item.datetime">{{ item.date }}</time>
            </p>
          </div>
        </div>
      </li>
    </ul>
  </div>

  <AppChatMessageContentFooter />
</template>
