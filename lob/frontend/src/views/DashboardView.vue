<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
</script>

<template>
  <div class="space-y-4">
    <div class="card-panel">
      <h2 class="text-lg font-semibold text-ink">Добро пожаловать</h2>
      <p class="mt-2 text-sm text-ink-muted">
        Вы вошли как
        <span class="font-medium text-ink">{{ auth.user?.email }}</span>.
        Это админ-панель LOB: мониторинг симуляций, воркеров и доступов.
      </p>
    </div>

    <div class="grid gap-4 sm:grid-cols-3">
      <div class="card-panel">
        <p class="text-xs uppercase tracking-wide text-ink-muted">Симуляции</p>
        <p class="mt-2 text-2xl font-semibold text-ink">—</p>
        <p class="mt-1 text-sm text-ink-muted">Данные появятся после подключения API</p>
      </div>
      <div class="card-panel">
        <p class="text-xs uppercase tracking-wide text-ink-muted">Воркеры</p>
        <p class="mt-2 text-2xl font-semibold text-ink">—</p>
        <p class="mt-1 text-sm text-ink-muted">Статусы воркеров — в разработке</p>
      </div>
      <div class="card-panel">
        <p class="text-xs uppercase tracking-wide text-ink-muted">Клиенты</p>
        <p class="mt-2 text-2xl font-semibold text-ink">—</p>
        <p class="mt-1 text-sm text-ink-muted">Выданные доступы — в разработке</p>
      </div>
    </div>
  </div>
</template>
