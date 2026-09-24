// 模板管理为业务定制页面，放在手写生成目录以避免后续 Studio 前端生成覆盖。
import request from '@/config/axios'

export type EntityId = string | number
export type EntityInput = Record<string, any>
export type EntityRecord = EntityInput & { id: EntityId }

export interface PageResult<T> {
  list: T[]
  total: number
}

export const getBoxunTemplateManagementPage = (params: Record<string, unknown>) => request.get<PageResult<EntityRecord>>({ url: '/boxun/template-management/page', params })

export const getBoxunTemplateManagement = (id: EntityId) => request.get<EntityRecord>({ url: '/boxun/template-management/get?id=' + id })

export const getBoxunTemplateManagementSimpleList = (params?: Record<string, unknown>) => request.get<EntityRecord[]>({ url: '/boxun/template-management/simple-list', params })

export const listBoxunTemplateManagementByCondition = (data?: EntityInput) => request.post<EntityRecord[]>({ url: '/boxun/template-management/list-by-condition', data })

export const createBoxunTemplateManagement = (data: EntityInput) => request.post<EntityRecord>({ url: '/boxun/template-management/create', data })

export const upsertBoxunTemplateManagement = (data: EntityInput) => request.post<EntityRecord>({ url: '/boxun/template-management/upsert', data })

export const updateBoxunTemplateManagement = (data: EntityInput) => request.put<number>({ url: '/boxun/template-management/update', data })

export const deleteBoxunTemplateManagement = (id: EntityId) => request.delete<boolean>({ url: '/boxun/template-management/delete?id=' + id })

export const deleteBoxunTemplateManagementList = (ids: EntityId[]) => request.delete<boolean>({ url: '/boxun/template-management/delete-list', params: { ids: ids.join(',') } })

export interface TemplateFieldMapping {
  name: string
  fieldName: string
}

export interface TemplateFieldMappingResponse {
  beanDesDTOS: TemplateFieldMapping[]
  businessDescription: string
}

export const inheritBoxunTemplateManagement = (currentProjectId: string, anotherProjectId = '1733040597008125954') =>
  request.post<boolean>({ url: '/boxun/template-management/inherit-templates', params: { currentProjectId, anotherProjectId } })

export const getBoxunTemplateManagementFieldMapping = (id: EntityId) =>
  request.get<TemplateFieldMappingResponse>({ url: '/boxun/template-management/field-mapping?id=' + id })
