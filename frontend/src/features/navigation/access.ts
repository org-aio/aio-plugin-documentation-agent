import { shallowRef } from 'vue'
import { collectAccessiblePaths, collectNoCachePaths, type MenuRecord } from './catalog.mjs'

// 后端菜单决定可访问路由；null 表示尚未加载或不做裁剪。
const accessiblePaths = shallowRef<Set<string> | null>(null)
const accessibleGroups = shallowRef<Set<string> | null>(null)
const noCachePaths = shallowRef<Set<string>>(new Set())

export const getAccessiblePaths = (): Set<string> | null => accessiblePaths.value
export const getNoCachePaths = (): Set<string> => noCachePaths.value

export const applyMenuAccess = (
  menus: readonly MenuRecord[],
  availablePaths: Iterable<string>
): void => {
  const paths = collectAccessiblePaths(menus, availablePaths)
  accessiblePaths.value = paths
  // 分组用于放行详情、字典数据等隐藏子页；只收集授权菜单覆盖到的分组。
  const groups = new Set<string>()
  for (const path of paths ?? []) {
    const segment = path.split('/').filter(Boolean)[0]
    if (segment) {
      groups.add(segment)
    }
  }
  accessibleGroups.value = paths === null ? null : groups
  noCachePaths.value = collectNoCachePaths(menus, availablePaths)
}

export const resetMenuAccess = (): void => {
  accessiblePaths.value = null
  accessibleGroups.value = null
  noCachePaths.value = new Set()
}

// 登录页、首页、个人中心和 404 兜底不依赖菜单授权。
const ALWAYS_ALLOWED = new Set(['/login', '/home', '/user/profile'])

export const isPathAccessible = (path: string, group?: string, hidden = false): boolean => {
  if (ALWAYS_ALLOWED.has(path) || path === '/:pathMatch(.*)*') {
    return true
  }
  const allowed = accessiblePaths.value
  if (allowed === null) {
    return true
  }
  if (allowed.has(path)) {
    return true
  }
  // 只有隐藏的详情/字典子页跟随所属菜单分组授权，避免放行同组其它未授权页面。
  if (!hidden || !group) {
    return false
  }
  return accessibleGroups.value?.has(group) ?? false
}
