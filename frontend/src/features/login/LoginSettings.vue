<script setup lang="ts">
import { computed, onBeforeUnmount, reactive, ref, watch } from 'vue'
import { Download, View } from '@element-plus/icons-vue'
import { appConfig, resolveAssetUrl, settings } from '@/config'
import { t } from '@/locales'
import { normalizeLoginConfiguration } from './config.mjs'
import { exportLoginConfiguration, LoginExportError } from './export.mjs'
import ImagePicker from './ImagePicker.vue'

withDefaults(defineProps<{ previewAvailable?: boolean }>(), { previewAvailable: true })
const emit = defineEmits<{ preview: []; loading: [value: boolean] }>()
const imageLoading = reactive({ background: false, hero: false, brand: false, qrcode: false })
const exporting = ref(false)
const exportError = ref('')
let exportController: AbortController | undefined
let mounted = true
const loading = computed(
  () =>
    imageLoading.background ||
    imageLoading.hero ||
    imageLoading.brand ||
    imageLoading.qrcode ||
    exporting.value
)
const colors = ['#0b1730', '#163c54', '#253449', '#e7edf7', '#edf5f1', '#ffffff']
const displayOptions = [
  'showBrand',
  'showTenant',
  'rememberAccount',
  'showThemeToggle',
  'showFooter',
  'showQrcode'
] as const

watch(loading, (value) => emit('loading', value))
onBeforeUnmount(() => {
  mounted = false
  exportController?.abort()
  emit('loading', false)
})

const resetLogin = () => {
  settings.login = normalizeLoginConfiguration(appConfig.login)
}

const exportLogin = async () => {
  if (loading.value) return
  exporting.value = true
  exportError.value = ''
  const controller = new AbortController()
  exportController = controller
  const timeout = window.setTimeout(() => controller.abort(), 15000)
  try {
    // 本地图片随展示配置内嵌导出，不读取账号、密码或验证码状态。
    const configuration = await exportLoginConfiguration(settings.login, {
      pageUrl: window.location.href,
      resolveAssetUrl,
      signal: controller.signal
    })
    if (!mounted) return
    controller.signal.throwIfAborted()
    const content = `${JSON.stringify(configuration, null, 2)}\n`
    const url = URL.createObjectURL(new Blob([content], { type: 'application/json' }))
    const link = document.createElement('a')
    link.href = url
    link.download = `${appConfig.appId}-login.json`
    document.body.append(link)
    link.click()
    link.remove()
    window.setTimeout(() => URL.revokeObjectURL(url), 1000)
  } catch (error) {
    controller.abort()
    if (mounted) {
      const field = error instanceof LoginExportError ? t(`login.${error.field}`) : ''
      exportError.value = `${field ? `${field}：` : ''}${t('login.exportFailed')}`
    }
  } finally {
    window.clearTimeout(timeout)
    exporting.value = false
    exportController = undefined
  }
}
</script>

<template>
  <div class="login-settings">
    <p class="settings-description">{{ t('login.description') }}</p>
    <ElButton
      v-if="previewAvailable"
      :icon="View"
      class="login-preview-button"
      @click="emit('preview')"
    >
      {{ t('login.preview') }}
    </ElButton>
    <p v-else class="login-field-hint login-live-preview">{{ t('login.livePreview') }}</p>
    <ElForm label-position="top" class="settings-form">
      <h3>{{ t('login.layout') }}</h3>
      <ElFormItem :label="t('login.layout')">
        <ElRadioGroup v-model="settings.login.layout" :aria-label="t('login.layout')">
          <ElRadioButton value="split">{{ t('login.split') }}</ElRadioButton>
          <ElRadioButton value="centered">{{ t('login.centered') }}</ElRadioButton>
        </ElRadioGroup>
      </ElFormItem>
      <ElFormItem v-if="settings.login.layout === 'split'" :label="t('login.panelPosition')">
        <ElRadioGroup v-model="settings.login.panelPosition" :aria-label="t('login.panelPosition')">
          <ElRadioButton value="left">{{ t('login.left') }}</ElRadioButton>
          <ElRadioButton value="right">{{ t('login.right') }}</ElRadioButton>
        </ElRadioGroup>
      </ElFormItem>
      <ElDivider />
      <h3>{{ t('login.background') }}</h3>
      <ElFormItem :label="t('login.backgroundType')">
        <ElRadioGroup
          v-model="settings.login.backgroundType"
          :aria-label="t('login.backgroundType')"
        >
          <ElRadioButton value="gradient">{{ t('login.gradient') }}</ElRadioButton>
          <ElRadioButton value="solid">{{ t('login.solid') }}</ElRadioButton>
          <ElRadioButton value="image">{{ t('login.image') }}</ElRadioButton>
        </ElRadioGroup>
      </ElFormItem>
      <div class="settings-row">
        <span>{{ t('login.backgroundColor') }}</span>
        <ElColorPicker
          :model-value="settings.login.backgroundColor"
          :predefine="colors"
          :aria-label="t('login.backgroundColor')"
          @update:model-value="
            settings.login.backgroundColor = $event || appConfig.login.backgroundColor
          "
        />
      </div>
      <template v-if="settings.login.backgroundType === 'image'">
        <ElFormItem :label="t('login.backgroundImage')" class="login-image-field">
          <ImagePicker
            v-model="settings.login.backgroundImage"
            :label="t('login.backgroundImage')"
            @loading="imageLoading.background = $event"
          />
        </ElFormItem>
        <ElFormItem :label="t('login.backgroundOverlay')">
          <ElSlider
            v-model="settings.login.backgroundOverlay"
            :min="0"
            :max="1"
            :step="0.05"
            :format-tooltip="(value: number) => `${Math.round(value * 100)}%`"
            :aria-label="t('login.backgroundOverlay')"
          />
        </ElFormItem>
      </template>
      <ElDivider />
      <h3>{{ t('login.hero') }}</h3>
      <div class="settings-row">
        <span>{{ t('login.showHero') }}</span>
        <ElSwitch v-model="settings.login.showHero" :aria-label="t('login.showHero')" />
      </div>
      <template v-if="settings.login.showHero">
        <ElFormItem :label="t('login.heroTitle')">
          <ElInput v-model="settings.login.heroTitle" maxlength="120" />
        </ElFormItem>
        <ElFormItem :label="t('login.heroDescription')">
          <ElInput
            v-model="settings.login.heroDescription"
            type="textarea"
            :rows="3"
            maxlength="400"
          />
        </ElFormItem>
        <ElFormItem :label="t('login.heroImage')">
          <ImagePicker
            v-model="settings.login.heroImage"
            :label="t('login.heroImage')"
            @loading="imageLoading.hero = $event"
          />
          <p class="login-field-hint">{{ t('login.heroImageHint') }}</p>
        </ElFormItem>
      </template>
      <ElDivider />
      <h3>{{ t('login.showBrand') }}</h3>
      <template v-if="settings.login.showBrand">
        <ElFormItem :label="t('login.brandImage')">
          <ImagePicker
            v-model="settings.login.brandImage"
            :label="t('login.brandImage')"
            @loading="imageLoading.brand = $event"
          />
        </ElFormItem>
      </template>
      <ElDivider />
      <h3>{{ t('login.form') }}</h3>
      <ElFormItem :label="t('login.formTitle')">
        <ElInput v-model="settings.login.formTitle" maxlength="80" />
      </ElFormItem>
      <ElFormItem :label="t('login.formDescription')">
        <ElInput
          v-model="settings.login.formDescription"
          type="textarea"
          :rows="2"
          maxlength="200"
        />
      </ElFormItem>
      <ElDivider />
      <h3>{{ t('login.components') }}</h3>
      <div v-for="entry in displayOptions" :key="entry" class="settings-row">
        <span>{{ t(`login.${entry}`) }}</span>
        <ElSwitch v-model="settings.login[entry]" :aria-label="t(`login.${entry}`)" />
      </div>
      <ElFormItem v-if="settings.login.showFooter" :label="t('login.footer')">
        <ElInput v-model="settings.login.footer" type="textarea" :rows="2" maxlength="200" />
      </ElFormItem>
      <template v-if="settings.login.showQrcode">
        <ElFormItem :label="t('login.qrcodeImage')">
          <ImagePicker
            v-model="settings.login.qrcodeImage"
            :label="t('login.qrcodeImage')"
            @loading="imageLoading.qrcode = $event"
          />
        </ElFormItem>
        <ElFormItem :label="t('login.qrcodeLink')">
          <ElInput v-model="settings.login.qrcodeLink" maxlength="2000" />
        </ElFormItem>
        <ElFormItem :label="t('login.qrcodeLabel')">
          <ElInput v-model="settings.login.qrcodeLabel" maxlength="80" />
        </ElFormItem>
        <ElFormItem :label="t('login.qrcodeDescription')">
          <ElInput
            v-model="settings.login.qrcodeDescription"
            type="textarea"
            :rows="2"
            maxlength="200"
          />
        </ElFormItem>
      </template>
    </ElForm>
    <ElDivider />
    <div class="login-settings-actions">
      <ElButton :icon="Download" :loading="exporting" :disabled="loading" @click="exportLogin">{{
        t('login.export')
      }}</ElButton>
      <ElButton :disabled="loading" @click="resetLogin">{{ t('login.reset') }}</ElButton>
    </div>
    <p class="login-field-hint">{{ t('login.exportHint') }}</p>
    <p v-if="exportError" class="settings-error" role="alert">{{ exportError }}</p>
  </div>
</template>

<style scoped>
.login-preview-button {
  width: 100%;
  margin-bottom: 26px;
}
.login-field-hint {
  margin-top: 8px;
  color: var(--el-text-color-secondary);
  font-size: 12px;
  line-height: 1.7;
}
.login-live-preview {
  margin-top: -14px;
  margin-bottom: 25px;
}
.login-image-field {
  margin-top: 14px;
}
.login-settings-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}
.login-settings-actions .el-button + .el-button {
  margin-left: 0;
}
</style>
