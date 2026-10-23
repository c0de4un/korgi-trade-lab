import type { NavItem } from '@/types/navbar'

export const navbarItems: NavItem[] = [
  {
    id: 'dashboard',
    label: 'Dashboard',
    to: '/',
  },
  {
    id: 'clients',
    label: 'Клиенты',
    children: [
      { id: 'clients-access', label: 'Доступы', to: '/clients/access' },
    ],
  },
  {
    id: 'simulations',
    label: 'Симуляции',
    children: [
      { id: 'simulations-list', label: 'Список', to: '/simulations' },
      { id: 'simulations-workers', label: 'Воркеры', to: '/simulations/workers' },
    ],
  },
  {
    id: 'monitoring',
    label: 'Мониторинг',
    children: [
      { id: 'monitoring-load', label: 'Нагрузка', to: '/monitoring/load' },
    ],
  },
  {
    id: 'settings',
    label: 'Настройки',
    children: [
      { id: 'settings-profile', label: 'Профиль', to: '/settings/profile' },
    ],
  },
]
