<script setup lang="ts">
import { t } from '@/locales'
import { useExamples } from './store'

const { storageError } = useExamples()
</script>

<template>
  <div class="demo-notices">
    <el-alert :title="t('examples.demoNotice')" type="info" show-icon :closable="false" />
    <el-alert
      v-if="storageError"
      :title="t(storageError)"
      type="error"
      show-icon
      :closable="false"
      role="alert"
    />
  </div>
</template>

<style scoped>
.demo-notices {
  display: grid;
  gap: 10px;
  margin-bottom: 16px;
}
</style>
