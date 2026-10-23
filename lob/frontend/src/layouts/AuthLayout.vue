<script setup lang="ts">
import ThemeToggle from '@/components/ThemeToggle.vue'

const appTitle = import.meta.env.VITE_APP_TITLE || 'Korgi LOB Admin'
</script>

<template>
  <div class="relative flex min-h-screen flex-col bg-surface">
    <div
      class="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-accent-soft/40 via-transparent to-transparent"
      aria-hidden="true"
    />

    <div class="relative z-10 flex items-center justify-between px-4 py-4 sm:px-6">
      <div>
        <p class="text-xs font-semibold uppercase tracking-wider text-accent">Korgi.Trade</p>
        <p class="text-sm font-medium text-ink">{{ appTitle }}</p>
      </div>
      <ThemeToggle />
    </div>

    <main class="relative z-10 flex flex-1 items-center justify-center px-4 pb-12">
      <div class="card-panel w-full max-w-md">
        <slot />
      </div>
    </main>
  </div>
</template>
