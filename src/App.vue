<script setup lang="ts">
import { onMounted } from 'vue';
import ThemeToggle from './components/ThemeToggle.vue';

onMounted(() => {
  // Apply saved theme on load
  const saved = localStorage.getItem('theme');
  if (saved === 'dark' || saved === 'light') {
    document.documentElement.setAttribute('data-theme', saved);
  } else {
    // No saved preference — follow OS
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
  }
});
</script>

<template>
  <div class="min-h-screen bg-base text-foreground">
    <!-- Fixed theme toggle — top right -->
    <div class="fixed top-3 right-3 sm:top-4 sm:right-4 z-50">
      <ThemeToggle />
    </div>
    <RouterView />
  </div>
</template>