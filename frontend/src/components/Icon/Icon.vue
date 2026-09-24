<script setup lang="ts">
import { computed } from 'vue'
import { ElIcon } from 'element-plus'
import { resolveIcon } from './icons'

const props = withDefaults(
  defineProps<{ icon?: string; size?: number | string; color?: string }>(),
  { icon: '', size: 16, color: undefined }
)
const iconComponent = computed(() => resolveIcon(props.icon))
</script>

<template>
  <ElIcon :size="size" :color="color" aria-hidden="true">
    <component :is="iconComponent" />
  </ElIcon>
</template>
