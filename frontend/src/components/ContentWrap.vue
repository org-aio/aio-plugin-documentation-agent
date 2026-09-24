<script setup lang="ts">
import type { CSSProperties } from 'vue'
import { ElCard, ElTooltip } from 'element-plus'
import { Icon } from './Icon'

withDefaults(defineProps<{ title?: string; message?: string; bodyStyle?: CSSProperties }>(), {
  title: '',
  message: '',
  bodyStyle: () => ({ padding: '16px' })
})
</script>

<template>
  <ElCard class="page-content-wrap" shadow="never" :body-style="bodyStyle">
    <template v-if="title || $slots.header" #header>
      <div class="page-content-header">
        <span v-if="title">{{ title }}</span>
        <ElTooltip v-if="message" :content="message">
          <Icon icon="ep:question-filled" />
        </ElTooltip>
        <slot name="header" />
      </div>
    </template>
    <slot />
  </ElCard>
</template>

<style scoped>
.page-content-wrap {
  margin-bottom: 16px;
  overflow: visible;
  color: var(--el-text-color-primary);
  background: var(--el-bg-color);
  border-color: var(--el-border-color-light);
}

.page-content-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}
</style>
