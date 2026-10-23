/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_LOB_API_URL: string
  readonly VITE_YANDEX_SMARTCAPTCHA_SITEKEY: string
  readonly VITE_APP_TITLE: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}

interface Window {
  smartCaptcha?: {
    render: (
      container: HTMLElement | string,
      params: {
        sitekey: string
        callback?: (token: string) => void
        'error-callback'?: () => void
        'expired-callback'?: () => void
        hl?: string
        theme?: 'light' | 'dark'
      },
    ) => number
    reset: (widgetId?: number) => void
    destroy: (widgetId?: number) => void
    getResponse: (widgetId?: number) => string
  }
}
