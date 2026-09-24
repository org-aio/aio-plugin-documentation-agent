<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { Picture } from '@element-plus/icons-vue'
import { appConfig, resolveAssetUrl, settings } from '@/config'

withDefaults(defineProps<{ alt?: string }>(), { alt: '' })

const attempt = ref(0)
const sources = computed(() => {
  const values = [settings.logo.trim(), appConfig.logo.trim(), '/logo.svg'].filter(Boolean)
  return [...new Set(values.map(resolveAssetUrl))]
})
const source = computed(() => sources.value[attempt.value])

watch(sources, () => {
  attempt.value = 0
})
</script>

<template>
  <img v-if="source" :src="source" :alt="alt" @error="attempt += 1" />
  <ElIcon v-else :aria-label="alt || undefined" :aria-hidden="!alt"><Picture /></ElIcon>
</template>
