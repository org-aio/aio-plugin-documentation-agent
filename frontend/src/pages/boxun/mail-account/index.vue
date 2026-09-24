<template>
  <ContentWrap>
    <el-form ref="queryFormRef" :inline="true" :model="queryParams" class="-mb-15px">
      <el-form-item label="发件邮箱" prop="mail">
        <el-input v-model="queryParams.mail" clearable placeholder="请输入发件邮箱" @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item label="SMTP 服务器" prop="host">
        <el-input v-model="queryParams.host" clearable placeholder="请输入 SMTP 服务器" @keyup.enter="handleQuery" />
      </el-form-item>
      <el-form-item>
        <el-button @click="handleQuery"><Icon class="mr-5px" icon="ep:search" />查询</el-button>
        <el-button @click="resetQuery"><Icon class="mr-5px" icon="ep:refresh" />重置</el-button>
        <el-button type="primary" @click="openForm('create')">
          <Icon class="mr-5px" icon="ep:plus" />新增
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
      <el-table-column label="发件邮箱" prop="mail" min-width="200" show-overflow-tooltip />
      <el-table-column label="用户名" prop="username" min-width="160" show-overflow-tooltip />
      <el-table-column label="SMTP 服务器" prop="host" min-width="200" show-overflow-tooltip />
      <el-table-column label="端口" prop="port" width="100" />
      <el-table-column label="SSL" width="90">
        <template #default="scope">{{ scope.row.sslEnable ? '是' : '否' }}</template>
      </el-table-column>
      <el-table-column label="STARTTLS" width="110">
        <template #default="scope">{{ scope.row.starttlsEnable ? '是' : '否' }}</template>
      </el-table-column>
      <el-table-column align="center" fixed="right" label="操作" width="150">
        <template #default="scope">
          <el-button link type="primary" @click="openForm('update', scope.row.id)">编辑</el-button>
          <el-button link type="danger" @click="handleDelete(scope.row.id)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
    <Pagination v-model:limit="queryParams.pageSize" v-model:page="queryParams.pageNo" :total="total" @pagination="getList" />
  </ContentWrap>

  <BoxunMailAccountForm ref="formRef" @success="getList" />
</template>

<script setup lang="ts">
import * as BoxunMailAccountApi from './BoxunMailAccountApi'
import BoxunMailAccountForm from './BoxunMailAccountForm.vue'

type BoxunMailAccountRow = Awaited<ReturnType<typeof BoxunMailAccountApi.getBoxunMailAccountPage>>['list'][number]
type EntityId = string | number

defineOptions({ name: 'BoxunMailAccount' })

const message = useMessage()
const { t } = useI18n()
const loading = ref(false)
const total = ref(0)
const rows = ref<BoxunMailAccountRow[]>([])
const queryFormRef = ref()
const queryParams = reactive({
  pageNo: 1,
  pageSize: 10,
  mail: '',
  host: ''
})

const getList = async () => {
  loading.value = true
  try {
    const data = await BoxunMailAccountApi.getBoxunMailAccountPage(queryParams)
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
  handleQuery()
}

const formRef = ref()
const openForm = (type: 'create' | 'update', id?: EntityId) => {
  formRef.value?.open(type, id)
}

const handleDelete = async (id?: EntityId) => {
  if (id === undefined) return
  await message.delConfirm()
  await BoxunMailAccountApi.deleteBoxunMailAccount(id)
  message.success(t('common.delSuccess'))
  await getList()
}

const checkedIds = ref<EntityId[]>([])
const handleSelectionChange = (rows: BoxunMailAccountRow[]) => {
  checkedIds.value = rows.map((row) => row.id).filter((id): id is EntityId => id !== undefined)
}

const handleDeleteBatch = async () => {
  await message.delConfirm()
  await BoxunMailAccountApi.deleteBoxunMailAccountList(checkedIds.value)
  message.success(t('common.delSuccess'))
  checkedIds.value = []
  await getList()
}

onMounted(getList)
</script>
