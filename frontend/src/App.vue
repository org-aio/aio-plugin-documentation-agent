<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AdminLayout from '@/layout/AdminLayout.vue'
import { ensureHostSession, hasSession, sessionRevision } from '@/utils/auth'
import { t } from '@/locales'
import { isAioEmbedded } from '@/features/navigation/aio-route.mjs'

const route = useRoute()
const router = useRouter()
const embedded = computed(() => isAioEmbedded(globalThis.window))
const ready = ref(hasSession() || route.meta.public === true || embedded.value)
const routeReady = ref(false)
const routeError = ref('')
const sessionError = ref('')
const reloadPage = (): void => globalThis.location.reload()
const connectHost = async (): Promise<void> => {
  sessionError.value = ''
  try {
    if (!(await ensureHostSession()) && !hasSession() && !route.meta.public) {
      sessionError.value = t('account.hostSessionFailed')
    }
  } catch (error) {
    sessionError.value = error instanceof Error ? error.message : t('account.hostSessionFailed')
  } finally {
    ready.value = true
  }
}
// 宿主页面直接挂载，只有页面真正发业务请求时才初始化会话。
if (!embedded.value) {
  void connectHost()
}
void router
  .isReady()
  .then(() => {
    routeReady.value = true
  })
  .catch((error: unknown) => {
    routeError.value = error instanceof Error ? error.message : t('app.pageLoadFailed')
  })
watch(sessionRevision, () => {
  if (!hasSession() && !route.meta.public) {
    void router.replace({ path: '/login', query: { redirect: route.fullPath } })
  }
})
</script>

<template>
  <RouterView v-if="ready && routeReady && route.meta.public" />
  <RouterView v-else-if="ready && routeReady && embedded" />
  <AdminLayout v-else-if="ready && routeReady && hasSession()" :key="sessionRevision" />
  <main v-else-if="routeError" class="host-session-error">
    <ElResult icon="error" :title="t('app.pageLoadFailed')" :sub-title="routeError">
      <template #extra>
        <ElButton type="primary" @click="reloadPage">{{ t('app.retryLoad') }}</ElButton>
      </template>
    </ElResult>
  </main>
  <main v-else-if="!routeReady" class="app-page-loading" aria-live="polite" aria-busy="true">
    <div class="app-page-loading__panel">
      <div class="app-page-loading__title"></div>
      <div class="app-page-loading__line app-page-loading__line--wide"></div>
      <div class="app-page-loading__line"></div>
      <div class="app-page-loading__line app-page-loading__line--short"></div>
      <div class="app-page-loading__status">
        <span class="app-page-loading__spinner"></span>
        <span>{{ t('app.pageLoading') }}</span>
      </div>
    </div>
  </main>
  <main v-else-if="ready" class="host-session-error">
    <ElResult icon="warning" :title="t('account.hostSessionFailedTitle')" :sub-title="sessionError">
      <template #extra>
        <ElButton type="primary" @click="connectHost">{{ t('account.retry') }}</ElButton>
      </template>
    </ElResult>
  </main>
  <main v-else class="host-session-error" aria-live="polite">
    <ElResult icon="info" :title="t('account.hostSessionConnecting')" />
  </main>
</template>

<style scoped>
.host-session-error {
  display: grid;
  min-height: 100dvh;
  place-items: center;
  padding: 24px;
  background: var(--app-content-bg-color);
}

.app-page-loading {
  display: grid;
  min-height: 100dvh;
  place-items: center;
  padding: 24px;
  background: var(--app-content-bg-color);
}

.app-page-loading__panel {
  width: min(100%, 760px);
  padding: 24px;
  border: 1px solid var(--panel-border-color);
  border-radius: 8px;
  background: var(--panel-bg-color);
}

.app-page-loading__title,
.app-page-loading__line {
  border-radius: 4px;
  background: var(--el-fill-color);
  animation: app-page-loading-pulse 1.4s ease-in-out infinite;
}

.app-page-loading__title {
  width: 180px;
  height: 20px;
  margin-bottom: 24px;
}

.app-page-loading__line {
  width: 72%;
  height: 14px;
  margin-top: 14px;
}

.app-page-loading__line--wide {
  width: 100%;
}

.app-page-loading__line--short {
  width: 46%;
}

.app-page-loading__status {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-top: 28px;
  color: var(--el-text-color-secondary);
}

.app-page-loading__spinner {
  width: 18px;
  height: 18px;
  border: 2px solid var(--el-border-color);
  border-top-color: var(--el-color-primary);
  border-radius: 50%;
  animation: app-page-loading-spin 0.8s linear infinite;
}

@keyframes app-page-loading-pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.45;
  }
}

@keyframes app-page-loading-spin {
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .app-page-loading__title,
  .app-page-loading__line,
  .app-page-loading__spinner {
    animation: none;
  }
}
</style>
