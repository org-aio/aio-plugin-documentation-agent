import { appConfig } from '@/config'
import { safeLocalStorage } from '@/utils/safeStorage'

export const CACHE_KEY = {
  USER: 'user',
  ROLE_ROUTERS: 'role-routers',
  DICT_CACHE: 'dicts'
} as const

const storageKey = (key: string) => `${appConfig.appId}:page-cache:${key}`

export const useCache = () => ({
  wsCache: {
    get: <T = unknown>(key: string): T | null => {
      const value = safeLocalStorage.getItem(storageKey(key))
      return value === null ? null : (JSON.parse(value) as T)
    },
    set: (key: string, value: unknown) => {
      safeLocalStorage.setItem(storageKey(key), JSON.stringify(value))
    },
    delete: (key: string) => {
      safeLocalStorage.removeItem(storageKey(key))
    }
  }
})
