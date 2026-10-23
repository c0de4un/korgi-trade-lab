const API_BASE = (import.meta.env.VITE_LOB_API_URL ?? '').replace(/\/$/, '')

export class ApiError extends Error {
  readonly status: number

  constructor(message: string, status = 400) {
    super(message)
    this.name = 'ApiError'
    this.status = status
  }
}

export interface RequestOptions extends Omit<RequestInit, 'body'> {
  body?: unknown
  token?: string | null
}

/**
 * Thin fetch wrapper. Base URL comes from VITE_LOB_API_URL.
 * Mock auth currently bypasses this; real endpoints will use it later.
 */
export async function apiRequest<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const { body, token, headers: initHeaders, ...rest } = options
  const headers = new Headers(initHeaders)

  if (body !== undefined && !headers.has('Content-Type')) {
    headers.set('Content-Type', 'application/json')
  }
  if (token) {
    headers.set('Authorization', `Bearer ${token}`)
  }

  const response = await fetch(`${API_BASE}${path}`, {
    ...rest,
    headers,
    body: body === undefined ? undefined : JSON.stringify(body),
  })

  if (!response.ok) {
    let message = `HTTP ${response.status}`
    try {
      const payload = (await response.json()) as { message?: string }
      if (payload.message) {
        message = payload.message
      }
    } catch {
      // ignore non-JSON error bodies
    }
    throw new ApiError(message, response.status)
  }

  if (response.status === 204) {
    return undefined as T
  }

  return (await response.json()) as T
}

export function getApiBaseUrl(): string {
  return API_BASE
}
