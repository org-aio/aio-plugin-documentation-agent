<!-- 试块评定：按 GB/T 50107-2010 对试件抗压强度代表值做合格判定。 -->
<template>
  <Dialog
    v-model="dialogVisible"
    :close-on-click-modal="false"
    title="试块评定"
    max-height="70vh"
    scroll
    width="min(720px, calc(100vw - 32px))"
  >
    <el-form ref="formRef" v-loading="loading" label-position="top">
      <el-row class="!mx-0" :gutter="16">
        <el-col :span="12">
          <el-form-item label="强度等级">
            <el-input v-model="formData.strengthGrade" placeholder="请输入强度等级，如 C30 或 C30P8" />
          </el-form-item>
        </el-col>
        <el-col :span="12">
          <el-form-item :label="`抗压强度代表值（MPa，共 ${formData.representativeValues.length} 组）`">
            <div class="evaluate-values">
              <div
                v-for="(value, index) in formData.representativeValues"
                :key="index"
                class="evaluate-value-row"
              >
                <span class="evaluate-value-label">第 {{ index + 1 }} 组</span>
                <el-input-number
                  v-model="formData.representativeValues[index]"
                  class="!w-100%"
                  :controls="false"
                  :min="0"
                  :precision="2"
                  placeholder="请输入代表值"
                />
                <el-button
                  class="evaluate-remove"
                  text
                  type="danger"
                  :disabled="formData.representativeValues.length <= 1"
                  @click="removeValue(index)"
                >
                  <Icon icon="ep:delete" />
                </el-button>
              </div>
              <el-button text type="primary" @click="addValue">
                <Icon class="mr-5px" icon="ep:plus" />添加一组
              </el-button>
            </div>
          </el-form-item>
        </el-col>
      </el-row>
      <el-form-item>
        <el-button type="primary" :loading="loading" @click="handleEvaluate">
          <Icon class="mr-5px" icon="ep:data-analysis" />开始评定
        </el-button>
      </el-form-item>
    </el-form>

    <el-alert
      v-if="result"
      class="mb-16px"
      :title="result.qualified ? '评定结论：合格' : '评定结论：不合格'"
      :type="result.qualified ? 'success' : 'error'"
      :description="`评定方法：${result.methodLabel}；样本容量：${result.sampleSize} 组`"
      :closable="false"
      show-icon
    />

    <el-descriptions v-if="result" :column="2" border>
      <el-descriptions-item label="强度等级标准值">
        {{ result.standardValue }} MPa
      </el-descriptions-item>
      <el-descriptions-item label="评定方法">
        {{ result.methodLabel }}
      </el-descriptions-item>
      <el-descriptions-item label="强度平均值">
        {{ result.mean.toFixed(2) }} MPa
      </el-descriptions-item>
      <el-descriptions-item label="强度最小值">
        {{ result.minimum.toFixed(2) }} MPa
      </el-descriptions-item>
      <el-descriptions-item v-if="result.standardDeviation != null" label="样本标准差">
        {{ result.standardDeviation.toFixed(2) }} MPa
      </el-descriptions-item>
      <el-descriptions-item v-if="result.lambda1 != null" label="合格评定系数 λ1 / λ2">
        {{ result.lambda1 }} / {{ result.lambda2 }}
      </el-descriptions-item>
      <el-descriptions-item v-if="result.lambda3 != null" label="合格评定系数 λ3 / λ4">
        {{ result.lambda3 }} / {{ result.lambda4 }}
      </el-descriptions-item>
      <el-descriptions-item label="平均值要求值">
        {{ result.meanRequirement.toFixed(2) }} MPa
        <el-tag class="ml-8px" :type="result.meanQualified ? 'success' : 'danger'" size="small">
          {{ result.meanQualified ? '满足' : '不满足' }}
        </el-tag>
      </el-descriptions-item>
      <el-descriptions-item label="最小值要求值">
        {{ result.minimumRequirement.toFixed(2) }} MPa
        <el-tag class="ml-8px" :type="result.minimumQualified ? 'success' : 'danger'" size="small">
          {{ result.minimumQualified ? '满足' : '不满足' }}
        </el-tag>
      </el-descriptions-item>
    </el-descriptions>

    <template #footer>
      <el-button @click="dialogVisible = false">关闭</el-button>
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import * as BoxunBlockRetentionLedgerApi from './BoxunBlockRetentionLedgerApi'

type EvaluationResult = BoxunBlockRetentionLedgerApi.SpecimenStrengthEvaluationResult

defineOptions({ name: 'BoxunBlockRetentionLedgerEvaluate' })

const message = useMessage()
const dialogVisible = ref(false)
const loading = ref(false)
const formRef = ref()
const result = ref<EvaluationResult>()
const formData = reactive<{
  strengthGrade: string
  representativeValues: (number | undefined)[]
}>({
  strengthGrade: '',
  representativeValues: [undefined, undefined, undefined]
})

const reset = (strengthGrade?: string) => {
  formData.strengthGrade = strengthGrade ?? ''
  formData.representativeValues = [undefined, undefined, undefined]
  result.value = undefined
}

const open = (strengthGrade?: string) => {
  reset(strengthGrade)
  dialogVisible.value = true
}
defineExpose({ open })

const addValue = () => {
  formData.representativeValues.push(undefined)
}

const removeValue = (index: number) => {
  if (formData.representativeValues.length <= 1) return
  formData.representativeValues.splice(index, 1)
}

const handleEvaluate = async () => {
  if (!formData.strengthGrade.trim()) {
    message.warning('请输入强度等级')
    return
  }
  const values = formData.representativeValues.filter(
    (value): value is number => typeof value === 'number' && Number.isFinite(value) && value > 0
  )
  if (values.length === 0) {
    message.warning('请至少输入一组有效的抗压强度代表值')
    return
  }
  loading.value = true
  try {
    result.value = await BoxunBlockRetentionLedgerApi.evaluateBoxunBlockRetentionLedgerStrength({
      strengthGrade: formData.strengthGrade.trim(),
      representativeValues: values
    })
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.evaluate-values {
  width: 100%;
}

.evaluate-value-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.evaluate-value-label {
  flex: 0 0 auto;
  min-width: 56px;
  color: var(--el-text-color-regular);
  font-size: 13px;
}

.evaluate-remove {
  flex: 0 0 auto;
}
</style>
