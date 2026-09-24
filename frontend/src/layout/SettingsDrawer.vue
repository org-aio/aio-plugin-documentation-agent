<script setup lang="ts">
import { ref, watch } from 'vue'
import { appConfig, resetSettings, settings, settingsSaveState } from '@/config'
import { t } from '@/locales'
import LogoPicker from './LogoPicker.vue'
import LoginSettings from '@/features/login/LoginSettings.vue'
import LoginPresentation from '@/features/login/LoginPresentation.vue'

const props = withDefaults(
  defineProps<{ initialSection?: 'appearance' | 'login'; loginPage?: boolean }>(),
  {
    initialSection: 'appearance',
    loginPage: false
  }
)
const visible = defineModel<boolean>({ default: false })
const section = ref(props.initialSection)
const previewVisible = ref(false)
const loginImageLoading = ref(false)
watch(visible, (open) => {
  if (open) section.value = props.initialSection
})
const logoLoading = ref(false)
watch(section, () => {
  logoLoading.value = false
  loginImageLoading.value = false
})
const colors = ['#409eff', '#3b82f6', '#6366f1', '#8b5cf6', '#14b8a6', '#f59e0b']

const updateThemeColor = (color: string | null) => {
  settings.themeColor = color || appConfig.themeColor
}
</script>

<template>
  <ElDrawer v-model="visible" :title="t('settings.title')" size="440px" class="settings-drawer">
    <ElTabs v-model="section" class="settings-tabs">
      <ElTabPane :label="t('settings.appearance')" name="appearance">
        <template v-if="section === 'appearance'">
          <p class="settings-description">{{ t('settings.description') }}</p>
          <ElForm label-position="top" class="settings-form">
            <h3>{{ t('settings.brand') }}</h3>
            <ElFormItem :label="t('settings.appTitle')">
              <ElInput v-model="settings.title" :placeholder="appConfig.title" maxlength="40" />
            </ElFormItem>
            <ElFormItem :label="t('settings.logo')">
              <LogoPicker @loading="logoLoading = $event" />
            </ElFormItem>
            <ElDivider />
            <h3>{{ t('settings.appearance') }}</h3>
            <div class="settings-row">
              <span>{{ t('settings.themeColor') }}</span>
              <ElColorPicker
                :model-value="settings.themeColor"
                :predefine="colors"
                :aria-label="t('settings.themeColor')"
                @update:model-value="updateThemeColor"
              />
            </div>
            <div class="settings-row">
              <span>{{ t('settings.dark') }}</span>
              <ElSwitch v-model="settings.dark" :aria-label="t('settings.dark')" />
            </div>
            <ElDivider />
            <h3>{{ t('settings.layout') }}</h3>
            <div
              v-for="entry in [
                'showLogo',
                'showBreadcrumb',
                'showTagsIcon',
                'uniqueOpened'
              ] as const"
              :key="entry"
              class="settings-row"
            >
              <span>{{ t(`settings.${entry}`) }}</span>
              <ElSwitch v-model="settings[entry]" :aria-label="t(`settings.${entry}`)" />
            </div>
            <div class="settings-row">
              <span>{{ t('settings.collapsed') }}</span>
              <ElSwitch v-model="settings.collapsed" :aria-label="t('settings.collapsed')" />
            </div>
            <div class="settings-row">
              <span>{{ t('settings.tagsView') }}</span>
              <ElSwitch v-model="settings.showTagsView" :aria-label="t('settings.tagsView')" />
            </div>
            <div class="settings-row">
              <span>{{ t('settings.footer') }}</span>
              <ElSwitch v-model="settings.showFooter" :aria-label="t('settings.footer')" />
            </div>
            <ElFormItem v-if="settings.showFooter" :label="t('settings.footerText')">
              <ElInput v-model="settings.footer" maxlength="100" />
            </ElFormItem>
          </ElForm>
        </template>
      </ElTabPane>
      <ElTabPane :label="t('login.tab')" name="login" lazy>
        <LoginSettings
          v-if="section === 'login'"
          :preview-available="!loginPage"
          @preview="previewVisible = true"
          @loading="loginImageLoading = $event"
        />
      </ElTabPane>
    </ElTabs>
    <p
      class="settings-hint"
      :class="{ 'settings-error': settingsSaveState === 'error' }"
      role="status"
      aria-live="polite"
    >
      {{ t(settingsSaveState === 'error' ? 'settings.saveFailed' : 'settings.saved') }}
    </p>
    <template #footer>
      <ElButton :disabled="logoLoading || loginImageLoading" @click="resetSettings">{{
        t('settings.reset')
      }}</ElButton>
      <ElButton type="primary" @click="visible = false">{{ t('settings.done') }}</ElButton>
    </template>
  </ElDrawer>
  <ElDialog
    v-model="previewVisible"
    :title="t('login.preview')"
    width="min(1180px, 96vw)"
    top="3vh"
    append-to-body
    class="login-preview-dialog"
    destroy-on-close
  >
    <p class="login-preview-hint">{{ t('login.previewHint') }}</p>
    <div class="login-preview-surface">
      <LoginPresentation embedded :configuration="settings.login">
        <ElForm label-position="top" disabled @submit.prevent>
          <ElFormItem :label="t('account.username')">
            <ElInput
              :placeholder="t('login.previewUsername')"
              :aria-label="t('account.username')"
              size="large"
            />
          </ElFormItem>
          <ElFormItem :label="t('account.password')">
            <ElInput
              :placeholder="t('login.previewPassword')"
              :aria-label="t('account.password')"
              type="password"
              size="large"
            />
          </ElFormItem>
          <ElFormItem v-if="settings.login.showTenant" :label="t('account.tenantId')">
            <ElInput
              :placeholder="t('login.previewTenant')"
              :aria-label="t('account.tenantId')"
              size="large"
            />
          </ElFormItem>
          <ElCheckbox v-if="settings.login.rememberAccount" class="login-preview-remember">{{
            t('login.previewRemember')
          }}</ElCheckbox>
          <ElButton type="primary" size="large" class="login-preview-submit">{{
            t('account.login')
          }}</ElButton>
        </ElForm>
      </LoginPresentation>
    </div>
  </ElDialog>
</template>

<style scoped>
.settings-tabs :deep(.el-tabs__header) {
  margin-bottom: 24px;
}
.login-preview-hint {
  margin-bottom: 14px;
  color: var(--el-text-color-secondary);
  font-size: 13px;
}
.login-preview-surface {
  overflow: hidden;
  border: 1px solid var(--panel-border-color);
  border-radius: 16px;
}
.login-preview-remember {
  margin-bottom: 18px;
}
.login-preview-submit {
  display: block;
  width: 100%;
}
</style>
