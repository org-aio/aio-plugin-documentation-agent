<script setup lang="ts">
import { onMounted, ref } from 'vue'
import PersonalCenter from '@/page-packages/user/src/views/Profile/PersonalCenter.vue'
import { t } from '@/locales'
import {
  accountState,
  loadProfile,
  updatePassword,
  updateProfile
} from '@/features/account/account'
import { LogoFileError, readLogoFile } from '@/layout/logoFile'

const errorMessage = ref('')
const refresh = async () => {
  errorMessage.value = ''
  try {
    await loadProfile()
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : t('account.profileFailed')
  }
}
const readAvatar = async (file: File): Promise<{ preview: string; file: File }> => {
  try {
    return { preview: await readLogoFile(file), file }
  } catch (error) {
    if (error instanceof LogoFileError) {
      throw new Error(t(error.messageKey))
    }
    throw error
  }
}
onMounted(refresh)
</script>

<template>
  <div class="account-page">
    <header class="account-page-heading">
      <h1>{{ t('account.personalCenter') }}</h1>
      <p>{{ t('account.profileSubtitle') }}</p>
    </header>
    <el-alert v-if="errorMessage" :title="errorMessage" type="error" :closable="false" show-icon>
      <el-button text @click="refresh">{{ t('account.retry') }}</el-button>
    </el-alert>
    <PersonalCenter
      v-if="accountState.profile"
      :profile="accountState.profile"
      :demo="false"
      :t="t"
      :read-avatar="readAvatar"
      :save-profile="updateProfile"
      :change-password="updatePassword"
    />
    <el-skeleton
      v-else-if="accountState.loading"
      :rows="8"
      animated
      :aria-label="t('account.loading')"
    />
  </div>
</template>

<style scoped>
.account-page-heading {
  margin-bottom: 20px;
}
.account-page-heading h1 {
  margin: 0 0 8px;
  font-size: 22px;
  font-weight: 600;
}
.account-page-heading p {
  margin: 0;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}
.account-page > .el-alert {
  margin-bottom: 16px;
}
</style>
