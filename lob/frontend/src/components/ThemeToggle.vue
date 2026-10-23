<script setup lang="ts">
import { useThemeStore } from '@/stores/theme'

const theme = useThemeStore()
</script>

<template>
  <button
    type="button"
    class="btn-ghost gap-2"
    :title="theme.resolved === 'dark' ? 'Светлая тема' : 'Тёмная тема'"
    @click="theme.toggleLightDark()"
  >
    <span aria-hidden="true">{{ theme.resolved === 'dark' ? '☀' : '☾' }}</span>
    <span class="hidden sm:inline">{{ theme.resolved === 'dark' ? 'Светлая' : 'Тёмная' }}</span>
  </button>
</template>
