<!-- 模板管理为业务定制页面，保留手写维护，避免 Studio 前端生成覆盖。 -->
<template>
  <Dialog v-model="dialogVisible" :title="dialogTitle" width="760">
    <el-form ref="formRef" v-loading="formLoading" :model="formData" :rules="formRules" label-width="120px">
      <el-row :gutter="16">
        <el-col :span="12">
          <el-form-item label="项目名称" prop="projectId">
            <el-select v-model="formData.projectId" filterable placeholder="请选择项目名称" class="!w-100%">
              <el-option v-for="project in props.projects" :key="project.id" :label="project.name" :value="project.id" />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="模板类型" prop="templateType">
            <el-select v-model="formData.templateType" placeholder="请选择模板类型" class="!w-100%">
              <el-option v-for="(label, value) in TEMPLATE_TYPE_NAMES" :key="value" :label="label" :value="Number(value)" />
            </el-select>
          </el-form-item>
        </el-col>
        <el-col :span="24">
          <el-form-item label="模板" prop="templateUrl">
            <el-upload
              :show-file-list="false"
              :http-request="handleTemplateUpload"
              accept=".xls,.xlsx"
              :disabled="formLoading"
            >
              <el-button :loading="uploading" :disabled="formLoading">上传模板</el-button>
            </el-upload>
            <span v-if="formData.templateUrl" class="template-file">{{ formData.templateUrl }}</span>
          </el-form-item>
        </el-col>
        <el-col :span="24">
          <el-form-item label="模板描述" prop="templateDescription">
            <el-input v-model="formData.templateDescription" placeholder="请输入模板描述" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="是否启用" prop="enableOrNot">
            <el-switch v-model="formData.enableOrNot" :active-value="1" :inactive-value="0" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item label="字节码全限定类名" prop="bytecodeRef">
            <el-input v-model="formData.bytecodeRef" placeholder="请输入字节码全限定类名" />
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
import type { UploadRequestOptions } from 'element-plus'
import { updateFile } from '@/api/infra/file'
import * as BoxunTemplateManagementApi from './BoxunTemplateManagementApi'
import { TEMPLATE_TYPE_NAMES, loadProjectOptions } from './templateOptions'

type BoxunTemplateManagementFormData = Parameters<typeof BoxunTemplateManagementApi.createBoxunTemplateManagement>[0]
type EntityId = string | number
type ProjectOption = Awaited<ReturnType<typeof loadProjectOptions>>[number]

const props = withDefaults(defineProps<{ projects?: ProjectOption[] }>(), { projects: () => [] })
const emit = defineEmits<{ success: [] }>()
defineOptions({ name: 'BoxunTemplateManagementForm' })

const message = useMessage()
const { t } = useI18n()
const dialogVisible = ref(false)
const dialogTitle = ref('')
const formLoading = ref(false)
const uploading = ref(false)
const formType = ref<'create' | 'update'>('create')
const editingId = ref<EntityId>()
const formRef = ref()
const initialFormData = () =>
  ({
    projectId: '',
    enableOrNot: 1,
    templateUrl: '',
    templateDescription: '',
    templateType: undefined,
    bytecodeRef: ''
  }) as BoxunTemplateManagementFormData
const formData = ref(initialFormData())
const formRules = reactive({
  projectId: [{ required: true, message: '请选择项目名称', trigger: 'change' }],
  templateUrl: [{ required: true, message: '请上传模板', trigger: 'change' }],
  templateType: [{ required: true, message: '请选择模板类型', trigger: 'change' }],
  enableOrNot: [{ required: true, message: '请选择是否启用', trigger: 'change' }]
})

const open = async (type: 'create' | 'update', id?: EntityId, defaults?: Partial<BoxunTemplateManagementFormData>) => {
  formType.value = type
  editingId.value = id
  dialogTitle.value = type === 'create' ? '新增模板管理' : '修改模板管理'
  formData.value = { ...initialFormData(), ...defaults }
  formLoading.value = true
  dialogVisible.value = true
  try {
    if (id !== undefined) {
      formData.value = { ...initialFormData(), ...(await BoxunTemplateManagementApi.getBoxunTemplateManagement(id)) }
    }
  } finally {
    formLoading.value = false
    await nextTick()
    formRef.value?.clearValidate()
  }
}
defineExpose({ open })

const handleTemplateUpload = async (options: UploadRequestOptions) => {
  const form = new FormData()
  form.append('file', options.file)
  uploading.value = true
  try {
    const url = await updateFile(form)
    formData.value.templateUrl = String(url ?? '')
    message.success('上传成功')
  } finally {
    uploading.value = false
  }
}

const submitForm = async () => {
  if (formLoading.value) return
  const valid = await formRef.value?.validate().catch(() => false)
  if (!valid) return
  const inputFields = ['projectId', 'enableOrNot', 'templateUrl', 'templateDescription', 'templateType', 'bytecodeRef']
  const input = Object.fromEntries(inputFields.map((key) => [key, formData.value[key]]))
  formLoading.value = true
  try {
    if (formType.value === 'create') {
      await BoxunTemplateManagementApi.createBoxunTemplateManagement(input)
      message.success(t('common.createSuccess'))
    } else {
      await BoxunTemplateManagementApi.updateBoxunTemplateManagement({ ...input, id: editingId.value })
      message.success(t('common.updateSuccess'))
    }
    dialogVisible.value = false
    emit('success')
  } finally {
    formLoading.value = false
  }
}
</script>

<style scoped>
.template-file {
  display: block;
  max-width: 100%;
  margin-top: 8px;
  overflow: hidden;
  color: var(--el-text-color-secondary);
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
