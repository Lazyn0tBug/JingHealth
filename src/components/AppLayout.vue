<script setup lang="ts">
/**
 * AppLayout — 统一页面布局容器
 *
 * 所有 view 外层包住此组件，获得一致的：
 * - max-width 容器
 * - 响应式内外边距
 * - 页面头部（标题 + 返回）
 * - 固定 ThemeToggle
 *
 * Usage:
 *   <AppLayout title="患者详情" back-href="/">
 *     ...page content...
 *   </AppLayout>
 *
 *   <AppLayout title="患者列表">
 *     ...page content...
 *   </AppLayout>
 */
import { useRouter } from 'vue-router';
import Icon from './Icon.vue';
import ThemeToggle from './ThemeToggle.vue';

defineProps<{
  /** 页面标题 */
  title?: string;
  /** 返回按钮目标路径，不传则不显示返回按钮 */
  backHref?: string;
  /** 返回按钮文字，默认为 "返回" */
  backLabel?: string;
  /** 容器最大宽度，默认为 "6xl" (72rem) */
  maxWidth?: string;
  /** 页面级别 padding，默认 "p-4 sm:p-6" */
  padding?: string;
}>();

const $router = useRouter();
</script>

<template>
  <div class="min-h-screen bg-base flex flex-col">
    <!-- 固定 ThemeToggle — 顶层右侧 -->
    <div class="fixed top-3 right-3 sm:top-4 sm:right-4 z-50">
      <ThemeToggle />
    </div>

    <!-- 主内容区 -->
    <main :class="padding ?? 'p-4 sm:p-6'">
      <div :class="['mx-auto', maxWidth ?? 'max-w-6xl', 'flex flex-col', 'space-y-4']">

        <!-- 页面头部 -->
        <header v-if="title || backHref" class="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-4">
          <button
            v-if="backHref"
            @click="() => { if (backHref) $router?.push(backHref); }"
            class="text-text-tertiary hover:text-text-secondary text-sm flex items-center gap-1 self-start"
          >
            <Icon name="arrow-left-line" size="base" />
            <span>{{ backLabel ?? '返回' }}</span>
          </button>
          <h1 v-if="title" class="text-xl sm:text-2xl font-semibold text-accent-secondary">
            {{ title }}
          </h1>
          <!-- header 右侧插槽（tab 行等） -->
          <div v-if="$slots.headerExtra" class="sm:ml-auto">
            <slot name="headerExtra" />
          </div>
        </header>

        <!-- 页面内容 -->
        <slot />

      </div>
    </main>
  </div>
</template>