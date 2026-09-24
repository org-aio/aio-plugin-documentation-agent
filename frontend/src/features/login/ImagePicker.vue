<script setup lang="ts">
import { onBeforeUnmount, ref } from 'vue'
import { Picture, Upload } from '@element-plus/icons-vue'
import { resolveAssetUrl } from '@/config'
import { LogoFileError, logoFileAccept, readLogoFile } from '@/layout/logoFile'
import { t } from '@/locales'

const value = defineModel<string>({ default: '' })
defineProps<{ label: string }>()
const emit = defineEmits<{ loading: [value: boolean] }>()
const input = ref<HTMLInputElement>()
const loading = ref(false)
const errorMessage = ref('')
let generation = 0

onBeforeUnmount(() => {
  generation += 1
  emit('loading', false)
})

const selectImage = async (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) {
    return
  }
  const request = ++generation
  loading.value = true
  emit('loading', true)
  errorMessage.value = ''
  try {
    const image = await readLogoFile(file)
    if (generation === request) {
      value.value = image
    }
  } catch (error) {
    if (generation === request) {
      const key = error instanceof LogoFileError ? error.messageKey : 'login.imageReadFailed'
      const messageKey = key === 'settings.logoDecodeFailed' ? 'login.imageDecodeFailed' : key
      errorMessage.value = t(
        messageKey === 'settings.logoReadFailed' ? 'login.imageReadFailed' : messageKey
      )
    }
  } finally {
    target.value = ''
    if (generation === request) {
      loading.value = false
      emit('loading', false)
    }
  }
}

const removeImage = () => {
  generation += 1
  value.value = ''
  errorMessage.value = ''
  loading.value = false
  emit('loading', false)
}
</script>

<template>
  <div class="login-image-picker" :aria-busy="loading">
    <div class="login-image-preview">
      <img v-if="value" :src="resolveAssetUrl(value)" :alt="label" />
      <div v-else class="login-image-empty">
        <ElIcon><Picture /></ElIcon>
        <span>{{ t('login.imageEmpty') }}</span>
      </div>
    </div>
    <div class="login-image-actions">
      <input
        ref="input"
        type="file"
        :accept="logoFileAccept"
        :aria-label="label"
        hidden
        :disabled="loading"
        @change="selectImage"
      />
      <ElButton :icon="Upload" :loading="loading" @click="input?.click()">{{
        t('login.chooseImage')
      }}</ElButton>
      <ElButton text :disabled="!value && !loading" @click="removeImage">{{
        t('login.removeImage')
      }}</ElButton>
    </div>
    <p class="login-field-hint">{{ t('login.imageHint') }}</p>
    <p v-if="errorMessage" class="settings-error" role="alert">{{ errorMessage }}</p>
  </div>
</template>

<style scoped>
.login-image-picker {
  width: 100%;
}
.login-image-preview {
  display: grid;
  place-items: center;
  height: 132px;
  overflow: hidden;
  border: 1px dashed var(--el-border-color);
  border-radius: 10px;
  background: var(--el-fill-color-light);
}
.login-image-preview img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
.login-image-empty {
  display: grid;
  justify-items: center;
  gap: 8px;
  color: var(--el-text-color-placeholder);
  font-size: 12px;
}
.login-image-empty .el-icon {
  font-size: 26px;
}
.login-image-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-top: 10px;
}
.login-image-actions .el-button + .el-button {
  margin-left: 0;
}
.login-field-hint {
  margin-top: 8px;
  color: var(--el-text-color-secondary);
  font-size: 12px;
  line-height: 1.6;
}
</style>
