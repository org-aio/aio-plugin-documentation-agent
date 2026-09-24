<template>
  <Dialog v-model="dialogVisible" title="菜单权限">
    <el-form ref="formRef" v-loading="formLoading" :model="formData" label-width="80px">
      <el-form-item label="角色名称">
        <el-tag>{{ formData.name }}</el-tag>
      </el-form-item>
      <el-form-item label="角色标识">
        <el-tag>{{ formData.code }}</el-tag>
      </el-form-item>
      <el-form-item label="菜单权限">
        <el-card class="w-full h-400px !overflow-y-scroll" shadow="never">
          <template #header>
            全选/全不选:
            <el-switch
              v-model="treeNodeAll"
              active-text="是"
              inactive-text="否"
              inline-prompt
              @change="handleCheckedTreeNodeAll"
            />
            全部展开/折叠:
            <el-switch
              v-model="menuExpand"
              active-text="展开"
              inactive-text="折叠"
              inline-prompt
              @change="handleCheckedTreeExpand"
            />
          </template>
          <el-tree
            ref="treeRef"
            :data="menuOptions"
            :props="defaultProps"
            empty-text="加载中，请稍候"
            node-key="id"
            show-checkbox
          />
        </el-card>
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button :disabled="formLoading" type="primary" @click="submitForm">确 定</el-button>
      <el-button @click="dialogVisible = false">取 消</el-button>
    </template>
  </Dialog>
</template>
<script lang="ts" setup>
import type { ElTree, FormInstance } from 'element-plus'
import { defaultProps, handleTree } from '@/utils/tree'
import * as RoleApi from '@/api/system/role'
import * as MenuApi from '@/api/system/menu'
import * as PermissionApi from '@/api/system/permission'

defineOptions({ name: 'SystemRoleAssignMenuForm' })

const { t } = useI18n() // 国际化
const message = useMessage() // 消息弹窗

interface RoleMenuFormData {
  id?: number
  name: string
  code: string
  menuIds: number[]
}

const dialogVisible = ref(false) // 弹窗的是否展示
const formLoading = ref(false) // 表单的加载中：1）修改时的数据加载；2）提交的按钮禁用
const formData = reactive<RoleMenuFormData>({
  id: undefined,
  name: '',
  code: '',
  menuIds: []
})
const formRef = ref<FormInstance>() // 表单 Ref
const menuOptions = ref<any[]>([]) // 菜单树形结构
const menuExpand = ref(false) // 展开/折叠
const treeRef = ref<InstanceType<typeof ElTree>>() // 菜单树组件 Ref
const treeNodeAll = ref(false) // 全选/全不选

/** 打开弹窗 */
const open = async (row: RoleApi.RoleVO) => {
  dialogVisible.value = true
  resetForm()
  formLoading.value = true
  try {
    // 先加载菜单并等待树组件渲染，再恢复选中状态。
    menuOptions.value = handleTree(await MenuApi.getSimpleMenusList())
    formData.id = row.id
    formData.name = row.name
    formData.code = row.code
    formData.menuIds = await PermissionApi.getRoleMenuList(row.id)
    await nextTick()
    // 仅恢复叶子权限，父节点的半选状态由树组件推导，避免扩大权限范围。
    const selectedLeafIds = formData.menuIds.filter(
      (menuId) => treeRef.value?.getNode(menuId)?.isLeaf
    )
    treeRef.value?.setCheckedKeys(selectedLeafIds)
  } finally {
    formLoading.value = false
  }
}
defineExpose({ open }) // 提供 open 方法，用于打开弹窗

/** 提交表单 */
const emit = defineEmits(['success']) // 定义 success 事件，用于操作成功后的回调
const submitForm = async () => {
  const roleId = formData.id
  if (roleId === undefined) {
    message.error('角色信息尚未加载，请重新打开弹窗')
    return
  }
  // 校验表单
  if (!formRef.value || !treeRef.value) {
    return
  }
  const valid = await formRef.value.validate()
  if (!valid) {
    return
  }
  // 保存选中节点和半选中的父节点，过滤非数字树键。
  const menuIds = [
    ...treeRef.value.getCheckedKeys(false),
    ...treeRef.value.getHalfCheckedKeys()
  ].filter((key): key is number => typeof key === 'number')
  // 提交请求
  formLoading.value = true
  try {
    const data = {
      roleId,
      menuIds
    }
    await PermissionApi.assignRoleMenu(data)
    message.success(t('common.updateSuccess'))
    dialogVisible.value = false
    // 发送操作成功的事件
    emit('success')
  } finally {
    formLoading.value = false
  }
}

/** 重置表单 */
const resetForm = () => {
  // 重置选项
  treeNodeAll.value = false
  menuExpand.value = false
  // 重置表单
  Object.assign(formData, {
    id: undefined,
    name: '',
    code: '',
    menuIds: []
  })
  treeRef.value?.setCheckedNodes([])
  formRef.value?.resetFields()
}

/** 全选/全不选 */
const handleCheckedTreeNodeAll = () => {
  treeRef.value?.setCheckedNodes(treeNodeAll.value ? menuOptions.value : [])
}

/** 展开/折叠全部 */
const handleCheckedTreeExpand = () => {
  const nodes = treeRef.value?.store.nodesMap ?? {}
  for (let node in nodes) {
    if (nodes[node].expanded === menuExpand.value) {
      continue
    }
    nodes[node].expanded = menuExpand.value
  }
}
</script>
