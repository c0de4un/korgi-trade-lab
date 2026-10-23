import { createRouter, createWebHistory } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/auth/LoginView.vue'),
      meta: { guest: true, title: 'Вход' },
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('@/views/auth/RegisterView.vue'),
      meta: { guest: true, title: 'Регистрация' },
    },
    {
      path: '/',
      component: () => import('@/layouts/AppShell.vue'),
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'dashboard',
          component: () => import('@/views/DashboardView.vue'),
          meta: { title: 'Dashboard' },
        },
        {
          path: 'clients/access',
          name: 'clients-access',
          component: () => import('@/views/PlaceholderView.vue'),
          meta: { title: 'Доступы клиентов' },
        },
        {
          path: 'simulations',
          name: 'simulations',
          component: () => import('@/views/PlaceholderView.vue'),
          meta: { title: 'Симуляции' },
        },
        {
          path: 'simulations/workers',
          name: 'simulations-workers',
          component: () => import('@/views/PlaceholderView.vue'),
          meta: { title: 'Воркеры симуляций' },
        },
        {
          path: 'monitoring/load',
          name: 'monitoring-load',
          component: () => import('@/views/PlaceholderView.vue'),
          meta: { title: 'Нагрузка' },
        },
        {
          path: 'settings/profile',
          name: 'settings-profile',
          component: () => import('@/views/PlaceholderView.vue'),
          meta: { title: 'Профиль' },
        },
      ],
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: '/',
    },
  ],
})

router.beforeEach((to) => {
  const auth = useAuthStore()
  const appTitle = import.meta.env.VITE_APP_TITLE || 'Korgi LOB Admin'
  const pageTitle = typeof to.meta.title === 'string' ? to.meta.title : null
  document.title = pageTitle ? `${pageTitle} · ${appTitle}` : appTitle

  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    return { name: 'login', query: { redirect: to.fullPath } }
  }
  if (to.meta.guest && auth.isAuthenticated) {
    return { name: 'dashboard' }
  }
  return true
})

export default router
