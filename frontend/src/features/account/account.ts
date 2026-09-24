import { reactive, watch } from 'vue'
import request from '@/config/axios'
import { appConfig } from '@/config'
import {
  hasSession,
  invalidateSession,
  saveApiSession,
  sessionRevision,
  updateSessionPermissions
} from '@/utils/auth'
import { t } from '@/locales'
import { buildAvatarUploadForm, resolveApiAssetUrl } from '@/features/account/profileUpload.mjs'
import type { MenuRecord } from '@/features/navigation/catalog.mjs'

export interface AccountProfile {
  id: number
  username: string
  nickname: string
  email: string
  mobile: string
  sex: number
  avatar: string
  createTime?: string | number
  dept?: { id?: number; name: string } | null
  roles?: { id?: number; name: string; code?: string }[]
  posts?: { id?: number; name: string }[]
}

export type ProfileChanges = Pick<AccountProfile, 'nickname' | 'email' | 'mobile' | 'sex'> & {
  avatar: string
  avatarFile?: File | null
}

export interface PermissionInfo {
  user?: AccountProfile
  roles: string[]
  permissions: string[]
  menus: MenuRecord[]
}

export const accountState = reactive<{
  profile: AccountProfile | null
  permissionInfo: PermissionInfo | null
  loading: boolean
}>({ profile: null, permissionInfo: null, loading: false })

let pendingProfile: Promise<AccountProfile | null> | undefined

const sessionExpired = (error: unknown): boolean => {
  const response = error as { status?: number; code?: string | number }
  return response.status === 401 || Number(response.code) === 401
}

// 会话切换立即清空身份信息；慢请求不得把已退出用户重新写回页头。
watch(
  sessionRevision,
  () => {
    accountState.profile = null
    accountState.permissionInfo = null
    accountState.loading = false
    pendingProfile = undefined
  },
  { flush: 'sync' }
)

export const loadProfile = (): Promise<AccountProfile | null> => {
  if (!hasSession()) {
    return Promise.resolve(null)
  }
  if (pendingProfile) {
    return pendingProfile
  }
  const revision = sessionRevision.value
  accountState.loading = true
  const pending = (async () => {
    const [profile, permissionInfo] = await Promise.all([
      request.get<AccountProfile>({ url: '/system/user/profile/get' }),
      request.get<PermissionInfo>({ url: '/system/auth/get-permission-info' })
    ])
    if (revision !== sessionRevision.value || !hasSession()) {
      return null
    }
    const permissions = Array.isArray(permissionInfo.permissions) ? permissionInfo.permissions : []
    const roles = Array.isArray(permissionInfo.roles) ? permissionInfo.roles : []
    updateSessionPermissions(permissions, roles)
    if (profile.avatar) {
      profile.avatar = resolveApiAssetUrl(appConfig.apiBase, profile.avatar)
    }
    accountState.profile = profile
    accountState.permissionInfo = {
      ...permissionInfo,
      permissions,
      roles,
      menus: permissionInfo.menus ?? []
    }
    return profile
  })()
    .catch((error: unknown) => {
      if (revision === sessionRevision.value && sessionExpired(error)) {
        invalidateSession()
      }
      throw error
    })
    .finally(() => {
      if (revision === sessionRevision.value) {
        accountState.loading = false
        pendingProfile = undefined
      }
    })
  pendingProfile = pending
  return pending
}

export const login = async (credentials: {
  username: string
  password: string
  tenantId?: string
  captchaVerification?: string
}): Promise<void> => {
  const revision = sessionRevision.value
  const token = await request.post<{
    accessToken: string
    refreshToken?: string
    userId?: number
    tenantId?: number | string
  }>({
    url: '/system/auth/login',
    data: {
      username: credentials.username,
      password: credentials.password,
      captchaVerification: credentials.captchaVerification
    },
    headers: credentials.tenantId ? { 'tenant-id': credentials.tenantId } : undefined
  })
  if (typeof token?.accessToken !== 'string' || !token.accessToken.trim()) {
    throw new Error(t('account.missingToken'))
  }
  if (revision !== sessionRevision.value) {
    throw new Error(t('account.sessionChanged'))
  }
  saveApiSession({
    accessToken: token.accessToken,
    refreshToken: token.refreshToken,
    userId: token.userId,
    tenantId: token.tenantId ?? credentials.tenantId
  })
  const loginRevision = sessionRevision.value
  try {
    const profile = await loadProfile()
    if (!profile) {
      throw new Error(t('account.sessionChanged'))
    }
  } catch (error) {
    if (loginRevision === sessionRevision.value) {
      invalidateSession()
    }
    throw error
  }
}

export const logout = async (): Promise<void> => {
  const revision = sessionRevision.value
  try {
    if (hasSession()) {
      await request.post({ url: '/system/auth/logout' })
    }
  } catch (error) {
    if (!sessionExpired(error)) {
      throw error
    }
  }
  if (revision === sessionRevision.value) {
    invalidateSession()
  }
}

export const updateProfile = async (changes: ProfileChanges): Promise<void> => {
  const revision = sessionRevision.value
  const { nickname, email, mobile, sex } = changes
  let avatar = changes.avatar
  // 服务端只接受文件地址；本机图片先保留预览，确认保存时再上传原始文件。
  if (changes.avatarFile) {
    const data = buildAvatarUploadForm(changes.avatarFile)
    const uploaded = await request.upload<{ data: string }>({
      url: '/infra/file/upload',
      mode: 'api',
      data
    })
    if (typeof uploaded.data !== 'string' || !uploaded.data) {
      throw new Error(t('account.avatarUploadFailed'))
    }
    avatar = uploaded.data
  }
  if (revision !== sessionRevision.value || !hasSession()) {
    return
  }
  await request.put({
    url: '/system/user/profile/update',
    data: { nickname, email, mobile, sex, avatar }
  })
  if (revision === sessionRevision.value && hasSession()) {
    await loadProfile()
  }
}

export const updatePassword = async (oldPassword: string, newPassword: string): Promise<void> => {
  await request.put({
    url: '/system/user/profile/update-password',
    data: { oldPassword, newPassword }
  })
}
