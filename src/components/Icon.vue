<script setup lang="ts">
/**
 * Icon — centralized icon abstraction layer.
 *
 * Usage: <Icon name="sun-line" size="lg" />
 *
 * Currently wraps RemixIcon (ri-* class naming).
 * To switch icon library, update ONLY this file:
 *   - Change the <i class="..."> tag to your library's component/SVG
 *   - The `name` prop format maps to your library's naming convention
 */
defineProps<{
  /** Icon name without ri- prefix (e.g. "sun-line", "search-line") */
  name: string;
  /** Size variant, corresponds to RemixIcon size classes */
  size?: 'xs' | 'sm' | 'base' | 'lg' | 'xl' | '2xl';
  /** Additional CSS classes */
  extraClass?: string;
}>();

const sizeMap: Record<string, string> = {
  xs: 'text-xs',
  sm: 'text-sm',
  base: 'text-base',
  lg: 'text-lg',
  xl: 'text-xl',
  '2xl': 'text-2xl',
};

function resolveIconClass(name: string): string {
  if (name.endsWith('-fill')) return `ri-${name}`;
  if (name.endsWith('-line')) return `ri-${name}`;
  return `ri-${name}-line`;
}
</script>

<template>
  <i
    :class="[
      resolveIconClass(name),
      size ? sizeMap[size] : '',
      extraClass ?? '',
    ]"
    :aria-label="name"
    role="img"
  ></i>
</template>