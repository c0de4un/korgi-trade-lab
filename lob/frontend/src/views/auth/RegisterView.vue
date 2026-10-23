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
const passwordConfirm = ref('')
const captchaToken = ref('')
const formError = ref<string | null>(null)

const canSubmit = computed(
  () =>
    Boolean(
      email.value.trim() &&
        password.value &&
        passwordConfirm.value &&
        captchaToken.value &&
        !auth.loading,
    ),
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
  if (password.value !== passwordConfirm.value) {
    formError.value = 'Пароли не совпадают'
    return
  }
  if (password.value.length < 10) {
    formError.value = 'Пароль должен быть не короче 10 символов'
    return
  }
  if (!canSubmit.value) {
    formError.value = 'Заполните форму и пройдите капчу'
    return
  }
  try {
    await auth.register({
      email: email.value,
      password: password.value,
      captchaToken: captchaToken.value,
    })
    await router.push({ name: 'dashboard' })
  } catch (err) {
    formError.value = err instanceof Error ? err.message : 'Не удалось зарегистрироваться'
    captchaToken.value = ''
  }
}
</script>

<template>
  <AuthLayout>
    <h1 class="text-xl font-semibold text-ink">Регистрация</h1>
    <p class="mt-1 text-sm text-ink-muted">Создание учётной записи администратора LOB</p>

    <form class="mt-6 space-y-4" @submit.prevent="onSubmit">
      <div>
        <label class="field-label" for="register-email">Email</label>
        <input
          id="register-email"
          v-model="email"
          class="field-input"
          type="email"
          autocomplete="username"
          required
          placeholder="admin@korgi.trade"
        />
      </div>

      <div>
        <label class="field-label" for="register-password">Пароль</label>
        <input
          id="register-password"
          v-model="password"
          class="field-input"
          type="password"
          autocomplete="new-password"
          required
          minlength="10"
          placeholder="минимум 10 символов"
        />
      </div>

      <div>
        <label class="field-label" for="register-password-confirm">Подтверждение пароля</label>
        <input
          id="register-password-confirm"
          v-model="passwordConfirm"
          class="field-input"
          type="password"
          autocomplete="new-password"
          required
          minlength="10"
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
        {{ auth.loading ? 'Регистрация…' : 'Зарегистрироваться' }}
      </button>
    </form>

    <p class="mt-4 text-center text-sm text-ink-muted">
      Уже есть аккаунт?
      <RouterLink class="font-medium text-accent hover:underline" :to="{ name: 'login' }">
        Войти
      </RouterLink>
    </p>
  </AuthLayout>
</template>
