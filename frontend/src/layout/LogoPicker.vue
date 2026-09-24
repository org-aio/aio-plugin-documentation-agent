<script setup lang="ts">
import { onBeforeUnmount, ref } from 'vue'
import { Upload } from '@element-plus/icons-vue'
import { appConfig, settings } from '@/config'
import { t } from '@/locales'
import AppIcon from './AppIcon.vue'
import { LogoFileError, logoFileAccept, readLogoFile } from './logoFile'

const emit = defineEmits<{ loading: [value: boolean] }>()
const fileInput = ref<HTMLInputElement>()
const loading = ref(false)
const errorMessage = ref('')
let active = true

onBeforeUnmount(() => {
  active = false
})

const selectLogo = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) {
    return
  }
  loading.value = true
  emit('loading', true)
  errorMessage.value = ''
  try {
    const logo = await readLogoFile(file)
    if (active) {
      settings.logo = logo
    }
  } catch (error) {
    console.warn('无法读取应用图标，保留当前图标。', error)
    const messageKey = error instanceof LogoFileError ? error.messageKey : 'settings.logoReadFailed'
    errorMessage.value = t(messageKey)
  } finally {
    input.value = ''
    loading.value = false
    emit('loading', false)
  }
}

const resetLogo = () => {
  settings.logo = appConfig.logo
  errorMessage.value = ''
}
</script>

<template>
  <div class="logo-picker" :aria-busy="loading">
    <div class="logo-picker-controls">
      <div class="logo-preview">
        <AppIcon :alt="t('settings.logoPreview')" />
      </div>
      <div class="logo-picker-actions">
        <input
          ref="fileInput"
          type="file"
          :accept="logoFileAccept"
          :aria-label="t('settings.chooseLogo')"
          :disabled="loading"
          hidden
          @change="selectLogo"
        />
        <ElButton :icon="Upload" :loading="loading" @click="fileInput?.click()">
          {{ t(loading ? 'settings.logoLoading' : 'settings.chooseLogo') }}
        </ElButton>
        <ElButton text :disabled="loading" @click="resetLogo">
          {{ t('settings.resetLogo') }}
        </ElButton>
      </div>
    </div>
    <p class="logo-picker-hint">{{ t('settings.logoHint') }}</p>
    <p v-if="errorMessage" class="settings-error" role="alert">{{ errorMessage }}</p>
  </div>
</template>

<style scoped>
.logo-picker {
  width: 100%;
}
.logo-picker-controls {
  display: flex;
  align-items: center;
  gap: 16px;
}
.logo-preview {
  display: grid;
  place-items: center;
  width: 76px;
  height: 76px;
  flex-shrink: 0;
  border: 1px solid var(--panel-border-color);
  border-radius: 12px;
  background: var(--el-fill-color-light);
}
.logo-preview :deep(img),
.logo-preview :deep(.el-icon) {
  width: 48px;
  height: 48px;
  object-fit: contain;
  border-radius: 8px;
}
.logo-picker-actions {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
}
.logo-picker-actions .el-button + .el-button {
  margin-left: 0;
}
.logo-picker-hint {
  margin-top: 10px;
  color: var(--el-text-color-secondary);
  font-size: 12px;
  line-height: 1.6;
}
</style>
