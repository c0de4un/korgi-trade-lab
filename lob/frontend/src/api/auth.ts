import type { AuthSession, LoginPayload, RegisterPayload } from '@/types/auth'
import { ApiError } from '@/api/http'

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms)
  })
}

function assertCaptcha(token: string): void {
  if (!token.trim()) {
    throw new ApiError('Пройдите проверку капчи', 400)
  }
}

function mockSession(email: string): AuthSession {
  const normalized = email.trim().toLowerCase()
  return {
    token: `mock-${crypto.randomUUID()}`,
    user: {
      id: crypto.randomUUID(),
      email: normalized,
      displayName: normalized.split('@')[0] || 'admin',
    },
  }
}

/** Mock login — later: POST `${VITE_LOB_API_URL}/api/v1/auth/login` */
export async function login(payload: LoginPayload): Promise<AuthSession> {
  assertCaptcha(payload.captchaToken)
  if (!payload.email.trim() || !payload.password) {
    throw new ApiError('Укажите email и пароль', 400)
  }
  await delay(300)
  return mockSession(payload.email)
}

/** Mock register — later: POST `${VITE_LOB_API_URL}/api/v1/auth/register` */
export async function register(payload: RegisterPayload): Promise<AuthSession> {
  assertCaptcha(payload.captchaToken)
  if (!payload.email.trim() || payload.password.length < 10) {
    throw new ApiError('Пароль должен быть не короче 10 символов', 400)
  }
  await delay(300)
  return mockSession(payload.email)
}

/** Mock logout — later: POST `${VITE_LOB_API_URL}/api/v1/auth/logout` */
export async function logout(): Promise<void> {
  await delay(100)
}
