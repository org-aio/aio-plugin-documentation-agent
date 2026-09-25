import type { Component } from 'vue'
import {
  createMemoryHistory,
  createRouter,
  createWebHashHistory,
  type RouteRecordRaw
} from 'vue-router'

import { pageMetadata } from '@/features/navigation/catalog.mjs'
import aioPages from '@/features/navigation/aio-pages.json'
import {
  createAioEntryResolver,
  isAioEmbedded,
  needsAioEntryRedirect,
  resolveAioDocumentRoute,
  shouldUseAioMemoryHistory
} from '@/features/navigation/aio-route.mjs'
import { isPathAccessible } from '@/features/navigation/access'
import { ensureHostSession, hasSession, safeRedirect } from '@/utils/auth'

const pages = {
  ...import.meta.glob<{ default: Component }>('/src/pages/**/*.vue'),
  // 元数据生成页面物化在 src/views/generated，页面仍由同一套文件路由与权限守卫消费。
  ...import.meta.glob<{ default: Component }>('/src/views/generated/**/*.vue'),
  ...import.meta.glob<{ default: Component }>('/src/page-packages/*/src/views/**/index.vue')
}
const routes: RouteRecordRaw[] = Object.entries(pages)
  .filter(([file]) => !file.includes('/components/'))
  .sort(([left], [right]) => left.localeCompare(right))
  .map(([file, component]) => {
    const path =
      file
        .replace(/^\/src\/page-packages\/[^/]+\/src\/views/, '')
        .replace('/src/views', '')
        .replace('/src/pages', '')
        .replace(/\.vue$/, '')
        .replace(/\/index$/, '') || '/'
    if (path === '/[...path]') {
      return {
        path: '/:pathMatch(.*)*',
        name: 'not-found',
        component,
        meta: { titleKey: 'error.title', hidden: true }
      }
    }
    const metadata =
      path === '/login'
        ? { titleKey: 'account.login', hidden: true, public: true }
        : path === '/user/profile'
          ? { titleKey: 'account.personalCenter', icon: 'user', hidden: true, showTab: true }
          : pageMetadata[path] ?? { titleKey: path.slice(1), icon: 'page', order: 100 }
    return {
      path: path === '/system/dict/data' ? '/dict/type/data/:dictType' : path,
      name: path.slice(1),
      component,
      meta: { ...metadata }
    }
  })

const registeredPaths = new Set<string>()
for (const route of routes) {
  if (registeredPaths.has(route.path)) {
    throw new Error(`页面路由冲突：${route.path}`)
  }
  registeredPaths.add(route.path)
}

export const availablePagePaths = routes.map((route) => route.path)

const resolveAioEntryRoute = createAioEntryResolver(aioPages)
const embedded = isAioEmbedded(window)

export const router = createRouter({
  history: shouldUseAioMemoryHistory(window)
    ? createMemoryHistory(import.meta.env.BASE_URL)
    : createWebHashHistory(import.meta.env.BASE_URL),
  routes: [{ path: '/', redirect: '/home' }, ...routes],
  scrollBehavior: () => ({ top: 0 })
})

// 登录页在壳外展示；退出后直接访问旧地址也必须重新登录。
router.beforeEach(async (to) => {
  const entryRoute =
    resolveAioDocumentRoute(document, aioPages) ??
    resolveAioEntryRoute(window.location.pathname, window.location.search)
  if (entryRoute && needsAioEntryRedirect(to, entryRoute)) {
    const target = router.resolve(entryRoute)
    return { path: target.path, query: target.query, hash: target.hash, replace: true }
  }
  if (!to.meta.public && !hasSession() && !embedded) {
    await ensureHostSession()
  }
  if (!to.meta.public && !hasSession() && !embedded) {
    return { path: '/login', query: { redirect: to.fullPath }, replace: true }
  }
  if (to.path === '/login' && hasSession()) {
    return safeRedirect(to.query.redirect)
  }
  // 菜单加载完成后裁剪未授权路由；服务端接口权限仍是最终边界。
  if (
    !to.meta.public &&
    !isPathAccessible(to.path, to.meta.group as string | undefined, to.meta.hidden === true)
  ) {
    return { path: '/home', replace: true }
  }
})
