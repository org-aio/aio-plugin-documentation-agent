<script setup lang="ts">
import { computed } from 'vue'
import { appConfig, settings } from '@/config'
import AppIcon from './AppIcon.vue'

defineProps<{ collapsed: boolean }>()
const title = computed(() => settings.title.trim() || appConfig.title)
</script>

<template>
  <RouterLink class="app-logo" to="/home" :title="title" :aria-label="title">
    <AppIcon class="app-logo-image" />
    <span v-if="!collapsed" class="app-logo-title">{{ title }}</span>
  </RouterLink>
</template>
