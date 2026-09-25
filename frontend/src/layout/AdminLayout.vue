<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, provide, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Expand, Fold, FullScreen, Moon, Setting, Sunny } from '@element-plus/icons-vue'
import { appConfig, settings } from '@/config'
import { t } from '@/locales'
import { availablePagePaths } from '@/router'
import request from '@/config/axios'
import { accountState, loadProfile } from '@/features/account/account'
import { setDictMap } from '@/utils/dict'
import {
  buildNavigation,
  type MenuRecord,
  type NavigationItem
} from '@/features/navigation/catalog.mjs'
import aioPages from '@/features/navigation/aio-pages.json'
import { filterMenusByRoutes } from '@/features/navigation/aio-route.mjs'
import { applyMenuAccess, getNoCachePaths, resetMenuAccess } from '@/features/navigation/access'
import { MENUS_CHANGED_EVENT } from '@/features/navigation/events'
import UserMenu from './UserMenu.vue'
import NavigationMenu from './NavigationMenu.vue'
import TagsView from './TagsView.vue'
import { iconFor } from './menuIcons'
import Logo from './Logo.vue'
import ProjectSwitcher from './ProjectSwitcher.vue'
import SettingsDrawer from './SettingsDrawer.vue'

interface PageTab {
  path: string
  title: string
  icon: string
}

const navigation = ref<NavigationItem[]>([])
const navigationError = ref('')
// 真实字典只加载一次；演示模式仍使用本地枚举，不请求后端。
let dictLoaded = false
const loadDictionaries = async () => {
  if (dictLoaded || appConfig.dataMode === 'demo') {
    return
  }
  try {
    const list = await request.get<unknown[]>({ url: '/system/dict-data/simple-list' })
    setDictMap(list)
    dictLoaded = true
  } catch (error) {
    // 字典失败不应阻断路由；保留枚举默认值，下次进入布局时重试。
    console.error('字典加载失败，保留默认枚举。', error)
  }
}
const refreshNavigation = async () => {
  try {
    await loadProfile()
    await loadDictionaries()
    const menus =
      appConfig.dataMode === 'demo'
        ? await request.get<MenuRecord[]>({ url: '/system/menu/list' })
        : accountState.permissionInfo?.menus ?? []
    const businessMenus = filterMenusByRoutes(
      menus,
      aioPages.map((page) => page.route)
    )
    applyMenuAccess(businessMenus, availablePagePaths)
    navigation.value = buildNavigation(businessMenus, availablePagePaths)
    navigationError.value = ''
  } catch (error) {
    resetMenuAccess()
    navigationError.value = error instanceof Error ? error.message : t('app.navigationFailed')
  }
}
const refreshNavigationInBackground = () => {
  void refreshNavigation()
}

const route = useRoute()
const router = useRouter()
const title = computed(() => settings.title.trim() || appConfig.title)
const pageTitle = computed(() => t(String(route.meta.titleKey || 'app.home')))
const settingsVisible = ref(false)
const mobileMenuOpen = ref(false)
const isMobile = ref(false)
const fullscreen = ref(false)
const refreshVersion = ref(0)
const collapsed = computed(() => settings.collapsed && !isMobile.value)
const tabs = ref<PageTab[]>([{ path: '/home', title: t('app.home'), icon: 'home' }])
// group 是父级路径（如 boxun/generated-data），不能直接拼翻译键；优先用 groupTitleKey。
const pageGroup = computed(() => {
  const titleKey = route.meta.groupTitleKey as string | undefined
  if (titleKey) {
    return t(titleKey)
  }
  return route.meta.group ? t(`app.${route.meta.group}`) : ''
})
const isSystemPage = computed(() => ['system', 'infra'].includes(String(route.meta.group)))
// 菜单 keepAlive:false 的页面禁用缓存；首页与个人中心始终缓存。
const cacheablePage = computed(() => {
  if (route.path === '/home' || route.path === '/user/profile') {
    return true
  }
  return !getNoCachePaths().has(route.path)
})

provide('openSettings', () => {
  settingsVisible.value = true
})

watch(
  () => route.fullPath,
  () => {
    mobileMenuOpen.value = false
    if (
      !route.name ||
      (route.meta.hidden && !route.meta.showTab) ||
      tabs.value.some((tab) => tab.path === route.fullPath)
    ) {
      return
    }
    tabs.value.push({
      path: route.fullPath,
      title: pageTitle.value,
      icon: String(route.meta.icon || 'page')
    })
  },
  { immediate: true }
)

watch(
  [title, pageTitle],
  () => {
    document.title = `${pageTitle.value} - ${title.value}`
  },
  { immediate: true }
)

const toggleMenu = () => {
  if (isMobile.value) {
    mobileMenuOpen.value = !mobileMenuOpen.value
    return
  }
  settings.collapsed = !settings.collapsed
}

const closeTab = (path: string) => {
  const index = tabs.value.findIndex((tab) => tab.path === path)
  tabs.value = tabs.value.filter((tab) => tab.path !== path)
  if (route.fullPath === path) {
    const nextTab = tabs.value[Math.max(index - 1, 0)]
    void router.push(nextTab?.path || '/home')
  }
}

const handleTabAction = (command: string, path: string) => {
  if (command === 'refresh') {
    refreshVersion.value += 1
    void router.push(path)
    return
  }
  if (command === 'closeCurrent') {
    closeTab(path)
    return
  }
  const selected = tabs.value.findIndex((tab) => tab.path === path)
  tabs.value = tabs.value.filter((tab, index) => {
    if (tab.path === '/home') return true
    if (command === 'closeOther') return tab.path === path
    if (command === 'closeLeft') return index >= selected
    if (command === 'closeRight') return index <= selected
    return command !== 'closeAll'
  })
  if (!tabs.value.some((tab) => tab.path === route.fullPath)) {
    void router.push(tabs.value.at(-1)?.path || '/home')
  }
}

const toggleFullscreen = async () => {
  if (document.fullscreenElement) {
    await document.exitFullscreen()
    return
  }
  await document.documentElement.requestFullscreen()
}

const updateViewport = () => {
  isMobile.value = window.matchMedia('(max-width: 767px)').matches
}
const updateFullscreen = () => {
  fullscreen.value = Boolean(document.fullscreenElement)
}
const closeMobileMenu = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    mobileMenuOpen.value = false
  }
}

onMounted(() => {
  refreshNavigationInBackground()
  window.addEventListener(MENUS_CHANGED_EVENT, refreshNavigationInBackground)
  updateViewport()
  window.addEventListener('resize', updateViewport)
  window.addEventListener('keydown', closeMobileMenu)
  document.addEventListener('fullscreenchange', updateFullscreen)
})
onBeforeUnmount(() => {
  window.removeEventListener(MENUS_CHANGED_EVENT, refreshNavigationInBackground)
  window.removeEventListener('resize', updateViewport)
  window.removeEventListener('keydown', closeMobileMenu)
  document.removeEventListener('fullscreenchange', updateFullscreen)
})
</script>

<template>
  <div class="admin-layout" :class="{ 'is-collapsed': collapsed, 'menu-open': mobileMenuOpen }">
    <a class="skip-link" href="#main-content">{{ t('app.skipContent') }}</a>
    <button
      v-if="isMobile && mobileMenuOpen"
      class="menu-backdrop"
      :aria-label="t('app.closeMenu')"
      @click="mobileMenuOpen = false"
    />
    <aside class="app-sidebar" :inert="isMobile && !mobileMenuOpen">
      <Logo v-if="settings.showLogo" :collapsed="collapsed" />
      <nav :aria-label="t('app.navigation')" class="app-navigation">
        <ElMenu
          :default-active="route.path"
          :collapse="collapsed"
          :collapse-transition="false"
          :default-openeds="[route.meta.group ? `/${route.meta.group}` : '/system']"
          :unique-opened="settings.uniqueOpened"
          router
        >
          <NavigationMenu :items="navigation" />
        </ElMenu>
      </nav>
    </aside>

    <div class="app-main">
      <header class="app-tool-header">
        <div class="header-navigation">
          <button class="icon-button" :aria-label="t('app.menu')" @click="toggleMenu">
            <ElIcon><Expand v-if="collapsed" /><Fold v-else /></ElIcon>
          </button>
          <ElBreadcrumb v-if="settings.showBreadcrumb" separator="/" class="app-breadcrumb">
            <ElBreadcrumbItem :to="{ path: '/home' }"
              ><ElIcon class="breadcrumb-icon"><component :is="iconFor('home')" /></ElIcon
              >{{ t('app.home') }}</ElBreadcrumbItem
            >
            <ElBreadcrumbItem v-if="pageGroup">{{ pageGroup }}</ElBreadcrumbItem>
            <ElBreadcrumbItem v-if="route.path !== '/home'">{{ pageTitle }}</ElBreadcrumbItem>
          </ElBreadcrumb>
        </div>
        <div class="header-actions">
          <ProjectSwitcher />
          <button
            class="icon-button"
            :aria-label="t(settings.dark ? 'app.lightMode' : 'app.darkMode')"
            :title="t(settings.dark ? 'app.lightMode' : 'app.darkMode')"
            @click="settings.dark = !settings.dark"
          >
            <ElIcon><Sunny v-if="settings.dark" /><Moon v-else /></ElIcon>
          </button>
          <button
            v-if="!isMobile"
            class="icon-button"
            :aria-label="t(fullscreen ? 'app.exitFullscreen' : 'app.fullscreen')"
            :title="t(fullscreen ? 'app.exitFullscreen' : 'app.fullscreen')"
            @click="toggleFullscreen"
          >
            <ElIcon><FullScreen /></ElIcon>
          </button>
          <button
            class="icon-button settings-button"
            :aria-label="t('app.settings')"
            :title="t('app.settings')"
            @click="settingsVisible = true"
          >
            <ElIcon><Setting /></ElIcon>
          </button>
          <UserMenu />
        </div>
      </header>

      <TagsView
        v-if="settings.showTagsView"
        :tabs="tabs"
        :show-icons="settings.showTagsIcon"
        :active-path="route.fullPath"
        @close="closeTab"
        @action="handleTabAction"
      />

      <main id="main-content" class="app-content" tabindex="-1">
        <ElAlert v-if="navigationError" :title="navigationError" type="error" :closable="false">
          <ElButton text @click="refreshNavigation">{{ t('app.retryNavigation') }}</ElButton>
        </ElAlert>
        <ElAlert
          v-if="isSystemPage && appConfig.dataMode === 'demo'"
          class="system-demo-notice"
          :title="t('system.demoNotice')"
          type="info"
          :closable="false"
          show-icon
        />
        <RouterView v-slot="{ Component }">
          <KeepAlive :max="12">
            <component
              :is="Component"
              v-if="cacheablePage"
              :key="`${route.fullPath}:${refreshVersion}`"
            />
          </KeepAlive>
          <component
            :is="Component"
            v-if="!cacheablePage"
            :key="`${route.fullPath}:${refreshVersion}`"
          />
        </RouterView>
      </main>
      <footer v-if="settings.showFooter" class="app-footer">
        {{ settings.footer }} · © {{ new Date().getFullYear() }} {{ title }}
      </footer>
    </div>
    <SettingsDrawer v-model="settingsVisible" />
  </div>
</template>
