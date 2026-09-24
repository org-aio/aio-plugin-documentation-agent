<script setup lang="ts">
import { ref } from 'vue'
import { t } from '@/locales'
import CategoryTree from '@/features/examples/CategoryTree.vue'
import DemoNotice from '@/features/examples/DemoNotice.vue'
import RecordsTable from '@/features/examples/RecordsTable.vue'

const selectedCategoryId = ref<string | null>(null)
</script>

<template>
  <div>
    <header class="page-header">
      <div>
        <h1>{{ t('examples.treeTable.title') }}</h1>
        <p class="page-description">{{ t('examples.treeTable.description') }}</p>
      </div>
    </header>
    <DemoNotice />
    <div class="tree-table-layout">
      <CategoryTree v-model="selectedCategoryId" />
      <RecordsTable :category-id="selectedCategoryId" :show-category-filter="false" />
    </div>
  </div>
</template>

<style scoped>
.tree-table-layout {
  display: grid;
  grid-template-columns: 280px minmax(0, 1fr);
  align-items: start;
  gap: 16px;
}
@media (max-width: 1000px) {
  .tree-table-layout {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
