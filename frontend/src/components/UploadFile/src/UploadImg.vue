<!-- 宿主文件上传组件：供 Studio 生成的表单使用，封装 /infra/file/upload。 -->
<template>
  <div class="upload-img">
    <el-upload
      :show-file-list="false"
      :http-request="handleUpload"
      accept="image/*"
      :disabled="disabled"
    >
      <img v-if="modelValue" :src="modelValue" class="upload-img__preview" alt="预览" />
      <el-button v-else :loading="uploading" :disabled="disabled">上传图片</el-button>
    </el-upload>
    <el-button v-if="modelValue" link type="danger" :disabled="disabled" @click="handleRemove">
      清除
    </el-button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage, type UploadRequestOptions } from 'element-plus'
import { updateFile } from '@/api/infra/file'

defineOptions({ name: 'UploadImg' })

withDefaults(defineProps<{ modelValue?: string; disabled?: boolean }>(), {
  modelValue: '',
  disabled: false
})
const emit = defineEmits<{ 'update:modelValue': [value: string] }>()

const uploading = ref(false)

const handleUpload = async (options: UploadRequestOptions) => {
  const formData = new FormData()
  formData.append('file', options.file)
  uploading.value = true
  try {
    const url = await updateFile(formData)
    emit('update:modelValue', String(url ?? ''))
    ElMessage.success('上传成功')
  } catch (error) {
    ElMessage.error(error instanceof Error ? error.message : '上传失败')
  } finally {
    uploading.value = false
  }
}

const handleRemove = () => emit('update:modelValue', '')
</script>

<style scoped>
.upload-img {
  display: flex;
  align-items: center;
  gap: 8px;
}
.upload-img__preview {
  max-width: 120px;
  max-height: 120px;
  object-fit: contain;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
}
</style>
