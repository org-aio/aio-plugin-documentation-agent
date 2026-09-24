<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { FolderOpened } from '@element-plus/icons-vue'
import { t } from '@/locales'
import {
  currentProjectId,
  loadProjects,
  projects,
  projectsLoading,
  setCurrentProject
} from '@/features/project/projectContext'

const selectedProjectId = computed({
  get: () => currentProjectId.value,
  set: (value: string) => setCurrentProject(value)
})

const projectOptions = computed(() =>
  projects.value.map((item) => ({
    value: String(item.id),
    label: String(item.projectName || item.name || item.id)
  }))
)

const currentProjectLabel = computed(
  () =>
    projectOptions.value.find((item) => item.value === currentProjectId.value)?.label ||
    t('project.select')
)

onMounted(() => {
  void loadProjects()
})
</script>

<template>
  <ElSelect
    v-model="selectedProjectId"
    class="project-switcher"
    filterable
    :loading="projectsLoading"
    :placeholder="t('project.select')"
    :aria-label="currentProjectLabel"
  >
    <template #prefix>
      <ElIcon><FolderOpened /></ElIcon>
    </template>
    <ElOption
      v-for="item in projectOptions"
      :key="item.value"
      :label="item.label"
      :value="item.value"
    />
  </ElSelect>
</template>

<style scoped>
.project-switcher {
  width: 220px;
}

@media (max-width: 767px) {
  .project-switcher {
    width: 150px;
  }
}
</style>
