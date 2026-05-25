<script setup lang="ts">
/**
 * AppLayout — 统一页面布局容器
 *
 * 所有 view 外层包住此组件，获得一致的：
 * - 顶层浮动导航栏
 * - max-width 容器
 * - 响应式内外边距
 * - 页面头部（标题 + 返回）
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
import { useRouter, useRoute } from 'vue-router';
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
  /** 是否显示顶部导航栏，默认 true */
  showNav?: boolean;
}>();

const $router = useRouter();
const $route = useRoute();

function isActive(href: string) {
  return $route.path === href ? 'text-accent-primary' : 'text-text-secondary';
}
</script>

<template>
  <div class="min-h-screen bg-base flex flex-col">
    <!-- 顶层浮动导航栏 -->
    <nav v-if="showNav !== false" class="fixed top-0 left-0 right-0 z-40 h-12 bg-[--bg-primary] border-b border-[--border-primary] flex items-center px-4 sm:px-6">
      <div class="mx-auto w-full max-w-6xl flex items-center justify-between gap-4">
        <!-- 左：Logo + 站点名 -->
        <button @click="$router.push('/')" class="flex items-center gap-2 hover:opacity-80 transition-opacity">
          <Icon name="heart-pulse-line" size="lg" extraClass="text-accent-primary" />
          <span class="text-sm font-semibold text-accent-secondary hidden sm:inline">Mini-PIMS</span>
        </button>

        <!-- 中：主导航链接 -->
        <div class="flex items-center gap-1 sm:gap-2">
          <button
            @click="$router.push('/')"
            :class="['flex items-center gap-1 px-3 py-1.5 rounded-lg text-sm transition-colors hover:bg-[--bg-secondary]', isActive('/')]"
          >
            <Icon name="user-line" size="base" />
            <span class="hidden sm:inline">患者列表</span>
          </button>
          <button
            @click="$router.push('/register')"
            :class="['flex items-center gap-1 px-3 py-1.5 rounded-lg text-sm transition-colors hover:bg-[--bg-secondary]', isActive('/register')]"
          >
            <Icon name="user-add-line" size="base" />
            <span class="hidden sm:inline">新建患者</span>
          </button>
        </div>

        <!-- 右：主题切换 -->
        <ThemeToggle />
      </div>
    </nav>

    <!-- 占位：导航栏高度 -->
    <div v-if="showNav !== false" class="h-12" />

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