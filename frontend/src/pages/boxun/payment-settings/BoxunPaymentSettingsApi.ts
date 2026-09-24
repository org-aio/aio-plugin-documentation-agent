// 支付设置为业务配置页面，放在手写目录以避免后续 Studio 前端生成覆盖。
// 数据落在平台的 infra_config，按约定 key 读写。
import request from '@/config/axios'

export type EntityId = string | number
export type EntityInput = Record<string, any>
export type EntityRecord = EntityInput & { id: EntityId }

export interface PageResult<T> {
  list: T[]
  total: number
}

/** 支付设置对应的配置键前缀。 */
export const PAYMENT_CONFIG_KEYS = {
  alipayEnabled: 'boxun.pay.alipay.enabled',
  alipayAppId: 'boxun.pay.alipay.app-id',
  alipayPrivateKey: 'boxun.pay.alipay.private-key',
  alipayPublicKey: 'boxun.pay.alipay.public-key',
  alipayGateway: 'boxun.pay.alipay.gateway',
  alipayNotifyUrl: 'boxun.pay.alipay.notify-url',
  priceList: 'boxun.pay.price-list'
} as const

/** 价目表条目。 */
export interface AccountPrice {
  accountType: number
  accountPrice: number
  durationDays: number
}

/** 支付设置表单。 */
export interface PaymentSettingsForm {
  alipayEnabled: boolean
  alipayAppId: string
  alipayPrivateKey: string
  alipayPublicKey: string
  alipayGateway: string
  alipayNotifyUrl: string
  priceList: AccountPrice[]
}

export const getConfigPage = (params: Record<string, unknown>) =>
  request.get<PageResult<EntityRecord>>({ url: '/infra/config/page', params })

export const createConfig = (data: EntityInput) =>
  request.post<EntityRecord>({ url: '/infra/config/create', data })

export const updateConfig = (data: EntityInput) =>
  request.put<number>({ url: '/infra/config/update', data })

/** 默认价目表：与原 boxun-boot 的 accountPriceList 写死数据保持一致。 */
export const DEFAULT_PRICE_LIST: AccountPrice[] = [
  { accountType: 0, accountPrice: 0, durationDays: 7 },
  { accountType: 1, accountPrice: 30, durationDays: 999 },
  { accountType: 2, accountPrice: 120, durationDays: 120 },
  { accountType: 3, accountPrice: 365, durationDays: 365 }
]

export const emptyPaymentSettings = (): PaymentSettingsForm => ({
  alipayEnabled: false,
  alipayAppId: '',
  alipayPrivateKey: '',
  alipayPublicKey: '',
  alipayGateway: '',
  alipayNotifyUrl: '',
  priceList: DEFAULT_PRICE_LIST.map((item) => ({ ...item }))
})
