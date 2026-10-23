<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'

import { useThemeStore } from '@/stores/theme'

const emit = defineEmits<{
  token: [string]
  error: [string]
  reset: []
}>()

const theme = useThemeStore()
const sitekey = import.meta.env.VITE_YANDEX_SMARTCAPTCHA_SITEKEY ?? ''
const containerRef = ref<HTMLElement | null>(null)
const widgetId = ref<number | null>(null)
const loadError = ref<string | null>(null)
const scriptLoading = ref(false)

const hasSitekey = computed(() => Boolean(sitekey.trim()))
const showDevFallback = computed(() => !hasSitekey.value && import.meta.env.DEV)

const SCRIPT_SRC = 'https://smartcaptcha.yandexcloud.net/captcha.js?render=onload'

function loadScript(): Promise<void> {
  if (window.smartCaptcha) {
    return Promise.resolve()
  }

  const existing = document.querySelector<HTMLScriptElement>(`script[src="${SCRIPT_SRC}"]`)
  if (existing) {
    return new Promise((resolve, reject) => {
      existing.addEventListener('load', () => resolve(), { once: true })
      existing.addEventListener('error', () => reject(new Error('Не удалось загрузить SmartCaptcha')), {
        once: true,
      })
    })
  }

  return new Promise((resolve, reject) => {
    const script = document.createElement('script')
    script.src = SCRIPT_SRC
    script.async = true
    script.defer = true
    script.onload = () => resolve()
    script.onerror = () => reject(new Error('Не удалось загрузить SmartCaptcha'))
    document.head.appendChild(script)
  })
}

function destroyWidget(): void {
  if (widgetId.value !== null && window.smartCaptcha) {
    try {
      window.smartCaptcha.destroy(widgetId.value)
    } catch {
      // widget may already be gone
    }
  }
  widgetId.value = null
  if (containerRef.value) {
    containerRef.value.innerHTML = ''
  }
}

async function renderWidget(): Promise<void> {
  if (!hasSitekey.value || !containerRef.value) {
    return
  }

  loadError.value = null
  scriptLoading.value = true
  try {
    await loadScript()
    destroyWidget()
    if (!window.smartCaptcha || !containerRef.value) {
      throw new Error('SmartCaptcha API недоступен')
    }
    widgetId.value = window.smartCaptcha.render(containerRef.value, {
      sitekey,
      hl: 'ru',
      theme: theme.resolved === 'dark' ? 'dark' : 'light',
      callback: (token: string) => {
        emit('token', token)
      },
      'error-callback': () => {
        emit('error', 'Ошибка капчи')
        emit('reset')
      },
      'expired-callback': () => {
        emit('reset')
      },
    })
  } catch (err) {
    const message = err instanceof Error ? err.message : 'Ошибка капчи'
    loadError.value = message
    emit('error', message)
  } finally {
    scriptLoading.value = false
  }
}

function useDevToken(): void {
  emit('token', `dev-captcha-${Date.now()}`)
}

onMounted(() => {
  void renderWidget()
})

onBeforeUnmount(() => {
  destroyWidget()
})

watch(
  () => theme.resolved,
  () => {
    if (hasSitekey.value) {
      void renderWidget()
    }
  },
)
</script>

<template>
  <div class="space-y-2">
    <div v-if="hasSitekey" ref="containerRef" class="min-h-[100px]" />

    <p v-if="scriptLoading" class="text-sm text-ink-muted">Загрузка капчи…</p>
    <p v-if="loadError" class="text-sm text-red-600 dark:text-red-400">{{ loadError }}</p>

    <div
      v-if="showDevFallback"
      class="rounded-lg border border-dashed border-line bg-surface-muted p-3 text-sm text-ink-muted"
    >
      <p class="mb-2">
        Sitekey SmartCaptcha не задан. В DEV можно пропустить капчу.
      </p>
      <button type="button" class="btn-ghost" @click="useDevToken">
        Skip captcha (dev)
      </button>
    </div>

    <p
      v-else-if="!hasSitekey"
      class="text-sm text-amber-700 dark:text-amber-300"
    >
      Задайте VITE_YANDEX_SMARTCAPTCHA_SITEKEY в .env
    </p>
  </div>
</template>
