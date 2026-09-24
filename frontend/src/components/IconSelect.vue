<script setup lang="ts">
import { ElOption, ElSelect } from 'element-plus'
import { Icon } from './Icon'
import { iconOptions } from './Icon/icons'
import { useI18n } from '@/hooks/web/useI18n'

withDefaults(defineProps<{ modelValue?: string; clearable?: boolean }>(), {
  modelValue: '',
  clearable: false
})
const emit = defineEmits<{ 'update:modelValue': [value: string] }>()
const { t } = useI18n()
</script>

<template>
  <ElSelect
    :model-value="modelValue"
    :clearable="clearable"
    :placeholder="t('pageHost.selectIcon')"
    filterable
    class="page-icon-select"
    @update:model-value="emit('update:modelValue', $event || '')"
  >
    <template #prefix>
      <Icon v-if="modelValue" :icon="modelValue" />
    </template>
    <ElOption v-for="icon in iconOptions" :key="icon" :value="icon" :label="icon">
      <span class="page-icon-option"><Icon :icon="icon" /> {{ icon }}</span>
    </ElOption>
  </ElSelect>
</template>

<style scoped>
.page-icon-select {
  width: 100%;
}

.page-icon-option {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}
</style>
