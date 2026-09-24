<script setup lang="ts">
import { computed, nextTick, reactive, ref, watch } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { t } from '@/locales'
import {
  categoryOptions,
  queryRecords,
  type ExampleRecord,
  type RecordInput,
  type RecordStatus
} from './model.mjs'
import { errorKey, useExamples } from './store'

const props = withDefaults(
  defineProps<{ categoryId?: string | null; showCategoryFilter?: boolean }>(),
  {
    categoryId: null,
    showCategoryFilter: true
  }
)
const examples = useExamples()
const { state } = examples
const search = reactive({ keyword: '', categoryId: '', status: '' as RecordStatus | '' })
const appliedSearch = reactive({ ...search })
const page = ref(1)
const pageSize = ref(10)
const options = computed(() => categoryOptions(state.value.categories))
const categoryLabels = computed(
  () => new Map(options.value.map((option) => [option.value, option.label]))
)
const selectedCategoryName = computed(() =>
  props.categoryId ? categoryLabels.value.get(props.categoryId) : ''
)
const result = computed(() =>
  queryRecords(state.value, {
    ...appliedSearch,
    categoryId: props.categoryId || appliedSearch.categoryId,
    page: page.value,
    pageSize: pageSize.value
  })
)

watch(
  () => props.categoryId,
  () => {
    page.value = 1
  }
)
watch(
  () => result.value.page,
  (value) => {
    page.value = value
  }
)

function runSearch() {
  Object.assign(appliedSearch, search)
  page.value = 1
}

function resetSearch() {
  Object.assign(search, { keyword: '', categoryId: '', status: '' })
  runSearch()
}

const dialogVisible = ref(false)
const saving = ref(false)
const editingId = ref<string | null>(null)
const formRef = ref<FormInstance>()
const form = reactive<RecordInput>({ name: '', code: '', categoryId: '', status: 'enabled' })
const rules: FormRules<RecordInput> = {
  name: [
    {
      required: true,
      whitespace: true,
      message: t('examples.errors.nameRequired'),
      trigger: 'blur'
    },
    { max: 60, message: t('examples.errors.nameTooLong'), trigger: 'blur' }
  ],
  code: [
    {
      required: true,
      pattern: /^[A-Za-z0-9_-]{2,32}$/,
      message: t('examples.errors.codeFormat'),
      trigger: 'blur',
      transform: (value: string) => value.trim()
    },
    {
      validator: (_rule, value: string, callback) => {
        const duplicate = state.value.records.some(
          (record) =>
            record.id !== editingId.value &&
            record.code.toLocaleLowerCase() === value.trim().toLocaleLowerCase()
        )
        callback(duplicate ? new Error(t('examples.errors.codeDuplicate')) : undefined)
      },
      trigger: 'blur'
    }
  ],
  categoryId: [
    { required: true, message: t('examples.errors.categoryRequired'), trigger: 'change' }
  ],
  status: [{ required: true, message: t('examples.errors.statusRequired'), trigger: 'change' }]
}

async function openForm(record?: ExampleRecord) {
  editingId.value = record?.id ?? null
  Object.assign(
    form,
    record
      ? {
          name: record.name,
          code: record.code,
          categoryId: record.categoryId,
          status: record.status
        }
      : {
          name: '',
          code: '',
          categoryId: props.categoryId || appliedSearch.categoryId || options.value[0]?.value || '',
          status: 'enabled'
        }
  )
  dialogVisible.value = true
  await nextTick()
  formRef.value?.clearValidate()
}

async function submitForm() {
  if (!formRef.value || saving.value) {
    return
  }
  const valid = await formRef.value.validate().catch(() => false)
  if (!valid) {
    return
  }
  saving.value = true
  try {
    examples.saveRecord({ ...form }, editingId.value)
    dialogVisible.value = false
    page.value = 1
    ElMessage.success(t('examples.feedback.saved'))
  } catch (error) {
    ElMessage.error(t(errorKey(error)))
  } finally {
    saving.value = false
  }
}

async function removeRecord(record: ExampleRecord) {
  try {
    await ElMessageBox.confirm(
      `${t('examples.confirm.record')}\n${record.name} (${record.code})`,
      t('examples.confirm.title'),
      {
        confirmButtonText: t('examples.action.confirm'),
        cancelButtonText: t('examples.action.cancel'),
        type: 'warning'
      }
    )
  } catch {
    return
  }
  try {
    examples.deleteRecord(record.id)
    ElMessage.success(t('examples.feedback.deleted'))
  } catch (error) {
    ElMessage.error(t(errorKey(error)))
  }
}

const dateFormatter = new Intl.DateTimeFormat('zh-CN', {
  year: 'numeric',
  month: '2-digit',
  day: '2-digit',
  hour: '2-digit',
  minute: '2-digit',
  hour12: false
})
</script>

<template>
  <section class="page-card records-panel">
    <el-form class="search-form" :inline="true" :model="search" @submit.prevent="runSearch">
      <el-form-item :label="t('examples.search.keyword')">
        <el-input
          v-model="search.keyword"
          :placeholder="t('examples.search.placeholder')"
          clearable
          maxlength="60"
        />
      </el-form-item>
      <el-form-item v-if="showCategoryFilter" :label="t('examples.search.category')">
        <el-select
          v-model="search.categoryId"
          :placeholder="t('examples.search.allCategories')"
          clearable
          filterable
        >
          <el-option
            v-for="option in options"
            :key="option.value"
            :label="option.label"
            :value="option.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item :label="t('examples.search.status')">
        <el-select
          v-model="search.status"
          :placeholder="t('examples.search.allStatuses')"
          clearable
        >
          <el-option :label="t('examples.status.enabled')" value="enabled" />
          <el-option :label="t('examples.status.disabled')" value="disabled" />
        </el-select>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" native-type="submit">{{ t('examples.action.search') }}</el-button>
        <el-button @click="resetSearch">{{ t('examples.action.reset') }}</el-button>
      </el-form-item>
    </el-form>

    <div class="table-toolbar">
      <div class="records-heading">
        <h2>{{ t('examples.table.title') }}</h2>
        <span class="record-count">{{ result.total }} {{ t('examples.table.total') }}</span>
        <el-tag v-if="selectedCategoryName" effect="plain"
          >{{ t('examples.table.scope') }}：{{ selectedCategoryName }}</el-tag
        >
      </div>
      <el-button type="primary" @click="openForm()">{{ t('examples.action.add') }}</el-button>
    </div>

    <el-table :data="result.items" row-key="id" stripe :empty-text="t('examples.table.empty')">
      <el-table-column
        prop="name"
        :label="t('examples.table.name')"
        min-width="150"
        show-overflow-tooltip
      />
      <el-table-column
        prop="code"
        :label="t('examples.table.code')"
        min-width="130"
        show-overflow-tooltip
      />
      <el-table-column :label="t('examples.table.category')" min-width="180" show-overflow-tooltip>
        <template #default="{ row }">{{ categoryLabels.get(row.categoryId) }}</template>
      </el-table-column>
      <el-table-column :label="t('examples.table.status')" width="90">
        <template #default="{ row }">
          <el-tag :type="row.status === 'enabled' ? 'success' : 'info'" size="small">{{
            t(`examples.status.${row.status}`)
          }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column :label="t('examples.table.updatedAt')" width="170">
        <template #default="{ row }">{{ dateFormatter.format(new Date(row.updatedAt)) }}</template>
      </el-table-column>
      <el-table-column :label="t('examples.table.actions')" width="126" fixed="right">
        <template #default="{ row }">
          <div class="table-actions">
            <el-button type="primary" link @click="openForm(row)">{{
              t('examples.action.edit')
            }}</el-button>
            <el-button type="danger" link @click="removeRecord(row)">{{
              t('examples.action.delete')
            }}</el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>
    <div class="records-pagination">
      <el-pagination
        v-model:current-page="page"
        v-model:page-size="pageSize"
        :page-sizes="[10, 20, 50]"
        :total="result.total"
        layout="total, sizes, prev, pager, next, jumper"
        background
        @size-change="page = 1"
      />
    </div>

    <el-dialog
      v-model="dialogVisible"
      :title="t(editingId ? 'examples.form.editTitle' : 'examples.form.addTitle')"
      width="520px"
      style="max-width: calc(100vw - 32px)"
      :close-on-click-modal="false"
      destroy-on-close
    >
      <el-alert
        v-if="options.length === 0"
        :title="t('examples.form.noCategories')"
        type="warning"
        :closable="false"
        class="category-warning"
      />
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        @submit.prevent="submitForm"
      >
        <el-form-item :label="t('examples.table.name')" prop="name">
          <el-input
            v-model="form.name"
            :placeholder="t('examples.form.namePlaceholder')"
            maxlength="60"
            show-word-limit
          />
        </el-form-item>
        <el-form-item :label="t('examples.table.code')" prop="code">
          <el-input
            v-model="form.code"
            :placeholder="t('examples.form.codePlaceholder')"
            maxlength="32"
          />
          <span class="form-hint">{{ t('examples.form.codeHint') }}</span>
        </el-form-item>
        <el-form-item :label="t('examples.table.category')" prop="categoryId">
          <el-select
            v-model="form.categoryId"
            :placeholder="t('examples.form.categoryPlaceholder')"
            filterable
            class="full-width"
          >
            <el-option
              v-for="option in options"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </el-form-item>
        <el-form-item :label="t('examples.table.status')" prop="status">
          <el-radio-group v-model="form.status">
            <el-radio value="enabled">{{ t('examples.status.enabled') }}</el-radio>
            <el-radio value="disabled">{{ t('examples.status.disabled') }}</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">{{ t('examples.action.cancel') }}</el-button>
        <el-button
          type="primary"
          :loading="saving"
          :disabled="options.length === 0"
          @click="submitForm"
          >{{ t('examples.action.save') }}</el-button
        >
      </template>
    </el-dialog>
  </section>
</template>

<style scoped>
.records-panel {
  min-width: 0;
}
.search-form {
  border-bottom: 1px solid var(--el-border-color-lighter);
  margin-bottom: 16px;
}
.search-form :deep(.el-form-item) {
  margin-right: 16px;
  margin-bottom: 16px;
}
.search-form :deep(.el-input) {
  width: 190px;
}
.search-form :deep(.el-select) {
  width: 180px;
}
.records-heading {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 10px;
}
.records-heading h2 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}
.record-count,
.form-hint {
  color: var(--el-text-color-secondary);
  font-size: 12px;
}
.records-pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: 20px;
  overflow-x: auto;
  padding-bottom: 4px;
}
.form-hint {
  line-height: 1.6;
  margin-top: 6px;
}
.category-warning {
  margin-bottom: 16px;
}
.full-width {
  width: 100%;
}
@media (max-width: 600px) {
  .search-form :deep(.el-form-item) {
    display: flex;
    margin-right: 0;
  }
  .search-form :deep(.el-input),
  .search-form :deep(.el-select) {
    width: 100%;
  }
  .records-pagination {
    justify-content: flex-start;
  }
}
</style>
