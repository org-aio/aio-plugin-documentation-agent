<script setup lang="ts">
import { t } from '@/locales'
import DemoNotice from '@/features/examples/DemoNotice.vue'
import RecordsTable from '@/features/examples/RecordsTable.vue'
</script>

<template>
  <div>
    <header class="page-header">
      <div>
        <h1>{{ t('examples.crud.title') }}</h1>
        <p class="page-description">{{ t('examples.crud.description') }}</p>
      </div>
    </header>
    <DemoNotice />
    <RecordsTable />
  </div>
</template>
