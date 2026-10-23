<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import ThemeToggle from '@/components/ThemeToggle.vue'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
const route = useRoute()
const router = useRouter()

const notificationsOpen = ref(false)
const profileOpen = ref(false)

const pageTitle = computed(() => {
  const title = route.meta.title
  return typeof title === 'string' ? title : 'LOB Admin'
})

const initials = computed(() => {
  const name = auth.user?.displayName ?? '?'
  return name.slice(0, 2).toUpperCase()
})

async function onLogout(): Promise<void> {
  await auth.logout()
  await router.push({ name: 'login' })
}
</script>

<template>
  <header class="flex h-14 items-center justify-between border-b border-line bg-surface-elevated px-4">
    <div>
      <h1 class="text-base font-semibold text-ink">{{ pageTitle }}</h1>
    </div>

    <div class="flex items-center gap-2">
      <ThemeToggle />

      <div class="relative">
        <button
          type="button"
          class="btn-ghost relative"
          title="Уведомления"
          @click="notificationsOpen = !notificationsOpen; profileOpen = false"
        >
          <span aria-hidden="true">🔔</span>
          <span
            class="absolute right-1.5 top-1.5 h-2 w-2 rounded-full bg-accent"
            aria-hidden="true"
          />
        </button>
        <div
          v-if="notificationsOpen"
          class="absolute right-0 z-20 mt-2 w-72 rounded-xl border border-line bg-surface-elevated p-3 shadow-lg"
        >
          <p class="text-sm font-medium text-ink">Уведомления</p>
          <p class="mt-2 text-sm text-ink-muted">Пока нет новых уведомлений.</p>
        </div>
      </div>

      <div class="relative">
        <button
          type="button"
          class="btn-ghost gap-2"
          @click="profileOpen = !profileOpen; notificationsOpen = false"
        >
          <span
            class="flex h-7 w-7 items-center justify-center rounded-full bg-accent-soft text-xs font-semibold text-accent"
          >
            {{ initials }}
          </span>
          <span class="hidden max-w-[10rem] truncate text-sm md:inline">
            {{ auth.user?.email }}
          </span>
        </button>
        <div
          v-if="profileOpen"
          class="absolute right-0 z-20 mt-2 w-56 rounded-xl border border-line bg-surface-elevated p-2 shadow-lg"
        >
          <p class="px-2 py-1 text-xs text-ink-muted">{{ auth.user?.displayName }}</p>
          <RouterLink
            to="/settings/profile"
            class="block rounded-lg px-2 py-2 text-sm text-ink hover:bg-surface-muted"
            @click="profileOpen = false"
          >
            Профиль
          </RouterLink>
          <button
            type="button"
            class="block w-full rounded-lg px-2 py-2 text-left text-sm text-red-600 hover:bg-surface-muted dark:text-red-400"
            @click="onLogout"
          >
            Выйти
          </button>
        </div>
      </div>
    </div>
  </header>
</template>
