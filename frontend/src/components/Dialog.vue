<script setup lang="ts">
import { computed, ref } from 'vue'
import { ElButton, ElDialog, ElScrollbar } from 'element-plus'
import { Icon } from './Icon'
import { useI18n } from '@/hooks/web/useI18n'

defineOptions({ inheritAttrs: false })
const props = withDefaults(
  defineProps<{
    modelValue?: boolean
    title?: string
    fullscreen?: boolean
    width?: string | number
    closeOnClickModal?: boolean
    scroll?: boolean
    maxHeight?: string | number
  }>(),
  {
    modelValue: false,
    title: '',
    fullscreen: true,
    width: 680,
    closeOnClickModal: false,
    scroll: false,
    maxHeight: 500
  }
)
const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>()
const { t } = useI18n()
const isFullscreen = ref(false)
const widthStyle = computed(() => {
  const width = String(props.width)
  return /^\d+$/.test(width) ? `${width}px` : width
})
const contentHeight = computed(() => (isFullscreen.value ? 'calc(100vh - 160px)' : props.maxHeight))
</script>

<template>
  <ElDialog
    v-bind="$attrs"
    :model-value="modelValue"
    :title="title"
    :width="widthStyle"
    :fullscreen="isFullscreen"
    :close-on-click-modal="closeOnClickModal"
    class="page-host-dialog"
    append-to-body
    destroy-on-close
    draggable
    @update:model-value="emit('update:modelValue', $event)"
  >
    <template #header>
      <div class="page-dialog-header">
        <slot name="title">{{ title }}</slot>
        <ElButton
          v-if="fullscreen"
          text
          :aria-label="t('pageHost.fullscreen')"
          @click="isFullscreen = !isFullscreen"
        >
          <Icon icon="ep:full-screen" />
        </ElButton>
      </div>
    </template>
    <ElScrollbar v-if="scroll" :max-height="contentHeight">
      <slot />
    </ElScrollbar>
    <slot v-else />
    <template v-if="$slots.footer" #footer>
      <slot name="footer" />
    </template>
  </ElDialog>
</template>

<style>
.page-host-dialog {
  max-width: calc(100vw - 24px);
  color: var(--el-text-color-primary);
  background: var(--el-bg-color-overlay);
}

.page-host-dialog.is-fullscreen {
  max-width: 100vw;
}

.page-dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 24px;
  padding-right: 24px;
  font-size: 18px;
}
</style>
