<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AdminLayout from '@/layout/AdminLayout.vue'
import { ensureHostSession, hasSession, sessionRevision } from '@/utils/auth'

const route = useRoute()
const router = useRouter()
const embedded = computed(() =>
  Boolean((globalThis.window as Window & { aioPlugin?: unknown } | undefined)?.aioPlugin)
)
const ready = ref(hasSession() || route.meta.public === true)
void ensureHostSession().finally(() => {
  ready.value = true
})
onMounted(() => {
  ready.value = true
})
watch(sessionRevision, () => {
  if (!hasSession() && !route.meta.public) {
    void router.replace({ path: '/login', query: { redirect: route.fullPath } })
  }
})
</script>

<template>
  <RouterView v-if="ready && route.meta.public" />
  <RouterView v-else-if="ready && hasSession() && embedded" />
  <AdminLayout v-else-if="hasSession()" :key="sessionRevision" />
</template>
