<script setup lang="ts">
import { ref } from 'vue'
import request from '@/config/axios'
import { t } from '@/locales'

const props = defineProps<{
  model: Record<string, unknown>
  route: 'commercial-concrete-ledger' | 'block-retention-ledger'
  disabled: boolean
}>()
const busy = defineModel<boolean>('busy', { default: false })
const error = ref('')

interface GroupNumbers {
  numberOfStandardCuringSpecimenGroups: number
  numberOfSetsOfImpermeableTestPieces: number
  numberOfSpecimensInTheSameCulture: number
  numberOfDemoldingSpecimenGroups: number
}

const calculate = async () => {
  if (busy.value || props.disabled) return
  error.value = ''
  const model = props.model
  const volume = model.sumVolume
  if (typeof volume !== 'number' || !Number.isFinite(volume) || volume < 0) {
    error.value = t('ledger.invalidVolume')
    return
  }
  const params = {
    pouringPosition: String(model.concatPosition ?? ''),
    strengthGrade: String(model.strengthGrade ?? ''),
    impermeabilityLevel: String(model.impermeabilityLevel ?? ''),
    volumeSum: volume
  }
  busy.value = true
  try {
    const counts = await request.get<GroupNumbers>({
      url: `/boxun/${props.route}/specimen-group-numbers`,
      params
    })
    // 计算期间可能切换表单或编辑输入；旧结果不能覆盖新台账。
    if (
      model === props.model &&
      volume === model.sumVolume &&
      params.pouringPosition === String(model.concatPosition ?? '') &&
      params.strengthGrade === String(model.strengthGrade ?? '') &&
      params.impermeabilityLevel === String(model.impermeabilityLevel ?? '')
    ) {
      Object.assign(model, counts)
    }
  } catch (failure) {
    error.value = failure instanceof Error ? failure.message : t('ledger.calculationFailed')
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <el-form-item>
    <el-button :disabled="disabled" :loading="busy" @click="calculate">
      <Icon class="mr-5px" icon="ep:cpu" />{{ t('ledger.calculateGroups') }}
    </el-button>
    <span v-if="error" class="calculation-error" role="alert">{{ error }}</span>
  </el-form-item>
</template>

<style scoped>
.calculation-error {
  margin-left: 12px;
  color: var(--el-color-danger);
}
</style>
