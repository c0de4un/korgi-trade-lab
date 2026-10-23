<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'

import type { NavItem } from '@/types/navbar'

const props = defineProps<{
  items: NavItem[]
  depth?: number
}>()

const route = useRoute()
const depth = computed(() => props.depth ?? 0)
const openIds = ref<Set<string>>(new Set())

function itemMatchesRoute(item: NavItem): boolean {
  if (item.to && (route.path === item.to || (item.to !== '/' && route.path.startsWith(`${item.to}/`)))) {
    return true
  }
  return item.children?.some((child) => itemMatchesRoute(child)) ?? false
}

function ensureOpenForRoute(): void {
  for (const item of props.items) {
    if (item.children?.length && itemMatchesRoute(item)) {
      openIds.value.add(item.id)
    }
  }
}

watch(
  () => route.path,
  () => {
    ensureOpenForRoute()
  },
  { immediate: true },
)

function toggle(id: string): void {
  if (openIds.value.has(id)) {
    openIds.value.delete(id)
  } else {
    openIds.value.add(id)
  }
  openIds.value = new Set(openIds.value)
}

function isOpen(id: string): boolean {
  return openIds.value.has(id)
}

function linkClass(to?: string): string {
  const active = to === '/' ? route.path === '/' : Boolean(to && route.path.startsWith(to))
  return [
    'block rounded-lg px-3 py-2 text-sm transition',
    active
      ? 'bg-accent-soft font-medium text-accent'
      : 'text-ink-muted hover:bg-surface-muted hover:text-ink',
  ].join(' ')
}
</script>

<template>
  <ul :class="depth === 0 ? 'space-y-1' : 'ml-3 mt-1 space-y-1 border-l border-line pl-2'">
    <li v-for="item in items" :key="item.id">
      <template v-if="item.children?.length">
        <button
          type="button"
          class="flex w-full items-center justify-between rounded-lg px-3 py-2 text-left text-sm font-medium text-ink hover:bg-surface-muted"
          @click="toggle(item.id)"
        >
          <span>{{ item.label }}</span>
          <span class="text-ink-muted" aria-hidden="true">{{ isOpen(item.id) ? '▾' : '▸' }}</span>
        </button>
        <NavTree v-if="isOpen(item.id)" :items="item.children" :depth="depth + 1" />
      </template>

      <RouterLink v-else-if="item.to" :to="item.to" :class="linkClass(item.to)">
        {{ item.label }}
      </RouterLink>

      <span v-else class="block px-3 py-2 text-sm text-ink-muted">{{ item.label }}</span>
    </li>
  </ul>
</template>
