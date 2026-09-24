<script setup lang="ts">
import { ElTooltip } from 'element-plus'
import { Icon } from './Icon'

defineProps<{ title?: string; message: string }>()
</script>

<template>
  <span class="page-label-tooltip">
    {{ title }}
    <ElTooltip :content="message" placement="top" effect="dark">
      <span tabindex="0" :aria-label="message"><Icon icon="ep:question-filled" /></span>
    </ElTooltip>
  </span>
</template>

<style scoped>
.page-label-tooltip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
}
</style>
