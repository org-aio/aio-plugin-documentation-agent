<script setup lang="ts">
import { onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { appConfig } from '@/config'
import request from '@/config/axios'
import { t } from '@/locales'
import { safeRedirect } from '@/utils/auth'
import { safeLocalStorage } from '@/utils/safeStorage'
import { login } from './account'
import { parseCaptchaConfig, type CaptchaConfig } from './captcha.mjs'
import CaptchaChallenge from './CaptchaChallenge.vue'

const props = withDefaults(defineProps<{ showTenant?: boolean; rememberAccount?: boolean }>(), {
  showTenant: false,
  rememberAccount: true
})
const route = useRoute()
const router = useRouter()
const storageKey = `admin-shell:${appConfig.appId}:remembered-account:v1`
const storageMessage = ref('')
const readUsername = () => {
  try {
    return props.rememberAccount ? safeLocalStorage.getItem(storageKey) || '' : ''
  } catch (error) {
    console.warn('无法读取记住的账号。', error)
    storageMessage.value = t('account.rememberAccountFailed')
    return ''
  }
}
const credentials = reactive({ username: readUsername(), password: '', tenantId: '' })
const remember = ref(Boolean(credentials.username))
const busy = ref(false)
const errorMessage = ref('')
const captchaConfig = ref<CaptchaConfig | null>(null)
const configLoading = ref(true)
const configError = ref('')
const captchaVisible = ref(false)
let configController: AbortController | undefined
let mounted = true

const saveUsername = () => {
  try {
    if (props.rememberAccount && remember.value) {
      safeLocalStorage.setItem(storageKey, credentials.username.trim())
    } else {
      safeLocalStorage.removeItem(storageKey)
    }
    storageMessage.value = ''
  } catch (error) {
    console.warn('无法保存记住的账号。', error)
    storageMessage.value = t('account.rememberAccountFailed')
  }
}
watch(
  [() => props.rememberAccount, remember],
  ([enabled, checked]) => {
    if (!enabled || !checked) {
      saveUsername()
    }
  },
  { immediate: true }
)

const loadCaptchaConfig = async () => {
  configController?.abort()
  const controller = new AbortController()
  configController = controller
  configLoading.value = true
  configError.value = ''
  captchaConfig.value = null
  try {
    const result = await request.get<unknown>({
      url: '/system/captcha/config',
      signal: controller.signal
    })
    if (!controller.signal.aborted) {
      captchaConfig.value = parseCaptchaConfig(result)
    }
  } catch (error) {
    if (!controller.signal.aborted) {
      console.warn('无法加载服务端验证码配置。', error)
      configError.value = t('account.captchaConfigFailed')
    }
  } finally {
    if (configController === controller) {
      configLoading.value = false
    }
  }
}

const performLogin = async (captchaVerification?: string) => {
  if (busy.value || !captchaConfig.value) {
    return
  }
  if (captchaConfig.value.enabled && !captchaVerification) {
    return
  }
  captchaVisible.value = false
  busy.value = true
  errorMessage.value = ''
  try {
    await login({
      username: credentials.username.trim(),
      password: credentials.password,
      tenantId: props.showTenant ? credentials.tenantId.trim() : undefined,
      captchaVerification
    })
    saveUsername()
    if (mounted) {
      await router.replace(safeRedirect(route.query.redirect))
    }
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : t('account.loginFailed')
  } finally {
    // 验证码取消和重试保留输入；真正尝试账号认证后才清空密码。
    credentials.password = ''
    busy.value = false
  }
}

const submit = () => {
  if (busy.value || captchaVisible.value || !captchaConfig.value) {
    return
  }
  errorMessage.value = ''
  if (!credentials.username.trim() || !credentials.password) {
    errorMessage.value = t(
      !credentials.username.trim() ? 'account.usernameRequired' : 'account.passwordRequired'
    )
    return
  }
  if (captchaConfig.value.enabled) {
    captchaVisible.value = true
  } else {
    void performLogin()
  }
}

onMounted(loadCaptchaConfig)
onBeforeUnmount(() => {
  mounted = false
  configController?.abort()
  credentials.password = ''
})
</script>

<template>
  <div class="account-login-form">
    <el-alert
      v-if="errorMessage"
      :title="errorMessage"
      type="error"
      :closable="false"
      show-icon
      role="alert"
    />
    <el-alert v-if="configError" type="error" :closable="false" show-icon role="alert">
      <p>{{ configError }}</p>
      <el-button :loading="configLoading" @click="loadCaptchaConfig">
        {{ t('account.retry') }}
      </el-button>
    </el-alert>
    <el-alert
      v-if="storageMessage"
      :title="storageMessage"
      type="warning"
      :closable="false"
      show-icon
    />
    <el-form label-position="top" :disabled="busy || captchaVisible" @submit.prevent="submit">
      <el-form-item :label="t('account.username')" required>
        <el-input
          v-model="credentials.username"
          :aria-label="t('account.username')"
          autocomplete="username"
          autofocus
        />
      </el-form-item>
      <el-form-item :label="t('account.password')" required>
        <el-input
          v-model="credentials.password"
          :aria-label="t('account.password')"
          type="password"
          show-password
          autocomplete="current-password"
        />
      </el-form-item>
      <el-form-item v-if="showTenant" :label="t('account.tenantId')">
        <el-input
          v-model="credentials.tenantId"
          :aria-label="t('account.tenantId')"
          autocomplete="off"
        />
      </el-form-item>
      <el-checkbox v-if="rememberAccount" v-model="remember" class="remember-account">
        {{ t('account.rememberAccount') }}
      </el-checkbox>
      <el-button
        class="login-submit"
        native-type="submit"
        type="primary"
        size="large"
        :disabled="!captchaConfig || captchaVisible"
        :loading="busy || configLoading"
      >
        {{ configLoading ? t('account.captchaLoadingConfig') : t('account.login') }}
      </el-button>
    </el-form>
    <CaptchaChallenge
      v-if="captchaVisible && captchaConfig?.enabled"
      :type="captchaConfig.type"
      @verified="performLogin"
      @cancel="captchaVisible = false"
    />
  </div>
</template>

<style scoped>
.account-login-form .el-alert {
  margin-bottom: 18px;
}
.account-login-form .el-alert p {
  margin: 0 0 8px;
}
.remember-account {
  margin-bottom: 14px;
}
.login-submit {
  width: 100%;
  margin-top: 8px;
}
</style>
