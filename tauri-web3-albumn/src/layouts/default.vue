<template>
  <TopNav />
  <main class="flex bg-[rgba(var(--b3f,250,250,250),1)] justify-center">
    <div class="min-h-screen w-250">
      <router-view />
    </div>
  </main>
</template>
<script setup lang="ts">
useHead({
  link: [
    { rel: 'icon', href: '/favicon.svg', type: 'image/svg+xml' },
    { rel: 'apple-touch-icon', href: '/pwa-192x192.png' },
    { rel: 'mask-icon', href: '/safari-pinned-tab.svg', color: '#00aba9' },
  ],
})
</script>
