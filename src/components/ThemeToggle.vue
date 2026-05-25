<script setup lang="ts">
import { ref, watch } from 'vue';
import Icon from './Icon.vue';

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

watch(isDark, (val: boolean) => applyTheme(val));
</script>

<template>
  <button
    @click="toggle"
    :title="isDark ? '切换亮色模式' : '切换暗色模式'"
    class="p-2 rounded-lg border transition-all duration-200 border-subtle bg-surface hover:bg-tertiary"
    :class="isDark ? 'text-accent-primary' : 'text-accent-secondary'"
  >
    <Icon :name="isDark ? 'sun-line' : 'moon-line'" size="lg" />
  </button>
</template>