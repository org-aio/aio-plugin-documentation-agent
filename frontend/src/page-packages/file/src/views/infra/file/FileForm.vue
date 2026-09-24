<template>
  <Dialog v-model="dialogVisible" title="上传文件">
    <el-upload
      ref="uploadRef"
      v-model:file-list="fileList"
      :auto-upload="false"
      :disabled="formLoading"
      :on-change="handleFileChange"
      multiple
      drag
    >
      <i class="el-icon-upload"></i>
      <div class="el-upload__text"> 将文件拖到此处，或 <em>点击上传</em></div>
      <template #tip>
        <div class="el-upload__tip">支持单文件、多文件上传，也可以选择文件夹批量上传。</div>
      </template>
    </el-upload>
    <div class="upload-options-row">
      <span class="upload-options-label">命名方式</span>
      <el-radio-group v-model="namingMode" :disabled="formLoading">
        <el-radio-button label="keep">保留原文件名</el-radio-button>
        <el-radio-button label="auto">自动命名</el-radio-button>
      </el-radio-group>
    </div>
    <div class="folder-upload-row">
      <el-button :disabled="formLoading" @click="selectFolder">
        <Icon icon="ep:folder-opened" class="mr-5px" /> 选择文件夹
      </el-button>
      <span v-if="folderFiles.length > 0" class="folder-upload-info">
        已选择 {{ folderName }}，共 {{ folderFiles.length }} 个文件
      </span>
    </div>
    <template #footer>
      <el-button :disabled="formLoading" type="primary" @click="submitFileForm">确 定</el-button>
      <el-button @click="dialogVisible = false">取 消</el-button>
    </template>
  </Dialog>
</template>
<script lang="ts" setup>
import * as FileApi from '@/api/infra/file'
import type { UploadFile, UploadInstance, UploadRawFile, UploadUserFile } from 'element-plus'

defineOptions({ name: 'InfraFileForm' })

const { t } = useI18n() // 国际化
const message = useMessage() // 消息弹窗

const dialogVisible = ref(false) // 弹窗的是否展示
const formLoading = ref(false) // 表单的加载中
const fileList = ref<UploadUserFile[]>([]) // 文件列表
const uploadRef = ref<UploadInstance>()
const folderFiles = ref<File[]>([])
const namingMode = ref<'keep' | 'auto'>('keep')
const keepOriginalName = computed(() => namingMode.value === 'keep')

const folderName = computed(() => getFolderName(folderFiles.value[0]))

/** 打开弹窗 */
const open = async () => {
  dialogVisible.value = true
  resetForm()
}
defineExpose({ open }) // 提供 open 方法，用于打开弹窗

/** 处理上传的文件发生变化 */
const handleFileChange = (_file: UploadFile) => {
  folderFiles.value = []
}

/** 提交表单 */
const submitFileForm = () => {
  if (folderFiles.value.length > 0) {
    submitFiles(folderFiles.value, getRelativePath)
    return
  }
  const files = getSelectedFiles()
  if (files.length == 0) {
    message.error('请上传文件')
    return
  }
  submitFiles(files, (file) => file.name)
}

/** 选择文件夹 */
const selectFolder = () => {
  const input = document.createElement('input') as DirectoryInputElement
  input.type = 'file'
  input.multiple = true
  input.webkitdirectory = true
  input.directory = true
  input.style.display = 'none'
  input.setAttribute('webkitdirectory', 'webkitdirectory')
  input.setAttribute('directory', 'directory')
  input.addEventListener('change', () => {
    handleFolderFiles(input.files)
    input.remove()
  })
  document.body.appendChild(input)
  input.click()
}

/** 处理文件夹选择 */
const handleFolderFiles = (files: FileList | null) => {
  folderFiles.value = Array.from(files || [])
  if (folderFiles.value.length === 0) {
    return
  }
  fileList.value = []
  uploadRef.value?.clearFiles()
}

/** 批量提交文件 */
const submitFiles = async (files: File[], getPath: (file: File) => string) => {
  formLoading.value = true
  try {
    const formData = new FormData()
    files.forEach((file) => {
      formData.append('files', file)
      formData.append('relativePaths', getPath(file))
    })
    formData.append('keepOriginalName', String(keepOriginalName.value))
    await FileApi.uploadFolder(formData)
    submitFormSuccess()
  } catch {
    submitFormError()
  }
}

/** 文件上传成功处理 */
const emit = defineEmits(['success']) // 定义 success 事件，用于操作成功后的回调
const submitFormSuccess = () => {
  // 清理
  dialogVisible.value = false
  formLoading.value = false
  unref(uploadRef)?.clearFiles()
  folderFiles.value = []
  // 提示成功，并刷新
  message.success(t('common.createSuccess'))
  emit('success')
}

/** 上传错误提示 */
const submitFormError = (): void => {
  message.error('上传失败，请您重新上传！')
  formLoading.value = false
}

/** 重置表单 */
const resetForm = () => {
  // 重置上传状态和文件
  formLoading.value = false
  uploadRef.value?.clearFiles()
  folderFiles.value = []
}

const getRelativePath = (file: File) => {
  return (file as File & { webkitRelativePath?: string }).webkitRelativePath || file.name
}

const getSelectedFiles = () => {
  return fileList.value
    .map((file) => file.raw)
    .filter((file): file is UploadRawFile => Boolean(file))
}

const getFolderName = (file?: File) => {
  if (!file) {
    return ''
  }
  return getRelativePath(file).split('/')[0]
}

type DirectoryInputElement = HTMLInputElement & {
  webkitdirectory: boolean
  directory: boolean
}
</script>
<style scoped>
.folder-upload-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
}

.upload-options-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
}

.upload-options-label {
  color: var(--el-text-color-regular);
  font-size: 14px;
}

.folder-upload-info {
  color: var(--el-text-color-secondary);
  font-size: 13px;
}

</style>
