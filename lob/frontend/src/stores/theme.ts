import { computed, ref, watch } from 'vue'
import { defineStore } from 'pinia'

export type ThemeMode = 'light' | 'dark' | 'system'

const STORAGE_KEY = 'korgi.lob.theme'

function readStoredMode(): ThemeMode {
  const raw = localStorage.getItem(STORAGE_KEY)
  if (raw === 'light' || raw === 'dark' || raw === 'system') {
    return raw
  }
  return 'system'
}

function systemPrefersDark(): boolean {
  return window.matchMedia('(prefers-color-scheme: dark)').matches
}

export const useThemeStore = defineStore('theme', () => {
  const mode = ref<ThemeMode>(readStoredMode())
  const systemDark = ref(systemPrefersDark())

  const resolved = computed<'light' | 'dark'>(() => {
    if (mode.value === 'system') {
      return systemDark.value ? 'dark' : 'light'
    }
    return mode.value
  })

  function applyDom(): void {
    document.documentElement.classList.toggle('dark', resolved.value === 'dark')
    document.documentElement.dataset.theme = resolved.value
  }

  function setMode(next: ThemeMode): void {
    mode.value = next
    localStorage.setItem(STORAGE_KEY, next)
    applyDom()
  }

  function toggleLightDark(): void {
    setMode(resolved.value === 'dark' ? 'light' : 'dark')
  }

  function init(): void {
    applyDom()
    const media = window.matchMedia('(prefers-color-scheme: dark)')
    const onChange = (event: MediaQueryListEvent) => {
      systemDark.value = event.matches
      if (mode.value === 'system') {
        applyDom()
      }
    }
    media.addEventListener('change', onChange)
  }

  watch(resolved, () => {
    applyDom()
  })

  return {
    mode,
    resolved,
    setMode,
    toggleLightDark,
    init,
  }
})
