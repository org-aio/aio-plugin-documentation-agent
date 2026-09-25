/* eslint-disable @typescript-eslint/no-explicit-any */
// 兼容被直接提取的上游请求方法；调用方可显式提供返回类型。
import { ElMessage } from 'element-plus'
import { appConfig } from '@/config'
import { createDemoTransport } from '@/features/system-demo/model.mjs'
import { createRequestClient, type RequestOptions } from './transport.mjs'

import {
  ensureHostSession,
  getRefreshToken,
  getSessionState,
  invalidateSession,
  sessionRevision,
  updateSessionTokens
} from '@/utils/auth'
import { createSessionRefresher } from '@/features/account/refresh.mjs'
import { MENUS_CHANGED_EVENT } from '@/features/navigation/events'
import { safeLocalStorage } from '@/utils/safeStorage'

export { SESSION_STORAGE_KEY } from '@/utils/auth'

const client = createRequestClient({
  mode: appConfig.dataMode,
  apiBase: appConfig.apiBase,
  session: getSessionState,
  ensureSession: ensureHostSession,
  refreshSession: createSessionRefresher({
    apiBase: appConfig.apiBase,
    getRefreshToken,
    onRefreshed: (accessToken, refreshToken) => updateSessionTokens(accessToken, refreshToken)
  }),
  demo: createDemoTransport({
    storage: safeLocalStorage,
    storageKey: `${appConfig.appId}:system-demo:v1`
  })
})

const invoke = async <T>(method: keyof typeof client, options: RequestOptions): Promise<T> => {
  const requestSession = sessionRevision.value
  try {
    const result = (await client[method](options)) as T
    if (['post', 'put', 'delete'].includes(method) && options.url.startsWith('/system/menu/')) {
      window.dispatchEvent(new Event(MENUS_CHANGED_EVENT))
    }
    return result
  } catch (error) {
    if (error instanceof Error && error.name === 'AbortError') {
      throw error
    }
    const failure = error as { status?: number; code?: number | string }
    if (
      requestSession === sessionRevision.value &&
      (failure.status === 401 || Number(failure.code) === 401)
    )
      invalidateSession()
    ElMessage.error(error instanceof Error ? error.message : '请求失败，请重试。')
    throw error
  }
}

const request = {
  get: <T = any>(options: RequestOptions) => invoke<T>('get', options),
  post: <T = any>(options: RequestOptions) => invoke<T>('post', options),
  postOriginal: <T = any>(options: RequestOptions) => invoke<T>('postOriginal', options),
  put: <T = any>(options: RequestOptions) => invoke<T>('put', options),
  delete: <T = any>(options: RequestOptions) => invoke<T>('delete', options),
  download: (options: RequestOptions) => invoke<Blob>('download', options),
  upload: <T = any>(options: RequestOptions) => invoke<T>('upload', options)
}

export default request
