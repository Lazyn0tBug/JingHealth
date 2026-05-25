<script setup lang="ts">
import { onMounted } from 'vue';

onMounted(() => {
  const saved = localStorage.getItem('theme');
  if (saved === 'dark' || saved === 'light') {
    document.documentElement.classList.toggle('dark', saved === 'dark');
  } else {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.classList.toggle('dark', prefersDark);
  }
});
</script>

<template>
  <div class="min-h-screen bg-base text-foreground">
    <RouterView />
  </div>
</template>