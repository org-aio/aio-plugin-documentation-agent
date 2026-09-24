<template>
  <Dialog
    v-model="dialogVisible"
    :close-on-click-modal="false"
    :title="dialogTitle"
    max-height="60vh"
    scroll
    width="min(620px, calc(100vw - 32px))"
  >
    <el-form ref="formRef" v-loading="formLoading" :model="formData" :rules="formRules" label-position="top">
      <el-row class="!mx-0" :gutter="16">
        <el-col :span="12">
          <el-form-item label="发件邮箱" prop="mail">
            <el-input v-model="formData.mail" placeholder="请输入发件邮箱" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="用户名" prop="username">
            <el-input v-model="formData.username" placeholder="请输入 SMTP 用户名" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="密码" prop="password">
            <el-input v-model="formData.password" type="password" show-password placeholder="请输入 SMTP 密码或授权码" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="SMTP 服务器" prop="host">
            <el-input v-model="formData.host" placeholder="例如 smtp.example.com" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="SMTP 端口" prop="port">
            <el-input-number v-model="formData.port" class="!w-100%" :min="1" :max="65535" controls-position="right" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="启用 SSL" prop="sslEnable">
            <el-switch v-model="formData.sslEnable" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="启用 STARTTLS" prop="starttlsEnable">
            <el-switch v-model="formData.starttlsEnable" />
          </el-form-item>
        </el-col>
      </el-row>
    </el-form>

    <template #footer>
      <el-button :disabled="formLoading" type="primary" @click="submitForm">确定</el-button>
      <el-button @click="dialogVisible = false">取消</el-button>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import * as BoxunMailAccountApi from './BoxunMailAccountApi'

type BoxunMailAccountFormData = Parameters<typeof BoxunMailAccountApi.createBoxunMailAccount>[0]
type EntityId = string | number

defineOptions({ name: 'BoxunMailAccountForm' })

const emit = defineEmits<{ success: [] }>()
const message = useMessage()
const { t } = useI18n()
const dialogVisible = ref(false)
const dialogTitle = ref('')
const formLoading = ref(false)
const formType = ref<'create' | 'update'>('create')
const editingId = ref<EntityId>()
const formRef = ref()
const initialFormData = () => ({
  mail: '',
  username: '',
  password: '',
  host: '',
  port: 465,
  sslEnable: true,
  starttlsEnable: false
})
const formData = ref<BoxunMailAccountFormData>(initialFormData())

const formRules = {
  mail: [{ required: true, message: '请输入发件邮箱', trigger: 'blur' }],
  username: [{ required: true, message: '请输入 SMTP 用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入 SMTP 密码', trigger: 'blur' }],
  host: [{ required: true, message: '请输入 SMTP 服务器', trigger: 'blur' }],
  port: [{ required: true, message: '请输入 SMTP 端口', trigger: 'blur' }]
}

const open = async (type: 'create' | 'update', id?: EntityId) => {
  dialogVisible.value = true
  dialogTitle.value = type === 'create' ? '新增邮件账号' : '编辑邮件账号'
  formType.value = type
  editingId.value = id
  formData.value = initialFormData()
  formRef.value?.clearValidate()
  if (type === 'update' && id !== undefined) {
    formLoading.value = true
    try {
      formData.value = { ...initialFormData(), ...(await BoxunMailAccountApi.getBoxunMailAccount(id)) }
    } finally {
      formLoading.value = false
    }
  }
}
defineExpose({ open })

const submitForm = async () => {
  if (formLoading.value) return
  const valid = await formRef.value?.validate().catch(() => false)
  if (!valid) return
  const input = { ...formData.value }
  if (formType.value === 'update') {
    input.id = editingId.value
  }
  formLoading.value = true
  try {
    if (formType.value === 'create') {
      await BoxunMailAccountApi.createBoxunMailAccount(input)
      message.success(t('common.createSuccess'))
    } else {
      await BoxunMailAccountApi.updateBoxunMailAccount(input)
      message.success(t('common.updateSuccess'))
    }
    dialogVisible.value = false
    emit('success')
  } finally {
    formLoading.value = false
  }
}
</script>
