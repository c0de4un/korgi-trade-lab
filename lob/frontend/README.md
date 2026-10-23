# Korgi LOB Admin (WebUI)

Vue 3 + Vite + TypeScript админ-панель для мониторинга и управления LOB Engine.

## Требования

- Node.js 22 LTS
- npm

## Быстрый старт

```bash
cd lob/frontend
cp .env.example .env
npm install
npm run dev
```

Откроется Vite dev-server (по умолчанию `http://127.0.0.1:5173`).

## Переменные окружения

| Переменная | Описание |
|---|---|
| `VITE_LOB_API_URL` | Base URL LOB API (без trailing slash), например `http://127.0.0.1:8080` |
| `VITE_YANDEX_SMARTCAPTCHA_SITEKEY` | Client key [Яндекс SmartCaptcha](https://captcha.yandex.cloud/) |
| `VITE_APP_TITLE` | Заголовок приложения в браузере |

Без `VITE_YANDEX_SMARTCAPTCHA_SITEKEY` в режиме `DEV` на формах входа/регистрации доступна кнопка **Skip captcha (dev)**.

## Скрипты

```bash
npm run dev        # локальная разработка
npm run build      # production-сборка
npm run preview    # превью dist
npm run typecheck  # проверка типов
```

## Auth (mock)

Сейчас login/register работают через mock в `src/api/auth.ts` и сохраняют сессию в `localStorage`. Реальный backend подключится позже через `VITE_LOB_API_URL`.
