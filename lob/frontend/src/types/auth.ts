export interface AuthUser {
  id: string
  email: string
  displayName: string
}

export interface LoginPayload {
  email: string
  password: string
  captchaToken: string
}

export interface RegisterPayload {
  email: string
  password: string
  captchaToken: string
}

export interface AuthSession {
  user: AuthUser
  token: string
}
