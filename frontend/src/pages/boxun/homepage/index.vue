<template>
  <div class="boxun-home">
    <ContentWrap>
      <div class="board-head">
        <div>
          <h2>近期需检测的样品</h2>
          <p>按当前项目展示今天前后三天的送检计划</p>
        </div>
        <ElSelect
          v-model="selectedProjectId"
          class="board-project"
          filterable
          placeholder="请选择项目"
          :loading="projectsLoading"
          @change="handleProjectChange"
        >
          <ElOption
            v-for="project in projects"
            :key="String(project.id)"
            :label="String(project.projectName || project.name || project.id)"
            :value="String(project.id)"
          />
        </ElSelect>
      </div>

      <div v-loading="inspectionLoading" class="sample-grid">
        <button
          v-for="card in sampleCards"
          :key="card.key"
          class="sample-card"
          type="button"
          @click="openSamples(card)"
        >
          <span class="sample-card-title">{{ card.title }}</span>
          <span class="sample-card-count">{{ card.items.length }}</span>
          <span class="sample-card-hint">条待送检</span>
        </button>
      </div>
    </ContentWrap>

    <ContentWrap>
      <div class="board-head">
        <div>
          <h2>进度统计</h2>
          <p>按工程、委托、报告维度查看当前项目进度</p>
        </div>
        <ElRadioGroup v-model="progressType" @change="loadProgress">
          <ElRadioButton :value="1">工程进度</ElRadioButton>
          <ElRadioButton :value="2">委托进度</ElRadioButton>
          <ElRadioButton :value="3">报告进度</ElRadioButton>
        </ElRadioGroup>
      </div>

      <ElSelect
        v-if="progressType !== 1 && sampleNames.length > 0"
        v-model="sampleType"
        class="board-sample-type"
        clearable
        placeholder="全部样品类型"
        @change="loadProgress"
      >
        <ElOption
          v-for="item in sampleNames"
          :key="String(item.id)"
          :label="String(item.materialName || item.id)"
          :value="Number(item.id)"
        />
      </ElSelect>

      <ElTable v-loading="progressLoading" :data="progressRows" class="board-table">
        <ElTableColumn prop="group" label="楼号" min-width="140" />
        <ElTableColumn prop="position" label="浇筑部位" min-width="160" />
        <ElTableColumn prop="date" label="日期" min-width="140" />
        <ElTableColumn prop="count" label="数量" width="100" />
      </ElTable>
      <ElEmpty v-if="!progressLoading && progressRows.length === 0" description="暂无进度数据" />
    </ContentWrap>

    <ElDialog v-model="samplesVisible" :title="samplesTitle" width="720px">
      <ElTable :data="samplesItems" max-height="420">
        <ElTableColumn prop="buildingNo" label="楼号" min-width="120" />
        <ElTableColumn prop="concatPosition" label="工程部位" min-width="160" />
        <ElTableColumn prop="strengthGrade" label="强度等级" min-width="120" />
        <ElTableColumn prop="productionDate" label="浇筑日期" min-width="140" />
      </ElTable>
      <ElEmpty v-if="samplesItems.length === 0" description="暂无样品" />
    </ElDialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { t } from '@/locales'
import * as BoxunHomePageApi from './BoxunHomePageApi'
import {
  currentProjectId,
  loadProjects,
  projects,
  projectsLoading,
  setCurrentProject
} from '@/features/project/projectContext'

defineOptions({ name: 'BoxunHomePage' })

type EntityRecord = BoxunHomePageApi.EntityRecord

const selectedProjectId = ref(currentProjectId.value)
const inspectionLoading = ref(false)
const progressLoading = ref(false)
const progressType = ref(1)
const sampleType = ref<number | undefined>(undefined)
const sampleNames = ref<EntityRecord[]>([])
const inspection = ref<BoxunHomePageApi.InspectionTodayResult>({
  byList: [],
  ksList: [],
  ttjList: [],
  cmList: [],
  yclList: [],
  otherSampleList: []
})

const sampleCards = computed(() => [
  { key: 'by', title: '标准养护', items: inspection.value.byList },
  { key: 'ks', title: '抗渗', items: inspection.value.ksList },
  { key: 'ttj', title: '同条件', items: inspection.value.ttjList },
  { key: 'cm', title: '拆模', items: inspection.value.cmList },
  { key: 'ycl', title: '原材料', items: inspection.value.yclList },
  { key: 'other', title: '其他样品', items: inspection.value.otherSampleList }
])

const progressRows = ref<{ group: string; position: string; date: string; count: number }[]>([])

const samplesVisible = ref(false)
const samplesTitle = ref('')
const samplesItems = ref<EntityRecord[]>([])

const loadInspection = async () => {
  if (!selectedProjectId.value) {
    inspection.value = { byList: [], ksList: [], ttjList: [], cmList: [], yclList: [], otherSampleList: [] }
    return
  }
  inspectionLoading.value = true
  try {
    inspection.value = await BoxunHomePageApi.getBoxunInspectionToday(selectedProjectId.value)
  } finally {
    inspectionLoading.value = false
  }
}

const loadSampleNames = async () => {
  if (progressType.value === 1) {
    sampleNames.value = []
    return
  }
  sampleNames.value = await BoxunHomePageApi.getBoxunSampleNamesByProgressType(progressType.value)
}

const loadProgress = async () => {
  await loadSampleNames()
  progressLoading.value = true
  try {
    const data = await BoxunHomePageApi.getBoxunVariousProgress(
      selectedProjectId.value,
      progressType.value,
      sampleType.value
    )
    if (progressType.value === 1) {
      progressRows.value = (data as BoxunHomePageApi.EngineeringProgressGroup[]).flatMap((group) =>
        group.y.map((item) => ({
          group: group.x || item.buildingNo || '-',
          position: item.pouringPosition || '-',
          date: item.productionDate || '-',
          count: 1
        }))
      )
    } else {
      progressRows.value = (data as BoxunHomePageApi.ProgressItem[]).map((item) => ({
        group: item.buildingNo || '-',
        position: item.pouringPosition || '-',
        date: item.commissionDate || item.reportDate || '-',
        count: 1
      }))
    }
  } finally {
    progressLoading.value = false
  }
}

const handleProjectChange = (value: string) => {
  setCurrentProject(value)
  void loadInspection()
  void loadProgress()
}

const openSamples = (card: { title: string; items: EntityRecord[] }) => {
  samplesTitle.value = `${card.title}（${card.items.length}）`
  samplesItems.value = card.items
  samplesVisible.value = true
}

onMounted(async () => {
  await loadProjects()
  selectedProjectId.value = currentProjectId.value
  await Promise.all([loadInspection(), loadProgress()])
})
</script>

<style scoped>
.boxun-home {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.board-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}
.board-head h2 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}
.board-head p {
  margin: 4px 0 0;
  color: var(--el-text-color-secondary);
  font-size: 13px;
}
.board-project {
  width: 240px;
}
.board-sample-type {
  width: 200px;
  margin-bottom: 12px;
}
.sample-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 12px;
  margin-top: 16px;
}
.sample-card {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 16px;
  border: 1px solid var(--panel-border-color);
  border-radius: 8px;
  background: var(--panel-bg-color);
  cursor: pointer;
  text-align: left;
  transition: border-color var(--transition-time-02);
}
.sample-card:hover {
  border-color: var(--el-color-primary);
}
.sample-card-title {
  color: var(--el-text-color-secondary);
  font-size: 13px;
}
.sample-card-count {
  font-size: 26px;
  font-weight: 600;
  color: var(--el-color-primary);
}
.sample-card-hint {
  color: var(--el-text-color-placeholder);
  font-size: 12px;
}
.board-table {
  margin-top: 8px;
}
</style>
