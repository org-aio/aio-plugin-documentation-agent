// 首页看板为业务定制页面，放在手写目录以避免后续 Studio 前端生成覆盖。
import request from '@/config/axios'

export type EntityId = string | number
export type EntityRecord = Record<string, any> & { id: EntityId }

/** 首页「近期需检测样品」按套件类型分组的结果。 */
export interface InspectionTodayResult {
  byList: EntityRecord[]
  ksList: EntityRecord[]
  ttjList: EntityRecord[]
  cmList: EntityRecord[]
  yclList: EntityRecord[]
  otherSampleList: EntityRecord[]
}

export interface EngineeringProgressItem {
  buildingNo?: string | null
  pouringPosition?: string | null
  productionDate?: string | null
  productionDateNum?: number | null
}

export interface EngineeringProgressGroup {
  x: string
  y: EngineeringProgressItem[]
}

export interface ProgressItem {
  buildingNo?: string | null
  pouringPosition?: string | null
  commissionDate?: string | null
  reportDate?: string | null
  sampleType?: number | null
}

/** 今日送检（原 theSamplesToBeSentForInspectionToday）。 */
export const getBoxunInspectionToday = (projectId: string) =>
  request.post<InspectionTodayResult>({ url: '/boxun/homepage/inspection-today', data: { projectId } })

/** 根据样品类型查询样品名称（原 querySampleNameBasedOnSampleType）。 */
export const getBoxunSampleNamesByType = (sampleType?: number) =>
  request.post<EntityRecord[]>({ url: '/boxun/homepage/sample-names-by-type', data: { sampleType } })

/** 根据进度类型查询相关样品名称（原 queryRelevantSampleNamesBasedOnProgressType）。 */
export const getBoxunSampleNamesByProgressType = (progressType: number) =>
  request.post<EntityRecord[]>({ url: '/boxun/homepage/sample-names-by-progress-type', data: { progressType } })

/** 各种进度（原 variousProgress），progressType 1 工程、2 委托、3 报告。 */
export const getBoxunVariousProgress = (projectId: string, progressType: number, sampleType?: number) =>
  request.post<EngineeringProgressGroup[] | ProgressItem[]>({
    url: '/boxun/homepage/various-progress',
    data: { projectId, progressType, sampleType }
  })

/** 查所有今天要送检的项目（原 getAllSjProject）。 */
export const getBoxunAllSjProject = () =>
  request.post<{ project: EntityRecord; inspectionToday: InspectionTodayResult }[]>({
    url: '/boxun/project-info/get-all-sj-project'
  })
