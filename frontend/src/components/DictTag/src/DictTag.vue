<script setup lang="ts">
import { computed } from 'vue'
import { ElTag } from 'element-plus'
import { getDictObj } from '@/utils/dict'

const props = defineProps<{ type: string; value?: unknown }>()
const option = computed(() => getDictObj(props.type, props.value))
</script>

<template>
  <ElTag v-if="option" :type="option.colorType || 'info'" :class="option.cssClass">
    {{ option.label }}
  </ElTag>
  <span v-else>{{ value ?? '-' }}</span>
</template>
