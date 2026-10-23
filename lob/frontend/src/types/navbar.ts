export interface NavItem {
  id: string
  label: string
  to?: string
  children?: NavItem[]
}
