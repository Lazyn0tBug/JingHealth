<script setup lang="ts">
import { ref, watch } from 'vue';

const isDark = ref(false);

// Sync on mount
if (typeof window !== 'undefined') {
  const saved = localStorage.getItem('theme');
  const prefersDark = !saved ? window.matchMedia('(prefers-color-scheme: dark)').matches : saved === 'dark';
  isDark.value = prefersDark;
}

function applyTheme(dark: boolean) {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-theme', dark ? 'dark' : 'light');
    localStorage.setItem('theme', dark ? 'dark' : 'light');
  }
}

function toggle() {
  isDark.value = !isDark.value;
  applyTheme(isDark.value);
}

// Apply initial state
applyTheme(isDark.value);

watch(isDark, (val) => applyTheme(val));
</script>

<template>
  <button
    @click="toggle"
    :title="isDark ? '切换亮色模式' : '切换暗色模式'"
    class="p-2 rounded-lg border transition-all duration-200"
    :class="isDark
      ? 'bg-gray-800 text-yellow-400 border-gray-600 hover:bg-gray-700'
      : 'bg-gray-100 text-gray-600 border-gray-200 hover:bg-gray-200'"
  >
    <!-- Moon (switch to dark) -->
    <svg v-if="!isDark" xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
    </svg>
    <!-- Sun (switch to light) -->
    <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
    </svg>
  </button>
</template>