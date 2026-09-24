<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessageBox } from 'element-plus'
import { ArrowDown, SwitchButton, User, UserFilled } from '@element-plus/icons-vue'
import { accountState, logout } from '@/features/account/account'
import { t } from '@/locales'

const router = useRouter()
const busy = ref(false)
const displayName = computed(
  () =>
    accountState.profile?.nickname || accountState.profile?.username || t('account.personalCenter')
)
const handleCommand = async (command: string) => {
  if (command === 'profile') {
    await router.push('/user/profile')
    return
  }
  if (busy.value) return
  try {
    await ElMessageBox.confirm(t('account.logoutConfirm'), t('account.logout'), {
      confirmButtonText: t('account.logout'),
      cancelButtonText: t('account.cancelLogout'),
      type: 'warning'
    })
  } catch {
    return
  }
  busy.value = true
  try {
    await logout()
  } catch {
    // 请求层已显示失败原因，保留当前会话便于重试。
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <ElDropdown trigger="click" @command="handleCommand">
    <button class="header-user" :disabled="busy" :aria-label="t('account.userMenu')">
      <ElAvatar :size="26" :src="accountState.profile?.avatar || undefined" :icon="UserFilled" />
      <span class="header-user-name">{{ displayName }}</span>
      <ElIcon class="header-user-arrow"><ArrowDown /></ElIcon>
    </button>
    <template #dropdown>
      <ElDropdownMenu>
        <ElDropdownItem command="profile" :icon="User">{{
          t('account.personalCenter')
        }}</ElDropdownItem>
        <ElDropdownItem command="logout" :icon="SwitchButton" divided :disabled="busy">{{
          t('account.logout')
        }}</ElDropdownItem>
      </ElDropdownMenu>
    </template>
  </ElDropdown>
</template>
