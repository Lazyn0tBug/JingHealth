<script setup lang="ts">
import { onMounted } from 'vue';

onMounted(() => {
  const saved = localStorage.getItem('theme');
  if (saved === 'dark' || saved === 'light') {
    document.documentElement.setAttribute('data-theme', saved);
  } else {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
  }
});
</script>

<template>
  <div class="min-h-screen bg-base text-foreground">
    <RouterView />
  </div>
</template>