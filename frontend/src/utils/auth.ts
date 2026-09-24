import { appConfig } from '@/config'
import { ref, shallowRef } from 'vue'
import { sameSessionIdentity, sessionIsActive } from '@/features/account/session.mjs'
export { safeRedirect } from '@/features/account/session.mjs'

export interface SessionState {
  mode?: 'demo' | 'api'
  signedOut?: boolean
  userId?: number
  accessToken?: string
  refreshToken?: string
  tenantId?: string | number
  permissions?: string[]
  roles?: string[]
}

export const SESSION_STORAGE_KEY = `${appConfig.appId}:session`

const readSession = (): SessionState => {
  try {
    const value: unknown = JSON.parse(localStorage.getItem(SESSION_STORAGE_KEY) || '{}')
    return value && typeof value === 'object' && !Array.isArray(value)
      ? (value as SessionState)
      : {}
  } catch (error) {
    console.warn('无法读取登录会话。', error)
    return { signedOut: true }
  }
}

const session = shallowRef<SessionState>(readSession())
export const sessionRevision = ref(0)
export const getSessionState = (): SessionState => session.value
export const hasSession = (): boolean => sessionIsActive(appConfig.dataMode, session.value)
export const getAccessToken = (): string => getSessionState().accessToken ?? ''
export const getRefreshToken = (): string => getSessionState().refreshToken ?? ''
export const getTenantId = (): string => String(getSessionState().tenantId ?? '')

const writeSession = (value: SessionState): void => {
  // 先持久化再发布状态；存储失败必须让调用方知晓，避免刷新后意外恢复会话。
  localStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(value))
  session.value = value
  sessionRevision.value += 1
}

export const saveApiSession = (value: SessionState): void => {
  writeSession({ ...value, mode: 'api', signedOut: false })
}

// AIO 已在宿主完成身份校验；首次打开插件时用宿主上下文换取插件会话。
export const ensureHostSession = async (): Promise<boolean> => {
  if (hasSession()) {
    return true
  }
  const bridge = (globalThis.window as Window & {
    aioPlugin?: {
      request: (payload: {
        method: string
        path: string
        query?: string | null
        body?: string
      }) => Promise<{ status: number; body: string }>
    }
  })?.aioPlugin
  if (!bridge) {
    return false
  }
  try {
    const response = await bridge.request({
      method: 'POST',
      path: '/system/auth/host-login',
      query: null,
      body: ''
    })
    const payload = response.body ? JSON.parse(response.body) : null
    const token = payload?.data
    if (response.status < 200 || response.status >= 300 || payload?.code !== 0) {
      return false
    }
    if (typeof token?.accessToken !== 'string' || !token.accessToken) {
      return false
    }
    saveApiSession({
      accessToken: token.accessToken,
      refreshToken: token.refreshToken,
      userId: token.userId
    })
    return true
  } catch (error) {
    console.warn('宿主会话自动接入失败。', error)
    return false
  }
}

export const updateSessionPermissions = (permissions: string[], roles: string[]): void => {
  if (
    JSON.stringify(session.value.permissions ?? []) === JSON.stringify(permissions) &&
    JSON.stringify(session.value.roles ?? []) === JSON.stringify(roles)
  ) {
    return
  }
  const value = { ...session.value, permissions, roles }
  localStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(value))
  session.value = value
}

// 刷新成功后只替换令牌字段，保留权限、角色与租户身份，避免页头身份闪空。
// 这里不递增 sessionRevision：令牌续期不是身份切换，进行中的资料请求不应被判为会话变更。
export const updateSessionTokens = (accessToken: string, refreshToken?: string): void => {
  const value = {
    ...session.value,
    accessToken,
    refreshToken: refreshToken ?? session.value.refreshToken
  }
  localStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(value))
  session.value = value
}

export const invalidateSession = (): void => {
  try {
    writeSession({ signedOut: true })
  } catch (error) {
    session.value = { signedOut: true }
    sessionRevision.value += 1
    throw error
  }
}

window.addEventListener('storage', (event) => {
  if (event.key === SESSION_STORAGE_KEY || event.key === null) {
    const next = readSession()
    const identityChanged = !sameSessionIdentity(session.value, next)
    session.value = next
    if (identityChanged) {
      sessionRevision.value += 1
    }
  }
})
