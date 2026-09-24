<script setup lang="ts">
import { ref } from 'vue'
import { Setting } from '@element-plus/icons-vue'
import { settings } from '@/config'
import LoginForm from '@/features/account/LoginForm.vue'
import LoginPresentation from '@/features/login/LoginPresentation.vue'
import SettingsDrawer from '@/layout/SettingsDrawer.vue'
import { t } from '@/locales'

const settingsVisible = ref(false)
</script>

<template>
  <LoginPresentation :configuration="settings.login">
    <LoginForm
      :show-tenant="settings.login.showTenant"
      :remember-account="settings.login.rememberAccount"
    />
    <template #actions>
      <ElButton
        :icon="Setting"
        circle
        :aria-label="t('login.settings')"
        :title="t('login.settings')"
        @click="settingsVisible = true"
      />
    </template>
  </LoginPresentation>
  <SettingsDrawer v-model="settingsVisible" initial-section="login" login-page />
</template>
