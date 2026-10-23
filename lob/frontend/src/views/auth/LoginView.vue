<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'

import YandexSmartCaptcha from '@/components/auth/YandexSmartCaptcha.vue'
import AuthLayout from '@/layouts/AuthLayout.vue'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
const router = useRouter()

const email = ref('')
const password = ref('')
const captchaToken = ref('')
const formError = ref<string | null>(null)

const canSubmit = computed(
  () => Boolean(email.value.trim() && password.value && captchaToken.value && !auth.loading),
)

function onCaptchaToken(token: string): void {
  captchaToken.value = token
  formError.value = null
}

function onCaptchaReset(): void {
  captchaToken.value = ''
}

async function onSubmit(): Promise<void> {
  formError.value = null
  if (!canSubmit.value) {
    formError.value = 'Заполните форму и пройдите капчу'
    return
  }
  try {
    await auth.login({
      email: email.value,
      password: password.value,
      captchaToken: captchaToken.value,
    })
    await router.push({ name: 'dashboard' })
  } catch (err) {
    formError.value = err instanceof Error ? err.message : 'Не удалось войти'
    captchaToken.value = ''
  }
}
</script>

<template>
  <AuthLayout>
    <h1 class="text-xl font-semibold text-ink">Вход</h1>
    <p class="mt-1 text-sm text-ink-muted">Админ-панель LOB Engine</p>

    <form class="mt-6 space-y-4" @submit.prevent="onSubmit">
      <div>
        <label class="field-label" for="login-email">Email</label>
        <input
          id="login-email"
          v-model="email"
          class="field-input"
          type="email"
          autocomplete="username"
          required
          placeholder="admin@korgi.trade"
        />
      </div>

      <div>
        <label class="field-label" for="login-password">Пароль</label>
        <input
          id="login-password"
          v-model="password"
          class="field-input"
          type="password"
          autocomplete="current-password"
          required
          placeholder="••••••••••"
        />
      </div>

      <YandexSmartCaptcha
        @token="onCaptchaToken"
        @reset="onCaptchaReset"
        @error="formError = $event"
      />

      <p v-if="formError || auth.error" class="text-sm text-red-600 dark:text-red-400">
        {{ formError || auth.error }}
      </p>

      <button type="submit" class="btn-primary w-full" :disabled="!canSubmit">
        {{ auth.loading ? 'Вход…' : 'Войти' }}
      </button>
    </form>

    <p class="mt-4 text-center text-sm text-ink-muted">
      Нет аккаунта?
      <RouterLink class="font-medium text-accent hover:underline" :to="{ name: 'register' }">
        Регистрация
      </RouterLink>
    </p>
  </AuthLayout>
</template>
