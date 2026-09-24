<template>
  <Dialog v-model="dialogVisible" :title="dialogTitle">
    <el-form
      ref="formRef"
      v-loading="formLoading"
      :model="formData"
      :rules="formRules"
      label-width="80px"
    >
      <el-form-item label="字典名称" prop="name">
        <el-input v-model="formData.name" placeholder="请输入字典名称" />
      </el-form-item>
      <el-form-item label="字典类型" prop="type">
        <el-input
          v-model="formData.type"
          :disabled="typeof formData.id !== 'undefined'"
          placeholder="请输入参数名称"
        />
      </el-form-item>
      <el-form-item label="所属业务包" prop="ownerPackage">
        <el-input
          v-model="formData.ownerPackage"
          clearable
          placeholder="用于 jdbc2enum 过滤，多个用英文逗号分隔，例如 iot,ai"
        />
      </el-form-item>
      <el-form-item label="状态" prop="status">
        <el-radio-group v-model="formData.status">
          <el-radio
            v-for="dict in getIntDictOptions(DICT_TYPE.COMMON_STATUS)"
            :key="dict.value"
            :value="dict.value"
          >
            {{ dict.label }}
          </el-radio>
        </el-radio-group>
      </el-form-item>
      <el-form-item label="备注" prop="remark">
        <el-input v-model="formData.remark" placeholder="请输入内容" type="textarea" />
      </el-form-item>
    </el-form>
    <template #footer>
      <el-button :disabled="formLoading" type="primary" @click="submitForm">确 定</el-button>
      <el-button @click="dialogVisible = false">取 消</el-button>
    </template>
  </Dialog>
</template>
<script lang="ts" setup>
import { DICT_TYPE, getIntDictOptions } from '@/utils/dict'
import * as DictTypeApi from '@/api/system/dict/dict.type'
import { CommonStatusEnum } from '@/utils/constants'

defineOptions({ name: 'SystemDictTypeForm' })

const { t } = useI18n() // 国际化
const message = useMessage() // 消息弹窗

const dialogVisible = ref(false) // 弹窗的是否展示
const dialogTitle = ref('') // 弹窗的标题
const formLoading = ref(false) // 表单的加载中：1）修改时的数据加载；2）提交的按钮禁用
const formType = ref('') // 表单的类型：create - 新增；update - 修改
const ownerPackagePattern =
  /^\s*$|^\s*[A-Za-z_][A-Za-z0-9_.-]*(\s*,\s*[A-Za-z_][A-Za-z0-9_.-]*)*\s*$/
const formData = ref({
  id: undefined,
  name: '',
  type: '',
  ownerPackage: '',
  status: CommonStatusEnum.ENABLE,
  remark: ''
})
const validateOwnerPackage = (
  _rule: unknown,
  value: string | undefined,
  callback: (error?: Error) => void
) => {
  const text = value?.trim() ?? ''
  if (!text) {
    callback()
    return
  }
  if (text.length > 255) {
    callback(new Error('所属业务包长度不能超过255个字符'))
    return
  }
  if (!ownerPackagePattern.test(text)) {
    callback(new Error('所属业务包只能包含字母、数字、下划线、中划线、点号，多个用英文逗号分隔'))
    return
  }
  callback()
}
const formRules = reactive({
  name: [{ required: true, message: '字典名称不能为空', trigger: 'blur' }],
  type: [{ required: true, message: '字典类型不能为空', trigger: 'blur' }],
  ownerPackage: [{ validator: validateOwnerPackage, trigger: 'blur' }],
  status: [{ required: true, message: '状态不能为空', trigger: 'change' }]
})
const formRef = ref() // 表单 Ref

/** 打开弹窗 */
const open = async (type: string, id?: number) => {
  dialogVisible.value = true
  dialogTitle.value = t('action.' + type)
  formType.value = type
  resetForm()
  // 修改时，设置数据
  if (id) {
    formLoading.value = true
    try {
      const data = await DictTypeApi.getDictType(id)
      formData.value = {
        ...data,
        ownerPackage: data.ownerPackage ?? ''
      }
    } finally {
      formLoading.value = false
    }
  }
}
defineExpose({ open }) // 提供 open 方法，用于打开弹窗

/** 提交表单 */
const emit = defineEmits(['success']) // 定义 success 事件，用于操作成功后的回调
const submitForm = async () => {
  // 校验表单
  if (!formRef) return
  const valid = await formRef.value.validate()
  if (!valid) return
  // 提交请求
  formLoading.value = true
  try {
    const data: DictTypeApi.DictTypeSaveVO = {
      ...formData.value,
      ownerPackage: normalizeOwnerPackage(formData.value.ownerPackage)
    }
    formData.value.ownerPackage = data.ownerPackage ?? ''
    if (formType.value === 'create') {
      await DictTypeApi.createDictType(data)
      message.success(t('common.createSuccess'))
    } else {
      await DictTypeApi.updateDictType(data)
      message.success(t('common.updateSuccess'))
    }
    dialogVisible.value = false
    // 发送操作成功的事件
    emit('success')
  } finally {
    formLoading.value = false
  }
}

/** 归一化所属业务包 */
const normalizeOwnerPackage = (value?: string) => {
  return (
    value
      ?.split(',')
      .map((item) => item.trim())
      .filter((item) => item.length > 0)
      .join(',') ?? ''
  )
}

/** 重置表单 */
const resetForm = () => {
  formData.value = {
    id: undefined,
    type: '',
    name: '',
    ownerPackage: '',
    status: CommonStatusEnum.ENABLE,
    remark: ''
  }
  formRef.value?.resetFields()
}
</script>
