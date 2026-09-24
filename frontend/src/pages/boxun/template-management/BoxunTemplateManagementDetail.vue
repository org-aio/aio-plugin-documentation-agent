<!-- 模板管理为业务定制页面，放在手写生成目录以避免后续 Studio 前端生成覆盖。 -->
<template>
  <Dialog v-model="dialogVisible" title="查看模板管理" width="760">
    <el-descriptions v-loading="loading" :column="2" border>
      <el-descriptions-item label="项目id">
        {{ projectName(detail?.projectId) }}
      </el-descriptions-item>
      <el-descriptions-item label="是否启用">
        {{ detail?.enableOrNot === 1 ? '启用' : '停用' }}
      </el-descriptions-item>
      <el-descriptions-item label="模板url">
        {{ detail?.templateUrl ?? '-' }}
      </el-descriptions-item>
      <el-descriptions-item label="模板描述">
        {{ detail?.templateDescription ?? '-' }}
      </el-descriptions-item>
      <el-descriptions-item label="模板类型">
        {{ templateTypeName(detail?.templateType) }}
      </el-descriptions-item>
      <el-descriptions-item label="字节码包路径">
        {{ detail?.bytecodeRef ?? '-' }}
      </el-descriptions-item>
    </el-descriptions>
  </Dialog>
</template>

<script setup lang="ts">
import * as BoxunTemplateManagementApi from './BoxunTemplateManagementApi'
import { templateTypeName, loadProjectNames } from './templateOptions'

type BoxunTemplateManagementDetail = Awaited<ReturnType<typeof BoxunTemplateManagementApi.getBoxunTemplateManagement>>
type EntityId = string | number

defineOptions({ name: 'BoxunTemplateManagementDetail' })

const dialogVisible = ref(false)
const loading = ref(false)
const detail = ref<BoxunTemplateManagementDetail>()
const projectNames = ref<Record<string, string>>({})

const open = async (id: EntityId) => {
  detail.value = undefined
  dialogVisible.value = true
  loading.value = true
  try {
    detail.value = await BoxunTemplateManagementApi.getBoxunTemplateManagement(id)
    projectNames.value = await loadProjectNames()
  } finally {
    loading.value = false
  }
}

const projectName = (value?: string) => projectNames.value[value ?? ''] ?? value ?? '-'
defineExpose({ open })
</script>
