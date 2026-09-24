<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import { ElMessage } from "element-plus";

interface Profile {
  username: string;
  nickname: string;
  email: string;
  mobile: string;
  sex: number;
  avatar: string;
  createTime?: string | number;
  dept?: { name: string } | null;
  roles?: { name: string }[];
  posts?: { name: string }[];
}
type ProfileChanges = Pick<
  Profile,
  "nickname" | "email" | "mobile" | "sex"
> & {
  avatar: string;
  avatarFile?: File | null;
};

// 宿主注入请求与图片读取能力，公共页面不绑定具体认证 Store 或上传服务。
const props = defineProps<{
  profile: Profile;
  demo: boolean;
  t: (key: string) => string;
  readAvatar: (file: File) => Promise<{ preview: string; file: File }>;
  saveProfile: (changes: ProfileChanges) => Promise<void>;
  changePassword: (oldPassword: string, newPassword: string) => Promise<void>;
}>();

const activeTab = ref("basic");
const saving = ref(false);
const readingAvatar = ref(false);
const avatarFile = ref<File | null>(null);
const errorMessage = ref("");
const fileInput = ref<HTMLInputElement>();
const draft = reactive<ProfileChanges>({
  nickname: "",
  email: "",
  mobile: "",
  sex: 0,
  avatar: "",
});
const password = reactive({ old: "", next: "", confirm: "" });
const initial = computed(() =>
  (props.profile.nickname || props.profile.username).slice(0, 1).toUpperCase(),
);
const resetDraft = () => {
  const { nickname, email, mobile, sex, avatar } = props.profile;
  avatarFile.value = null;
  Object.assign(draft, {
    nickname,
    email: email ?? "",
    mobile: mobile ?? "",
    sex: sex ?? 0,
    avatar: avatar ?? "",
  });
  errorMessage.value = "";
};
watch(() => props.profile, resetDraft, { immediate: true });
watch(activeTab, () => {
  errorMessage.value = "";
  Object.assign(password, { old: "", next: "", confirm: "" });
});

const selectAvatar = async (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) {
    return;
  }
  readingAvatar.value = true;
  errorMessage.value = "";
  try {
    const avatar = await props.readAvatar(file);
    draft.avatar = avatar.preview;
    avatarFile.value = avatar.file;
  } catch (error) {
    errorMessage.value =
      error instanceof Error ? error.message : props.t("account.saveFailed");
  } finally {
    input.value = "";
    readingAvatar.value = false;
  }
};

const clearAvatar = () => {
  draft.avatar = "";
  avatarFile.value = null;
};

const saveBasic = async () => {
  errorMessage.value = "";
  if (!draft.nickname.trim()) {
    errorMessage.value = props.t("account.nicknameRequired");
    return;
  }
  if (draft.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(draft.email)) {
    errorMessage.value = props.t("account.emailInvalid");
    return;
  }
  if (draft.mobile && !/^\d{11}$/.test(draft.mobile)) {
    errorMessage.value = props.t("account.mobileInvalid");
    return;
  }
  saving.value = true;
  try {
    await props.saveProfile({
      ...draft,
      nickname: draft.nickname.trim(),
      avatarFile: avatarFile.value,
    });
    ElMessage.success(props.t("account.saved"));
  } catch (error) {
    errorMessage.value =
      error instanceof Error ? error.message : props.t("account.saveFailed");
  } finally {
    saving.value = false;
  }
};

const savePassword = async () => {
  errorMessage.value = "";
  if (!password.old) {
    errorMessage.value = props.t("account.passwordRequired");
    return;
  }
  if (password.next.length < 6 || password.next.length > 20) {
    errorMessage.value = props.t("account.passwordLength");
    return;
  }
  if (password.next !== password.confirm) {
    errorMessage.value = props.t("account.passwordMismatch");
    return;
  }
  if (password.old === password.next) {
    errorMessage.value = props.t("account.passwordUnchanged");
    return;
  }
  saving.value = true;
  try {
    await props.changePassword(password.old, password.next);
    Object.assign(password, { old: "", next: "", confirm: "" });
    ElMessage.success(props.t("account.passwordSaved"));
  } catch (error) {
    errorMessage.value =
      error instanceof Error ? error.message : props.t("account.saveFailed");
  } finally {
    saving.value = false;
  }
};

const details = computed(() => [
  ["account.username", props.profile.username],
  ["account.mobile", props.profile.mobile],
  ["account.email", props.profile.email],
  ["account.dept", props.profile.dept?.name],
  ["account.roles", props.profile.roles?.map((item) => item.name).join("、")],
  ["account.posts", props.profile.posts?.map((item) => item.name).join("、")],
  [
    "account.createdAt",
    props.profile.createTime
      ? new Date(props.profile.createTime).toLocaleString("zh-CN")
      : "",
  ],
]);
</script>

<template>
  <section class="personal-center">
    <el-card class="profile-summary" shadow="never">
      <div class="profile-identity">
        <el-avatar :size="84" :src="profile.avatar || undefined">{{
          initial
        }}</el-avatar>
        <h2>{{ profile.nickname || profile.username }}</h2>
        <el-tag size="small" :type="demo ? 'warning' : 'primary'">{{
          t(demo ? "account.demoMode" : "account.apiMode")
        }}</el-tag>
      </div>
      <dl class="profile-details">
        <div v-for="[key, value] in details" :key="key">
          <dt>{{ t(key || "") }}</dt>
          <dd>{{ value || t("account.notSet") }}</dd>
        </div>
      </dl>
    </el-card>

    <el-card class="profile-editor" shadow="never">
      <el-tabs v-model="activeTab">
        <el-tab-pane :label="t('account.basicInfo')" name="basic" />
        <el-tab-pane :label="t('account.changePassword')" name="password" />
      </el-tabs>
      <el-alert
        v-if="errorMessage"
        class="profile-alert"
        :title="errorMessage"
        type="error"
        :closable="false"
        show-icon
        role="alert"
      />

      <el-form
        v-if="activeTab === 'basic'"
        label-position="top"
        class="profile-form"
        :disabled="saving"
        @submit.prevent="saveBasic"
      >
        <el-alert
          v-if="demo"
          class="profile-alert"
          :title="t('account.demoProfileNotice')"
          type="info"
          :closable="false"
          show-icon
        />
        <div class="avatar-editor">
          <el-avatar
            :size="64"
            :src="draft.avatar || undefined"
            :alt="t('account.avatarAlt')"
            >{{ initial }}</el-avatar
          >
          <div>
            <input
              ref="fileInput"
              class="avatar-file-input"
              type="file"
              accept=".svg,.png,.jpg,.jpeg,.webp,.ico"
              :aria-label="t('account.chooseAvatar')"
              @change="selectAvatar"
            />
            <el-button :loading="readingAvatar" @click="fileInput?.click()">{{
              t("account.chooseAvatar")
            }}</el-button>
            <el-button
              v-if="draft.avatar"
              text
              @click="clearAvatar"
              >{{ t("account.removeAvatar") }}</el-button
            >
            <p class="field-hint">{{ t("account.avatarHint") }}</p>
          </div>
        </div>
        <el-form-item :label="t('account.nickname')" required>
          <el-input
            v-model="draft.nickname"
            :aria-label="t('account.nickname')"
            maxlength="30"
            autocomplete="nickname"
          />
        </el-form-item>
        <el-form-item :label="t('account.email')">
          <el-input
            v-model="draft.email"
            :aria-label="t('account.email')"
            maxlength="50"
            type="email"
            autocomplete="email"
          />
        </el-form-item>
        <el-form-item :label="t('account.mobile')">
          <el-input
            v-model="draft.mobile"
            :aria-label="t('account.mobile')"
            maxlength="11"
            type="tel"
            autocomplete="tel"
          />
        </el-form-item>
        <el-form-item :label="t('account.sex')">
          <el-radio-group v-model="draft.sex" :aria-label="t('account.sex')">
            <el-radio :value="0">{{ t("account.sexUnknown") }}</el-radio>
            <el-radio :value="1">{{ t("account.sexMale") }}</el-radio>
            <el-radio :value="2">{{ t("account.sexFemale") }}</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-button
          native-type="submit"
          type="primary"
          :loading="saving"
          :disabled="readingAvatar"
          >{{ t("account.save") }}</el-button
        >
        <el-button @click="resetDraft">{{ t("account.cancel") }}</el-button>
      </el-form>

      <div v-else-if="demo" class="password-notice">
        <el-alert
          :title="t('account.demoPasswordNotice')"
          type="info"
          :closable="false"
          show-icon
        />
      </div>
      <el-form
        v-else
        label-position="top"
        class="profile-form"
        :disabled="saving"
        @submit.prevent="savePassword"
      >
        <el-form-item :label="t('account.oldPassword')" required>
          <el-input
            v-model="password.old"
            :aria-label="t('account.oldPassword')"
            type="password"
            show-password
            autocomplete="current-password"
          />
        </el-form-item>
        <el-form-item :label="t('account.newPassword')" required>
          <el-input
            v-model="password.next"
            :aria-label="t('account.newPassword')"
            type="password"
            show-password
            autocomplete="new-password"
            maxlength="20"
          />
        </el-form-item>
        <el-form-item :label="t('account.confirmPassword')" required>
          <el-input
            v-model="password.confirm"
            :aria-label="t('account.confirmPassword')"
            type="password"
            show-password
            autocomplete="new-password"
            maxlength="20"
          />
        </el-form-item>
        <el-button native-type="submit" type="primary" :loading="saving">{{
          t("account.save")
        }}</el-button>
      </el-form>
    </el-card>
  </section>
</template>

<style scoped>
.personal-center {
  display: grid;
  grid-template-columns: minmax(260px, 320px) minmax(0, 1fr);
  gap: 20px;
  align-items: start;
}
.profile-identity {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 8px 0 24px;
}
.profile-identity h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}
.profile-details {
  margin: 0;
  font-size: 13px;
}
.profile-details > div {
  display: flex;
  justify-content: space-between;
  gap: 18px;
  padding: 14px 0;
  border-top: 1px solid var(--el-border-color-lighter);
}
.profile-details dt {
  flex-shrink: 0;
  color: var(--el-text-color-secondary);
}
.profile-details dd {
  margin: 0;
  text-align: right;
  overflow-wrap: anywhere;
  color: var(--el-text-color-regular);
}
.profile-form {
  max-width: 580px;
  padding: 12px 0 8px;
}
.profile-alert {
  margin-bottom: 20px;
}
.avatar-editor {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}
.avatar-editor .el-avatar {
  flex-shrink: 0;
}
.avatar-file-input {
  display: none;
}
.field-hint {
  margin: 8px 0 0;
  font-size: 12px;
  line-height: 1.6;
  color: var(--el-text-color-secondary);
}
.password-notice {
  padding: 12px 0 24px;
}
@media (max-width: 800px) {
  .personal-center {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
