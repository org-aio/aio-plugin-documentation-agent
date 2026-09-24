import { reactive, ref, watch } from 'vue'
import configuration from '../app.config.json'
import { normalizeLoginConfiguration, type LoginConfiguration } from './features/login/config.mjs'

export const appConfig = Object.freeze({
  ...configuration,
  login: normalizeLoginConfiguration((configuration as { login?: unknown }).login)
})

export interface AppSettings {
  login: LoginConfiguration
  title: string
  logo: string
  themeColor: string
  dark: boolean
  collapsed: boolean
  showTagsView: boolean
  showTagsIcon: boolean
  showLogo: boolean
  showBreadcrumb: boolean
  uniqueOpened: boolean
  showFooter: boolean
  footer: string
}

const storageKey = `admin-shell:${appConfig.appId}:settings:v1`
const defaults: AppSettings = {
  login: { ...appConfig.login },
  title: appConfig.title,
  logo: appConfig.logo,
  themeColor: appConfig.themeColor,
  dark: false,
  collapsed: false,
  showTagsView: appConfig.showTagsView,
  showTagsIcon: true,
  showLogo: true,
  showBreadcrumb: true,
  uniqueOpened: true,
  showFooter: appConfig.showFooter,
  footer: appConfig.footer
}

const readSettings = (): AppSettings => {
  try {
    const saved: unknown = JSON.parse(localStorage.getItem(storageKey) || 'null')
    if (!saved || typeof saved !== 'object') {
      return { ...defaults, login: { ...defaults.login } }
    }
    const values = saved as Record<string, unknown>
    const result = { ...defaults }
    for (const key of Object.keys(defaults) as (keyof AppSettings)[]) {
      if (key === 'login') {
        continue
      }
      if (typeof values[key] === typeof defaults[key]) {
        Object.assign(result, { [key]: values[key] })
      }
    }
    if (result.logo === '/logo.svg') {
      result.logo = defaults.logo
    }
    result.login = normalizeLoginConfiguration(values.login, defaults.login)
    return result
  } catch (error) {
    console.warn('无法读取应用偏好，将使用默认配置。', error)
    return { ...defaults, login: { ...defaults.login } }
  }
}

export const settings = reactive<AppSettings>(readSettings())
export const settingsSaveState = ref<'saved' | 'error'>('saved')

export const resetSettings = () => {
  Object.assign(settings, defaults, { login: { ...defaults.login } })
}

// 本地图片跟随部署前缀；外部图片和内嵌图片保持原地址。
export const resolveAssetUrl = (value: string): string => {
  if (/^(https?:\/\/|data:image\/|blob:)/i.test(value)) {
    return value
  }
  return `${import.meta.env.BASE_URL}${value.replace(/^\/+/, '')}`
}

const mixColor = (color: string, target: number, weight: number) => {
  const channels = [1, 3, 5].map((offset) => {
    const channel = Number.parseInt(color.slice(offset, offset + 2), 16)
    return Math.round(channel + (target - channel) * weight)
      .toString(16)
      .padStart(2, '0')
  })
  return `#${channels.join('')}`
}

watch(
  settings,
  (value) => {
    const root = document.documentElement
    root.classList.toggle('dark', value.dark)
    root.style.colorScheme = value.dark ? 'dark' : 'light'
    const primary = /^#[\da-f]{6}$/i.test(value.themeColor) ? value.themeColor : '#409eff'
    root.style.setProperty('--el-color-primary', primary)
    for (const level of [3, 5, 7, 8, 9]) {
      const color = mixColor(primary, value.dark ? 20 : 255, level / 10)
      root.style.setProperty(`--el-color-primary-light-${level}`, color)
    }
    root.style.setProperty('--el-color-primary-dark-2', mixColor(primary, 0, 0.2))
    const favicon = document.querySelector<HTMLLinkElement>('link[rel="icon"]')
    if (favicon) {
      favicon.href = resolveAssetUrl(value.logo || appConfig.logo)
    }
    try {
      localStorage.setItem(storageKey, JSON.stringify(value))
      settingsSaveState.value = 'saved'
    } catch (error) {
      settingsSaveState.value = 'error'
      console.warn('无法保存应用偏好，本次设置仅在当前页面生效。', error)
    }
  },
  { deep: true, immediate: true, flush: 'sync' }
)
