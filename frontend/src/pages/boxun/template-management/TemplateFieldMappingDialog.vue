<!-- 模板管理为业务定制页面，保留手写维护，避免 Studio 前端生成覆盖。 -->
<template>
  <Dialog v-model="dialogVisible" :title="`${businessDescription}字段映射`" width="760">
    <el-table v-loading="loading" :data="fields" border max-height="520">
      <el-table-column type="index" label="#" width="60" align="center" />
      <el-table-column label="中文" prop="name" min-width="180" show-overflow-tooltip />
      <el-table-column label="字段名" prop="fieldName" min-width="220" show-overflow-tooltip />
    </el-table>
  </Dialog>
</template>

<script setup lang="ts">
import * as BoxunTemplateManagementApi from './BoxunTemplateManagementApi'

type EntityId = string | number
type TemplateFieldMapping = BoxunTemplateManagementApi.TemplateFieldMapping

defineOptions({ name: 'TemplateFieldMappingDialog' })

const dialogVisible = ref(false)
const loading = ref(false)
const fields = ref<TemplateFieldMapping[]>([])
const businessDescription = ref('模板')

const open = async (id: EntityId) => {
  fields.value = []
  businessDescription.value = '模板'
  dialogVisible.value = true
  loading.value = true
  try {
    const result = await BoxunTemplateManagementApi.getBoxunTemplateManagementFieldMapping(id)
    fields.value = result.beanDesDTOS ?? []
    businessDescription.value = result.businessDescription || '模板'
  } finally {
    loading.value = false
  }
}

defineExpose({ open })
</script>
