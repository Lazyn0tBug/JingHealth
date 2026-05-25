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
      ? 'border-[--border-primary] bg-[--bg-secondary] text-[--accent-primary] hover:bg-[--bg-tertiary]'
      : 'border-[--border-primary] bg-[--bg-secondary] text-[--accent-secondary] hover:bg-[--bg-tertiary]'"
  >
    <!-- Moon (switch to dark) -->
    <i v-if="!isDark" class="ri-moon-line text-[--text-secondary] text-lg"></i>
    <!-- Sun (switch to light) -->
    <i v-else class="ri-sun-line text-[--accent-primary] text-lg"></i>
  </button>
</template>