<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AdminLayout from '@/layout/AdminLayout.vue'
import StartupLoading from '@/components/StartupLoading.vue'
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
  <Suspense v-if="ready && routeReady && route.meta.public">
    <RouterView />
    <template #fallback>
      <StartupLoading />
    </template>
  </Suspense>
  <Suspense v-else-if="ready && routeReady && embedded">
    <RouterView />
    <template #fallback>
      <StartupLoading />
    </template>
  </Suspense>
  <AdminLayout v-else-if="ready && routeReady && hasSession()" :key="sessionRevision" />
  <main v-else-if="routeError" class="host-session-error">
    <ElResult icon="error" :title="t('app.pageLoadFailed')" :sub-title="routeError">
      <template #extra>
        <ElButton type="primary" @click="reloadPage">{{ t('app.retryLoad') }}</ElButton>
      </template>
    </ElResult>
  </main>
  <StartupLoading v-else-if="!routeReady" />
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
</style>
