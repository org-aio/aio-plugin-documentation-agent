<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AdminLayout from '@/layout/AdminLayout.vue'
import { ensureHostSession, hasSession, sessionRevision } from '@/utils/auth'
import { t } from '@/locales'

const route = useRoute()
const router = useRouter()
const embedded = computed(() =>
  Boolean((globalThis.window as (Window & { aioPlugin?: unknown }) | undefined)?.aioPlugin)
)
const ready = ref(hasSession() || route.meta.public === true || embedded.value)
const sessionError = ref('')
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
watch(sessionRevision, () => {
  if (!hasSession() && !route.meta.public) {
    void router.replace({ path: '/login', query: { redirect: route.fullPath } })
  }
})
</script>

<template>
  <RouterView v-if="ready && route.meta.public" />
  <RouterView v-else-if="ready && embedded" />
  <AdminLayout v-else-if="hasSession()" :key="sessionRevision" />
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
