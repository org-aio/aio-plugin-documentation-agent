<script setup lang="ts">
import { t } from '@/locales'
</script>

<template>
  <section class="error-page">
    <span class="error-number">404</span>
    <h1>{{ t('error.title') }}</h1>
    <p>{{ t('error.description') }}</p>
    <RouterLink to="/home"
      ><ElButton type="primary">{{ t('error.backHome') }}</ElButton></RouterLink
    >
  </section>
</template>
