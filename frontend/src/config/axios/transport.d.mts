import type { DemoTransport } from '../../features/system-demo/model.mjs'
export interface RequestOptions {
  url: string
  mode?: 'api'
  params?: object
  data?: unknown
  headers?: HeadersInit
  signal?: AbortSignal
}
export interface Session {
  accessToken?: string
  refreshToken?: string
  tenantId?: string | number
}
export type RequestClient = Record<
  'get' | 'post' | 'postOriginal' | 'put' | 'delete' | 'download' | 'upload',
  (options: RequestOptions) => Promise<unknown>
>
export function createRequestClient(options: {
  mode: string
  apiBase: string
  session: () => Session
  demo: DemoTransport
  refreshSession?: () => Promise<boolean>
  fetcher?: typeof fetch
  bridge?: {
    request: (payload: {
      method: string
      path: string
      query?: string | null
      body?: string
    }) => Promise<{ status: number; body: string; content_type?: string }>
  }
}): RequestClient
