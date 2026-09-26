<script setup lang="ts">
import { t } from '@/locales'
</script>

<template>
  <main class="app-page-loading" aria-live="polite" aria-busy="true">
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
</template>

<style scoped>
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
