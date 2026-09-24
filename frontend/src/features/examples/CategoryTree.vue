<script setup lang="ts">
import { computed, nextTick, reactive, ref, watch } from 'vue'
import {
  ElMessage,
  ElMessageBox,
  type FormInstance,
  type FormRules,
  type TreeInstance
} from 'element-plus'
import { t } from '@/locales'
import { buildCategoryTree, deleteCategory, type CategoryNode } from './model.mjs'
import { errorKey, useExamples } from './store'

const props = defineProps<{ modelValue: string | null }>()
const emit = defineEmits<{ 'update:modelValue': [value: string | null] }>()
const examples = useExamples()
const { state } = examples
const treeRef = ref<TreeInstance>()
const filterText = ref('')
const tree = computed(() => buildCategoryTree(state.value))
const selected = computed(() =>
  state.value.categories.find((category) => category.id === props.modelValue)
)

watch(filterText, (value) => treeRef.value?.filter(value))
watch(
  () => props.modelValue,
  async (value) => {
    await nextTick()
    treeRef.value?.setCurrentKey(value ?? undefined)
  }
)
watch(
  () => state.value.categories,
  async () => {
    if (props.modelValue && !selected.value) {
      emit('update:modelValue', null)
    }
    await nextTick()
    treeRef.value?.filter(filterText.value)
  }
)

function matchesFilter(value: string, data: CategoryNode) {
  return data.name.toLocaleLowerCase().includes(value.trim().toLocaleLowerCase())
}

function selectCategory(data: CategoryNode) {
  emit('update:modelValue', data.id)
}

function showAll() {
  filterText.value = ''
  emit('update:modelValue', null)
  treeRef.value?.setCurrentKey(undefined)
}

const dialogVisible = ref(false)
const saving = ref(false)
const editingId = ref<string | null>(null)
const formRef = ref<FormInstance>()
const form = reactive({ name: '', parentId: null as string | null })
const parentName = computed(
  () =>
    state.value.categories.find((category) => category.id === form.parentId)?.name ||
    t('examples.tree.root')
)
const rules: FormRules<typeof form> = {
  name: [
    {
      required: true,
      whitespace: true,
      min: 1,
      max: 30,
      message: t('examples.errors.categoryName'),
      trigger: 'blur'
    },
    {
      validator: (_rule, value: string, callback) => {
        const duplicate = state.value.categories.some(
          (category) =>
            category.id !== editingId.value &&
            category.parentId === form.parentId &&
            category.name.toLocaleLowerCase() === value.trim().toLocaleLowerCase()
        )
        callback(duplicate ? new Error(t('examples.errors.categoryDuplicate')) : undefined)
      },
      trigger: 'blur'
    }
  ]
}

async function openForm(mode: 'root' | 'child' | 'rename') {
  const category = selected.value
  if (mode !== 'root' && !category) {
    return
  }
  const renaming = mode === 'rename'
  editingId.value = renaming ? category?.id ?? null : null
  form.name = renaming ? category?.name ?? '' : ''
  form.parentId = renaming
    ? category?.parentId ?? null
    : mode === 'child'
      ? category?.id ?? null
      : null
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
    examples.saveCategory({ ...form }, editingId.value)
    const categoryId = editingId.value || state.value.categories.at(-1)?.id || null
    filterText.value = ''
    emit('update:modelValue', categoryId)
    dialogVisible.value = false
    ElMessage.success(t('examples.feedback.categorySaved'))
  } catch (error) {
    ElMessage.error(t(errorKey(error)))
  } finally {
    saving.value = false
  }
}

async function removeCategory() {
  const category = selected.value
  if (!category) {
    return
  }
  try {
    deleteCategory(state.value, category.id)
  } catch (error) {
    ElMessage.error(t(errorKey(error)))
    return
  }
  try {
    await ElMessageBox.confirm(
      `${t('examples.confirm.category')}\n${category.name}`,
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
    examples.deleteCategory(category.id)
    emit('update:modelValue', category.parentId)
    ElMessage.success(t('examples.feedback.categoryDeleted'))
  } catch (error) {
    ElMessage.error(t(errorKey(error)))
  }
}
</script>

<template>
  <aside class="page-card category-panel" :aria-label="t('examples.tree.manageLabel')">
    <div class="category-heading">
      <h2>{{ t('examples.tree.title') }}</h2>
      <el-button type="primary" link @click="openForm('root')">{{
        t('examples.tree.addRoot')
      }}</el-button>
    </div>
    <el-input
      v-model="filterText"
      :placeholder="t('examples.tree.filterPlaceholder')"
      :aria-label="t('examples.tree.filterPlaceholder')"
      clearable
      maxlength="30"
    />
    <button
      class="all-records"
      :class="{ active: modelValue === null }"
      :aria-pressed="modelValue === null"
      @click="showAll"
    >
      <span>{{ t('examples.tree.all') }}</span>
      <span class="category-count">{{ state.records.length }}</span>
    </button>
    <el-tree
      ref="treeRef"
      class="category-tree"
      :data="tree"
      node-key="id"
      :props="{ label: 'name', children: 'children' }"
      :filter-node-method="matchesFilter"
      :current-node-key="modelValue ?? undefined"
      :empty-text="t('examples.tree.empty')"
      default-expand-all
      highlight-current
      :expand-on-click-node="false"
      @node-click="selectCategory"
    >
      <template #default="{ data }">
        <span class="category-node">
          <span class="category-name" :title="data.name">{{ data.name }}</span>
          <span class="category-count">{{ data.recordCount }}</span>
        </span>
      </template>
    </el-tree>
    <div class="category-management">
      <p class="selection-label">
        {{ t('examples.tree.selected') }}：<strong>{{
          selected?.name || t('examples.tree.all')
        }}</strong>
      </p>
      <div class="category-buttons">
        <el-button size="small" :disabled="!selected" @click="openForm('child')">{{
          t('examples.tree.addChild')
        }}</el-button>
        <el-button size="small" :disabled="!selected" @click="openForm('rename')">{{
          t('examples.tree.rename')
        }}</el-button>
        <el-button size="small" type="danger" plain :disabled="!selected" @click="removeCategory">{{
          t('examples.tree.delete')
        }}</el-button>
      </div>
      <p class="category-hint">{{ t('examples.tree.hint') }}</p>
    </div>

    <el-dialog
      v-model="dialogVisible"
      :title="t(editingId ? 'examples.tree.renameTitle' : 'examples.tree.addTitle')"
      width="440px"
      style="max-width: calc(100vw - 32px)"
      :close-on-click-modal="false"
      destroy-on-close
    >
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-position="top"
        @submit.prevent="submitForm"
      >
        <el-form-item :label="t('examples.tree.parent')">
          <el-input :model-value="parentName" disabled />
        </el-form-item>
        <el-form-item :label="t('examples.tree.name')" prop="name">
          <el-input
            v-model="form.name"
            :placeholder="t('examples.tree.namePlaceholder')"
            maxlength="30"
            show-word-limit
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">{{ t('examples.action.cancel') }}</el-button>
        <el-button type="primary" :loading="saving" @click="submitForm">{{
          t('examples.action.save')
        }}</el-button>
      </template>
    </el-dialog>
  </aside>
</template>

<style scoped>
.category-panel {
  min-width: 0;
  align-self: start;
}
.category-heading {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
}
.category-heading h2 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}
.all-records {
  display: flex;
  justify-content: space-between;
  width: 100%;
  margin-top: 14px;
  padding: 9px 10px;
  border: 0;
  border-radius: 4px;
  background: transparent;
  color: var(--el-text-color-primary);
  font: inherit;
  font-size: 14px;
  cursor: pointer;
}
.all-records:hover {
  background: var(--el-fill-color-light);
}
.all-records.active {
  color: var(--el-color-primary);
  background: var(--el-color-primary-light-9);
}
.all-records:focus-visible {
  outline: 2px solid var(--el-color-primary);
  outline-offset: 2px;
}
.category-tree {
  margin-top: 4px;
  min-height: 160px;
}
.category-tree :deep(.el-tree-node__content) {
  height: 36px;
  border-radius: 4px;
  padding-right: 10px;
}
.category-node {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex: 1;
  gap: 8px;
  min-width: 0;
}
.category-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.category-count {
  color: var(--el-text-color-secondary);
  font-size: 12px;
}
.category-management {
  border-top: 1px solid var(--el-border-color-lighter);
  margin-top: 16px;
  padding-top: 14px;
}
.selection-label {
  margin: 0 0 12px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
  overflow-wrap: anywhere;
}
.selection-label strong {
  color: var(--el-text-color-primary);
  font-weight: 500;
}
.category-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.category-buttons :deep(.el-button + .el-button) {
  margin-left: 0;
}
.category-hint {
  margin: 12px 0 0;
  color: var(--el-text-color-secondary);
  font-size: 12px;
  line-height: 1.7;
}
</style>
