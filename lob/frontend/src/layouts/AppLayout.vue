<script setup lang="ts">
import AppHeader from '@/components/layout/AppHeader.vue'
import AppSidebar from '@/components/layout/AppSidebar.vue'
</script>

<template>
  <div class="flex min-h-screen bg-surface">
    <AppSidebar />
    <div class="flex min-w-0 flex-1 flex-col">
      <AppHeader />
      <main class="flex-1 overflow-auto p-4 sm:p-6">
        <slot />
      </main>
    </div>
  </div>
</template>
