import { messages as shellMessages } from './zh-CN'
import { messages as exampleMessages } from '@/features/examples/messages'

import { messages as accountMessages } from '@/features/account/messages'
import { messages as loginMessages } from '@/features/login/messages'

const messages: Record<string, string> = {
  ...shellMessages,
  ...exampleMessages,
  ...accountMessages,
  ...loginMessages
}

export const t = (key: string): string => messages[key] ?? key
