import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import * as authApi from '@/api/auth'
import type { AuthUser, LoginPayload, RegisterPayload } from '@/types/auth'

const STORAGE_KEY = 'korgi.lob.auth'

interface PersistedAuth {
  user: AuthUser
  token: string
}

function readPersisted(): PersistedAuth | null {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) {
      return null
    }
    return JSON.parse(raw) as PersistedAuth
  } catch {
    return null
  }
}

export const useAuthStore = defineStore('auth', () => {
  const persisted = readPersisted()
  const user = ref<AuthUser | null>(persisted?.user ?? null)
  const token = ref<string | null>(persisted?.token ?? null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const isAuthenticated = computed(() => Boolean(user.value && token.value))

  function persist(): void {
    if (!user.value || !token.value) {
      localStorage.removeItem(STORAGE_KEY)
      return
    }
    const payload: PersistedAuth = { user: user.value, token: token.value }
    localStorage.setItem(STORAGE_KEY, JSON.stringify(payload))
  }

  async function login(payload: LoginPayload): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const session = await authApi.login(payload)
      user.value = session.user
      token.value = session.token
      persist()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Ошибка входа'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function register(payload: RegisterPayload): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const session = await authApi.register(payload)
      user.value = session.user
      token.value = session.token
      persist()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Ошибка регистрации'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function logout(): Promise<void> {
    loading.value = true
    error.value = null
    try {
      await authApi.logout()
    } finally {
      user.value = null
      token.value = null
      persist()
      loading.value = false
    }
  }

  return {
    user,
    token,
    loading,
    error,
    isAuthenticated,
    login,
    register,
    logout,
  }
})
