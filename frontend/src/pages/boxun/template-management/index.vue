<!-- 模板管理为业务定制页面，保留手写维护，避免 Studio 前端生成覆盖。 -->
<template>
  <ContentWrap>
    <el-form ref="queryFormRef" :inline="true" :model="queryParams" class="-mb-15px">
      <el-form-item label="项目名称">
        <el-select
          v-model="queryParams.projectId"
          clearable
          filterable
          placeholder="请选择项目名称"
          class="!w-260px"
          @change="handleQuery"
        >
          <el-option v-for="project in projects" :key="project.id" :label="project.name" :value="project.id" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button @click="handleQuery"><Icon class="mr-5px" icon="ep:search" />查询</el-button>
        <el-button @click="resetQuery"><Icon class="mr-5px" icon="ep:refresh" />重置</el-button>
        <el-button type="primary" @click="openForm('create')">
          <Icon class="mr-5px" icon="ep:plus" />新增
        </el-button>
        <el-button type="success" @click="handleInheritPreset">
          <Icon class="mr-5px" icon="ep:copy-document" />为项目添加预设模板
        </el-button>
        <el-button :loading="exportLoading" type="success" @click="handleExport">
          <Icon class="mr-5px" icon="ep:download" />导出
        </el-button>
        <el-button :disabled="checkedIds.length === 0" type="danger" @click="handleDeleteBatch">
          <Icon class="mr-5px" icon="ep:delete" />批量删除
        </el-button>
      </el-form-item>
    </el-form>
  </ContentWrap>

  <ContentWrap>
    <el-table v-loading="loading" :data="rows" @selection-change="handleSelectionChange">
      <el-table-column align="center" type="selection" width="55" />
      <el-table-column align="center" label="行号" width="90">
        <template #default="scope">{{ (queryParams.pageNo - 1) * queryParams.pageSize + scope.$index + 1 }}</template>
      </el-table-column>
      <el-table-column label="项目名称" prop="projectId" min-width="180" show-overflow-tooltip>
        <template #default="scope">{{ projectName(scope.row.projectId) }}</template>
      </el-table-column>
      <el-table-column label="模板类型" prop="templateType" min-width="140" show-overflow-tooltip>
        <template #default="scope">{{ templateTypeName(scope.row.templateType) }}</template>
      </el-table-column>
      <el-table-column label="模板下载" prop="templateUrl" min-width="180" show-overflow-tooltip>
        <template #default="scope">
          <el-button v-if="scope.row.templateUrl" link type="primary" @click="handleDownload(scope.row.templateUrl)">
            下载
          </el-button>
          <span v-else class="text-gray-400">无文件</span>
        </template>
      </el-table-column>
      <el-table-column label="是否启用" prop="enableOrNot" min-width="100">
        <template #default="scope">
          <el-tag :type="scope.row.enableOrNot === 1 ? 'success' : 'info'">
            {{ scope.row.enableOrNot === 1 ? '启用' : '停用' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="模板描述" prop="templateDescription" min-width="140" show-overflow-tooltip />
      <el-table-column label="字节码包路径" prop="bytecodeRef" min-width="160" show-overflow-tooltip />
      <el-table-column align="center" fixed="right" label="操作" width="260">
        <template #default="scope">
          <el-button link type="primary" @click="openDetail(scope.row.id)">查看</el-button>
          <el-button link type="primary" @click="openFieldMapping(scope.row.id)">字段映射</el-button>
          <el-button link type="primary" @click="openForm('update', scope.row.id)">编辑</el-button>
          <el-button link type="danger" @click="handleDelete(scope.row.id)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
    <Pagination v-model:limit="queryParams.pageSize" v-model:page="queryParams.pageNo" :total="total" @pagination="getList" />
  </ContentWrap>

  <BoxunTemplateManagementForm ref="formRef" :projects="projects" @success="getList" />
  <BoxunTemplateManagementDetail ref="detailRef" />
  <TemplateFieldMappingDialog ref="fieldMappingRef" />
</template>

<script setup lang="ts">
import request from '@/config/axios'
import download from '@/utils/download'
import * as BoxunTemplateManagementApi from './BoxunTemplateManagementApi'
import BoxunTemplateManagementForm from './BoxunTemplateManagementForm.vue'
import BoxunTemplateManagementDetail from './BoxunTemplateManagementDetail.vue'
import TemplateFieldMappingDialog from './TemplateFieldMappingDialog.vue'
import { loadProjectOptions, templateTypeName } from './templateOptions'

type BoxunTemplateManagementRow = Awaited<ReturnType<typeof BoxunTemplateManagementApi.getBoxunTemplateManagementPage>>['list'][number]
type EntityId = string | number
type ProjectOption = Awaited<ReturnType<typeof loadProjectOptions>>[number]

defineOptions({ name: 'BoxunTemplateManagement' })

const message = useMessage()
const { t } = useI18n()
const loading = ref(false)
const total = ref(0)
const rows = ref<BoxunTemplateManagementRow[]>([])
const projects = ref<ProjectOption[]>([])
const queryFormRef = ref()
const queryParams = reactive({
  pageNo: 1,
  pageSize: 10,
  projectId: ''
})

const projectName = (value?: string) => projects.value.find((project) => project.id === value)?.name ?? value ?? '-'

const getList = async () => {
  loading.value = true
  try {
    const data = await BoxunTemplateManagementApi.getBoxunTemplateManagementPage(queryParams)
    rows.value = data.list
    total.value = Number(data.total)
  } finally {
    loading.value = false
  }
}

const handleQuery = () => {
  queryParams.pageNo = 1
  getList()
}

const resetQuery = () => {
  queryFormRef.value?.resetFields()
  queryParams.projectId = ''
  handleQuery()
}

const formRef = ref()
const openForm = (type: 'create' | 'update', id?: EntityId) => {
  formRef.value?.open(type, id, type === 'create' ? { projectId: queryParams.projectId } : undefined)
}

const detailRef = ref()
const openDetail = (id: EntityId) => {
  detailRef.value?.open(id)
}

const fieldMappingRef = ref()
const openFieldMapping = (id: EntityId) => {
  fieldMappingRef.value?.open(id)
}

const handleDelete = async (id?: EntityId) => {
  if (id === undefined) return
  await message.delConfirm()
  await BoxunTemplateManagementApi.deleteBoxunTemplateManagement(id)
  message.success(t('common.delSuccess'))
  await getList()
}

const checkedIds = ref<EntityId[]>([])
const handleSelectionChange = (rows: BoxunTemplateManagementRow[]) => {
  checkedIds.value = rows.map((row) => row.id).filter((id): id is EntityId => id !== undefined)
}

const handleDeleteBatch = async () => {
  await message.delConfirm()
  await BoxunTemplateManagementApi.deleteBoxunTemplateManagementList(checkedIds.value)
  message.success(t('common.delSuccess'))
  checkedIds.value = []
  await getList()
}

const handleInheritPreset = async () => {
  const currentProjectId = String(queryParams.projectId ?? '')
  if (!currentProjectId) {
    message.warning('请选择项目')
    return
  }
  await message.confirm('确定要为当前项目添加预设模板吗？已存在的模板类型不会重复添加。')
  await BoxunTemplateManagementApi.inheritBoxunTemplateManagement(currentProjectId)
  message.success('添加成功')
  await getList()
}

const handleDownload = (url: string) => {
  window.open(url, '_blank', 'noopener,noreferrer')
}

const exportLoading = ref(false)
const handleExport = async () => {
  await message.exportConfirm()
  exportLoading.value = true
  try {
    const data = await request.download({ url: '/boxun/template-management/export-excel', params: queryParams })
    download.excel(data, 'boxunTemplateManagement.xls')
  } finally {
    exportLoading.value = false
  }
}

onMounted(async () => {
  projects.value = await loadProjectOptions()
  await getList()
})
</script>
