<script setup lang="ts">
import { ElPagination } from 'element-plus'

const props = withDefaults(
  defineProps<{ total: number; page?: number; limit?: number; pagerCount?: number }>(),
  { page: 1, limit: 20, pagerCount: 5 }
)
const emit = defineEmits<{
  'update:page': [value: number]
  'update:limit': [value: number]
  pagination: [value: { page: number; limit: number }]
}>()

const changePage = (page: number) => {
  emit('update:page', page)
  emit('pagination', { page, limit: props.limit })
}

const changeSize = (limit: number) => {
  emit('update:limit', limit)
  emit('update:page', 1)
  emit('pagination', { page: 1, limit })
}
</script>

<template>
  <div class="page-pagination">
    <ElPagination
      :current-page="page"
      :page-size="limit"
      :page-sizes="[10, 20, 30, 50, 100]"
      :pager-count="pagerCount"
      :total="total"
      background
      layout="total, sizes, prev, pager, next"
      @update:current-page="changePage"
      @update:page-size="changeSize"
    />
  </div>
</template>

<style scoped>
.page-pagination {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
  overflow-x: auto;
}

@media (max-width: 640px) {
  .page-pagination {
    justify-content: flex-start;
  }
}
</style>
