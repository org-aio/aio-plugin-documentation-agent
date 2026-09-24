import { t as shellTranslate } from '@/locales'
import { messages } from '@/compat/messages'

export const useI18n = () => ({
  t: (key: string): string => messages[key] ?? shellTranslate(key)
})
