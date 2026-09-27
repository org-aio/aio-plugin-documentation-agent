import { messages as shellMessages } from './zh-CN'
import { messages as exampleMessages } from '@/features/examples/messages'

import { messages as accountMessages } from '@/features/account/messages'
import { messages as loginMessages } from '@/features/login/messages'
import { messages as ledgerMessages } from '@/features/ledger/messages'

const messages: Record<string, string> = {
  ...shellMessages,
  ...exampleMessages,
  ...accountMessages,
  ...loginMessages,
  ...ledgerMessages
}

export const t = (key: string): string => messages[key] ?? key
