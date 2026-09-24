<script setup lang="ts">
import type { NavigationItem } from '@/features/navigation/catalog.mjs'
import { iconFor } from './menuIcons'

defineProps<{ items: NavigationItem[] }>()
</script>

<template>
  <template v-for="item in items" :key="item.path">
    <ElSubMenu v-if="item.children" :index="item.path" popper-class="app-menu-popper">
      <template #title>
        <ElIcon :size="16"><component :is="iconFor(item.icon)" /></ElIcon>
        <span class="app-menu-title">{{ item.title }}</span>
      </template>
      <NavigationMenu :items="item.children" />
    </ElSubMenu>
    <ElMenuItem v-else :index="item.path">
      <ElIcon :size="16"><component :is="iconFor(item.icon)" /></ElIcon>
      <template #title>
        <span class="app-menu-title">{{ item.title }}</span>
      </template>
    </ElMenuItem>
  </template>
</template>
