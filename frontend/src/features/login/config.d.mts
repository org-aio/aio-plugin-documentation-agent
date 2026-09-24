export interface LoginConfiguration {
  layout: 'split' | 'centered'
  panelPosition: 'left' | 'right'
  backgroundType: 'gradient' | 'solid' | 'image'
  backgroundColor: string
  backgroundImage: string
  backgroundOverlay: number
  heroTitle: string
  heroDescription: string
  heroImage: string
  formTitle: string
  formDescription: string
  brandImage: string
  showBrand: boolean
  showHero: boolean
  showTenant: boolean
  rememberAccount: boolean
  showThemeToggle: boolean
  showFooter: boolean
  footer: string
  showQrcode: boolean
  qrcodeImage: string
  qrcodeLink: string
  qrcodeLabel: string
  qrcodeDescription: string
}
export const DEFAULT_LOGIN_CONFIGURATION: Readonly<LoginConfiguration>
export function normalizeLoginConfiguration(value?: unknown, defaults?: unknown): LoginConfiguration
export function validateLoginConfiguration(value: unknown): LoginConfiguration
