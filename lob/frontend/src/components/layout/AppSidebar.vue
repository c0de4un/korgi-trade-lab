<script setup lang="ts">
import { navbarItems } from '@/configs/navbar'
import NavTree from '@/components/layout/NavTree.vue'
</script>

<template>
  <aside class="flex h-full w-64 shrink-0 flex-col border-r border-line bg-surface-elevated">
    <div class="border-b border-line px-4 py-4">
      <RouterLink to="/" class="block">
        <p class="text-xs font-semibold uppercase tracking-wider text-accent">Korgi.Trade</p>
        <p class="text-lg font-semibold text-ink">LOB Admin</p>
      </RouterLink>
    </div>
    <nav class="flex-1 overflow-y-auto p-3">
      <NavTree :items="navbarItems" />
    </nav>
  </aside>
</template>
